use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{sleep, Duration};
use std::path::PathBuf;
use crate::events::ExtensionServiceEvent;
use crate::device::{Device, types::{DeviceType, ConnectionType}};

const EXTENSION_SLAVE_ADDR: u16 = 0x16;
const I2C_BUS: &str = "/dev/i2c-2";
const PACKET_SIZE: usize = 8;

// I2C Detection packet format
const REQUEST_PACKET: [u8; 8] = [0xAA, 0x55, 0x01, 0xFF, 0x00, 0x69, 0x88, 0xFF];

/// Represents an I2C extension device
#[derive(Debug, Clone)]
pub struct I2CExtension {
    pub bus_path: String,
    pub slave_addr: u16,
    pub unique_id: String,
    pub name: String,
    pub connection_key: String,
}

impl I2CExtension {
    /// Create a new I2C extension with detected information
    pub fn new(bus_path: String, slave_addr: u16) -> Self {
        let unique_id = format!("i2c-{}-0x{:02x}", bus_path.replace("/dev/i2c-", ""), slave_addr);
        let name = format!("I2C Extension Device (0x{:02X})", slave_addr);
        let connection_key = "KEY_F1".to_string(); // Default connection key for I2C extensions
        
        Self {
            bus_path,
            slave_addr,
            unique_id,
            name,
            connection_key,
        }
    }

    /// Convert I2C extension to Device struct
    pub fn to_device(&self) -> Device {
        // Create a synthetic path for I2C devices
        let path = PathBuf::from(format!("/dev/i2c-extension-{}", self.unique_id));
        
        Device::new_custom(
            path,
            DeviceType::Extension,
            ConnectionType::Wired,
            self.connection_key.clone(),
            self.name.clone(),
            0x0000, // Default vendor ID for I2C devices
            self.unique_id.clone(),
        )
    }
}

/// Process I2C response data to validate communication
fn process_i2c_data(data: &[u8]) -> bool {
    // Validate packet format: first byte should be 0xAA, last byte should be 0xFF
    data.len() >= 2 && data[0] == 0xAA && data[data.len() - 1] == 0xFF
}

/// Attempt to connect and communicate with I2C extension device
async fn test_i2c_connection(bus_path: &str, slave_addr: u16) -> Result<bool, LinuxI2CError> {
    // Use spawn_blocking for I2C operations as they are blocking
    let bus_path = bus_path.to_string();
    let result = task::spawn_blocking(move || -> Result<bool, LinuxI2CError> {
        // Device Initialization
        let mut dev = LinuxI2CDevice::new(&bus_path, slave_addr)?;

        // Data Transmission
        dev.write(&REQUEST_PACKET)?;

        // Wait for response
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Read response
        let mut receive_buffer: [u8; PACKET_SIZE] = [0; PACKET_SIZE];
        dev.read(&mut receive_buffer)?;

        // Validate response
        Ok(process_i2c_data(&receive_buffer))
    }).await;

    match result {
        Ok(res) => res,
        Err(_) => Err(LinuxI2CError::Errno(1)),
    }
}

/// Watch for I2C extension devices with periodic polling
pub async fn watch_i2c_extensions(
    event_sender: mpsc::Sender<ExtensionServiceEvent>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting I2C extension detection...");
    
    let mut connected_devices: std::collections::HashMap<String, I2CExtension> = std::collections::HashMap::new();
    let poll_interval = Duration::from_secs(2); // Poll every 2 seconds

    loop {
        // Test the configured I2C bus and address
        match test_i2c_connection(I2C_BUS, EXTENSION_SLAVE_ADDR).await {
            Ok(true) => {
                // Valid I2C extension detected
                let extension = I2CExtension::new(I2C_BUS.to_string(), EXTENSION_SLAVE_ADDR);
                let device_id = extension.unique_id.clone();
                
                if !connected_devices.contains_key(&device_id) {
                    // New device detected
                    println!("I2C extension detected: {} at {}", extension.name, extension.bus_path);
                    
                    let device = extension.to_device();
                    connected_devices.insert(device_id.clone(), extension);
                    
                    if let Err(e) = event_sender.send(ExtensionServiceEvent::Added(device)).await {
                        eprintln!("Failed to send I2C Added event: {}", e);
                        break Ok(());
                    }
                }
            }
            Ok(false) => {
                // Device responded but with invalid data
                eprintln!("I2C device at {} responded with invalid data", I2C_BUS);
                
                // Remove device if it was previously connected
                let device_id = format!("i2c-{}-0x{:02x}", I2C_BUS.replace("/dev/i2c-", ""), EXTENSION_SLAVE_ADDR);
                if let Some(extension) = connected_devices.remove(&device_id) {
                    println!("Removing I2C extension due to invalid response: {}", extension.name);
                    let device = extension.to_device();
                    
                    if let Err(e) = event_sender.send(ExtensionServiceEvent::Removed(device)).await {
                        eprintln!("Failed to send I2C Removed event: {}", e);
                    }
                }
            }
            Err(_) => {
                // No device or communication error
                let device_id = format!("i2c-{}-0x{:02x}", I2C_BUS.replace("/dev/i2c-", ""), EXTENSION_SLAVE_ADDR);
                if let Some(extension) = connected_devices.remove(&device_id) {
                    println!("I2C extension disconnected: {}", extension.name);
                    let device = extension.to_device();
                    
                    if let Err(e) = event_sender.send(ExtensionServiceEvent::Removed(device)).await {
                        eprintln!("Failed to send I2C Removed event: {}", e);
                    }
                }
            }
        }

        // Wait before next poll
        sleep(poll_interval).await;
    }
}

/// Scan multiple I2C buses for extension devices
pub async fn scan_i2c_buses(
    buses: Vec<String>,
    addresses: Vec<u16>,
    event_sender: mpsc::Sender<ExtensionServiceEvent>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Scanning I2C buses: {:?} for addresses: {:?}", buses, addresses);
    
    let mut connected_devices: std::collections::HashMap<String, I2CExtension> = std::collections::HashMap::new();
    let poll_interval = Duration::from_secs(5); // Poll every 5 seconds for scanning mode

    loop {
        for bus in &buses {
            for &addr in &addresses {
                match test_i2c_connection(bus, addr).await {
                    Ok(true) => {
                        let extension = I2CExtension::new(bus.clone(), addr);
                        let device_id = extension.unique_id.clone();
                        
                        if !connected_devices.contains_key(&device_id) {
                            println!("I2C extension found: {} at {} (0x{:02X})", extension.name, bus, addr);
                            
                            let device = extension.to_device();
                            connected_devices.insert(device_id.clone(), extension);
                            
                            if let Err(e) = event_sender.send(ExtensionServiceEvent::Added(device)).await {
                                eprintln!("Failed to send I2C scan Added event: {}", e);
                            }
                        }
                    }
                    Ok(false) => {
                        // Device responded but invalid - remove if previously connected
                        let device_id = format!("i2c-{}-0x{:02x}", bus.replace("/dev/i2c-", ""), addr);
                        if let Some(extension) = connected_devices.remove(&device_id) {
                            let device = extension.to_device();
                            if let Err(e) = event_sender.send(ExtensionServiceEvent::Removed(device)).await {
                                eprintln!("Failed to send I2C scan Removed event: {}", e);
                            }
                        }
                    }
                    Err(_) => {
                        // No device - remove if previously connected
                        let device_id = format!("i2c-{}-0x{:02x}", bus.replace("/dev/i2c-", ""), addr);
                        if let Some(extension) = connected_devices.remove(&device_id) {
                            println!("I2C extension disconnected: {} from {} (0x{:02X})", extension.name, bus, addr);
                            let device = extension.to_device();
                            if let Err(e) = event_sender.send(ExtensionServiceEvent::Removed(device)).await {
                                eprintln!("Failed to send I2C scan Removed event: {}", e);
                            }
                        }
                    }
                }
            }
        }

        sleep(poll_interval).await;
    }
}
