use extensions::detection::service::start_extension_detection;
use extensions::events::ExtensionServiceEvent;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting I2C Extension Detection Example");
    
    // Create a channel for extension events
    let (tx, mut rx) = mpsc::channel::<ExtensionServiceEvent>(100);
    
    // Start the detection service in a background task
    let detection_handle = tokio::spawn(async move {
        if let Err(e) = start_extension_detection(tx).await {
            eprintln!("Detection service error: {}", e);
        }
    });
    
    // Listen for extension events
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                ExtensionServiceEvent::Added(device) => {
                    println!("🔌 Extension Added:");
                    println!("  Name: {}", device.name());
                    println!("  Type: {:?}", device.device_type());
                    println!("  Connection: {:?}", device.connection_type());
                    println!("  Unique ID: {}", device.unique_id());
                    println!("  Connection Key: {}", device.connection_key());
                    println!("  Path: {:?}", device.path());
                    println!();
                }
                ExtensionServiceEvent::Removed(device) => {
                    println!("🔌 Extension Removed:");
                    println!("  Name: {}", device.name());
                    println!("  Unique ID: {}", device.unique_id());
                    println!();
                }
            }
        }
    });
    
    println!("Extension detection running... Press Ctrl+C to stop");
    
    // Wait for the detection service
    detection_handle.await?;
    
    Ok(())
}
