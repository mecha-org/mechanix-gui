use extensions::detection::service::start_extension_detection_with_scan;
use extensions::events::ExtensionServiceEvent;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting I2C Extension Scanning Example");
    
    // Define I2C buses and addresses to scan
    let i2c_buses = vec![
        "/dev/i2c-0".to_string(),
        "/dev/i2c-1".to_string(),
        "/dev/i2c-2".to_string(),
    ];
    
    let i2c_addresses = vec![
        0x16, // Primary extension address
        0x17, // Secondary extension address
        0x18, // Tertiary extension address
    ];
    
    println!("Scanning I2C buses: {:?}", i2c_buses);
    println!("Scanning addresses: {:?}", i2c_addresses.iter().map(|&a| format!("0x{:02X}", a)).collect::<Vec<_>>());
    
    // Create a channel for extension events
    let (tx, mut rx) = mpsc::channel::<ExtensionServiceEvent>(100);
    
    // Start the detection service with scanning in a background task
    let detection_handle = tokio::spawn(async move {
        if let Err(e) = start_extension_detection_with_scan(tx, i2c_buses, i2c_addresses).await {
            eprintln!("Detection service error: {}", e);
        }
    });
    
    // Listen for extension events
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                ExtensionServiceEvent::Added(device) => {
                    println!("🔌 Extension Detected:");
                    println!("  Name: {}", device.name());
                    println!("  Type: {:?}", device.device_type());
                    println!("  Connection: {:?}", device.connection_type());
                    println!("  Unique ID: {}", device.unique_id());
                    println!("  Connection Key: {}", device.connection_key());
                    println!("  Path: {:?}", device.path());
                    
                    // Check if it's an I2C device
                    if device.unique_id().starts_with("i2c-") {
                        println!("  🔧 I2C Extension Details:");
                        if let Some(bus_info) = device.unique_id().strip_prefix("i2c-") {
                            println!("    Bus Info: {}", bus_info);
                        }
                    }
                    println!();
                }
                ExtensionServiceEvent::Removed(device) => {
                    println!("🔌 Extension Disconnected:");
                    println!("  Name: {}", device.name());
                    println!("  Unique ID: {}", device.unique_id());
                    println!();
                }
            }
        }
    });
    
    println!("Extension scanning running... Press Ctrl+C to stop");
    
    // Wait for the detection service
    detection_handle.await?;
    
    Ok(())
}
