use evdevil::{ enumerate_hotplug, Evdev };
use tokio::sync::mpsc;
use tokio::task;
use evdev::EventSummary::Key;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::events::ExtensionServiceEvent;
use crate::device::Device;

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

async fn monitor_device_events(
    path: PathBuf,
    event_sender: mpsc::Sender<ExtensionServiceEvent>
) {
    let mut key_a_pressed = false;
    let mut current_device: Option<Device> = None;

    loop {
        // Use spawn_blocking for the blocking I/O operations
        let path_clone = path.clone();
        // let result = task::spawn_blocking(move || {
        //     evdev::Device::open(&path_clone)
        //         .and_then(|mut device| device.fetch_events())
        // }).await;
         let result = task::spawn_blocking(move || {
            evdev::Device::open(&path_clone)
                .and_then(|mut device| {
                    // Collect events into a Vec to own the data
                    device.fetch_events().map(|events| events.collect::<Vec<_>>())
                })
        }).await;

        match result {
            Ok(Ok(events)) => {
                for event in events {
                    let event_summary = event.destructure();
                    match event_summary {
                        evdev::EventSummary::Key(_, KEY_C, 1) => {
                            if !key_a_pressed {
                                key_a_pressed = true;
                                println!("Extension key 'A' pressed on device: {:?}", path);
                                
                                // Create device and send Added event
                                if let Ok(evdev) = Evdev::open(&path) {
                                    let device = Device::new(evdev);
                                    current_device = Some(device.clone());
                                    if let Err(e) = event_sender.send(
                                        ExtensionServiceEvent::Added(device)
                                    ).await {
                                        eprintln!("Failed to send Added event: {}", e);
                                        return;
                                    }
                                }
                            }
                        }
                        evdev::EventSummary::Key(_, KEY_C, 0) => {
                            if key_a_pressed {
                                key_a_pressed = false;
                                println!("Extension key 'A' released on device: {:?}", path);
                                
                                // Send Removed event with the same device
                                if let Some(device) = current_device.take() {
                                    if let Err(e) = event_sender.send(
                                        ExtensionServiceEvent::Removed(device)
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
                // Device error or disconnected, exit monitoring
                eprintln!("Device monitoring stopped for: {:?}", path);
                break;
            }
        }
    }
}
