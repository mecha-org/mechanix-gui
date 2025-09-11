use evdevil::{ enumerate_hotplug, Evdev };
use tokio::sync::mpsc;
use tokio::task;
use evdev::KeyCode;
use evdev::EventSummary::Key;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::events::ExtensionServiceEvent;
use crate::device::Device;
use crate::detection::i2c;
use std::str::FromStr;

pub async fn watch_hotplug(
    event_sender: mpsc::Sender<ExtensionServiceEvent>
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a channel to communicate between blocking thread and async context
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Spawn a blocking task that handles the iterator
    task::spawn_blocking(move || {
        match enumerate_hotplug() {
            Ok(mut hotplug_iter) => {
                loop {
                    match hotplug_iter.next() {
                        Some(Ok(device)) => {
                            if tx.send(Ok(device)).is_err() {
                                break; // Receiver dropped
                            }
                        }
                        Some(Err(e)) => {
                            let _ = tx.send(Err(e));
                            break;
                        }
                        None => {
                            break;
                        } // Iterator ended
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Err(e));
            }
        }
    });

    println!("Starting hotplug watch...");

    // Keep track of devices and their monitoring tasks
    let mut device_tasks: HashMap<PathBuf, tokio::task::JoinHandle<()>> = HashMap::new();

    // Process events in async context
    while let Some(result) = rx.recv().await {
        match result {
            Ok(evdev_device) => {
                println!(
                    "Device connected: {}",
                    evdev_device.name().unwrap_or("Unknown".to_string())
                );
                let path = evdev_device.path().to_owned();

                // Cancel existing task for this device if any
                if let Some(old_task) = device_tasks.remove(&path) {
                    old_task.abort();
                }

                // Start continuous monitoring for this device
                let event_sender_clone = event_sender.clone();
                let path_clone = path.clone();
                let task_handle = task::spawn(async move {
                    monitor_device_events(path_clone, event_sender_clone).await;
                });

                device_tasks.insert(path, task_handle);
            }
            Err(e) => {
                eprintln!("Detection error: {}", e);
                break;
            }
        }
    }

    // Cancel all remaining tasks
    for (_, task) in device_tasks {
        task.abort();
    }

    Ok(())
}

async fn monitor_device_events(path: PathBuf, event_sender: mpsc::Sender<ExtensionServiceEvent>) {
    let mut current_device: Option<Device> = None;
    let mut connection_key: Option<KeyCode> = None;

    // Initialize device and check if it's an extension
    if let Ok(evdev) = Evdev::open(&path) {
        let device = Device::new(evdev);
        let (is_ext, key_str) = device.is_extension();
        
        if is_ext {
            // Parse the connection key
            match KeyCode::from_str(key_str) {
                Ok(key_code) => {
                    connection_key = Some(key_code);
                    current_device = Some(device.clone());
                    println!("Monitoring extension device with key: {} at {:?}", key_str, path);
                }
                Err(e) => {
                    eprintln!("Failed to parse connection key '{}': {}", key_str, e);
                    return; // Exit if we can't parse the key
                }
            }
        } else {
            // Not an extension device, don't monitor
            println!("Device at {:?} is not an extension, skipping monitoring", path);
            return;
        }
    } else {
        eprintln!("Failed to open device at {:?}", path);
        return;
    }

    loop {
        // Use spawn_blocking for the blocking I/O operations
        let path_clone = path.clone();
        let result = task::spawn_blocking(move || {
            evdev::Device::open(&path_clone).and_then(|mut device| {
                // Collect events into a Vec to own the data
                device.fetch_events().map(|events| events.collect::<Vec<_>>())
            })
        }).await;

        match result {
            Ok(Ok(events)) => {
                for event in events {
                    let event_summary = event.destructure();
                    match event_summary {
                        evdev::EventSummary::Key(_, key, 1) => {
                            // Check if this is the correct connection key for this extension
                            if let (Some(expected_key), Some(device)) = (&connection_key, &current_device) {
                                if key == *expected_key {
                                    println!("Extension connection key pressed on device: {:?}", path);
                                    
                                    if let Err(e) = event_sender.send(
                                        ExtensionServiceEvent::Added(device.clone())
                                    ).await {
                                        eprintln!("Failed to send Added event: {}", e);
                                        return;
                                    }
                                }
                            }
                        }
                        evdev::EventSummary::Key(_, key, 0) => {
                            // Check if this is the correct connection key for this extension
                            if let (Some(expected_key), Some(device)) = (&connection_key, &current_device) {
                                if key == *expected_key {
                                    println!("Extension connection key released on device: {:?}", path);
                                    
                                    if let Err(e) = event_sender.send(
                                        ExtensionServiceEvent::Removed(device.clone())
                                    ).await {
                                        eprintln!("Failed to send Removed event: {}", e);
                                        return;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Ok(Err(_)) | Err(_) => {
                // Device error or disconnected
                eprintln!("Device monitoring stopped for: {:?}", path);
                
                // Send removal event for the device if we have it
                if let Some(device) = current_device.take() {
                    println!("Sending removal event for disconnected device: {:?}", path);
                    if let Err(e) = event_sender.send(
                        ExtensionServiceEvent::Removed(device)
                    ).await {
                        eprintln!("Failed to send Removed event for disconnected device: {}", e);
                    }
                }
                break;
            }
        }
    }
}

/// Start both hotplug (evdev) and I2C extension detection services concurrently
pub async fn start_extension_detection(
    event_sender: mpsc::Sender<ExtensionServiceEvent>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting extension detection services...");
    
    // Clone the sender for each detection service
    let hotplug_sender = event_sender.clone();
    let i2c_sender = event_sender.clone();
    
    // Start hotplug detection task
    let hotplug_task = task::spawn(async move {
        if let Err(e) = watch_hotplug(hotplug_sender).await {
            eprintln!("Hotplug detection error: {}", e);
        }
    });
    
    // Start I2C detection task
    let i2c_task = task::spawn(async move {
        if let Err(e) = i2c::watch_i2c_extensions(i2c_sender).await {
            eprintln!("I2C detection error: {}", e);
        }
    });
    
    // Wait for either task to complete (both should run indefinitely)
    tokio::select! {
        _ = hotplug_task => {
            println!("Hotplug detection task ended");
        }
        _ = i2c_task => {
            println!("I2C detection task ended");
        }
    }
    
    Ok(())
}

/// Start extension detection with custom I2C buses and addresses for scanning
pub async fn start_extension_detection_with_scan(
    event_sender: mpsc::Sender<ExtensionServiceEvent>,
    i2c_buses: Vec<String>,
    i2c_addresses: Vec<u16>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting extension detection services with I2C scanning...");
    
    // Clone the sender for each detection service
    let hotplug_sender = event_sender.clone();
    let i2c_sender = event_sender.clone();
    
    // Start hotplug detection task
    let hotplug_task = task::spawn(async move {
        if let Err(e) = watch_hotplug(hotplug_sender).await {
            eprintln!("Hotplug detection error: {}", e);
        }
    });
    
    // Start I2C scanning task
    let i2c_task = task::spawn(async move {
        if let Err(e) = i2c::scan_i2c_buses(i2c_buses, i2c_addresses, i2c_sender).await {
            eprintln!("I2C scanning error: {}", e);
        }
    });
    
    // Wait for either task to complete (both should run indefinitely)
    tokio::select! {
        _ = hotplug_task => {
            println!("Hotplug detection task ended");
        }
        _ = i2c_task => {
            println!("I2C scanning task ended");
        }
    }
    
    Ok(())
}
