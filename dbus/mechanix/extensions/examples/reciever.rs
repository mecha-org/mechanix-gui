use tokio;
use extensions::proxy::reciever::ExtensionServiceReceiver;
use extensions::events::ExtensionServiceEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ExtensionService receiver client...");

    // Create receiver and get the event channel
    let (receiver, mut event_receiver) = ExtensionServiceReceiver::new().await?;

    // Start the receiver service in a background task
    let receiver_task = tokio::spawn(async move {
        if let Err(e) = receiver.start_service().await {
            eprintln!("Receiver service failed: {}", e);
        }
    });

    // Handle events from the receiver
    let event_handler_task = tokio::spawn(async move {
        println!("Waiting for ExtensionServiceEvents...");
        
        while let Some(event) = event_receiver.recv().await {
            match event {
                ExtensionServiceEvent::Added(device) => {
                    println!("📦 ExtensionServiceEvent::Added received!");
                    println!("  Device: {}", device.name());
                    println!("  Path: {:?}", device.path());
                    println!("  Type: {:?}", device.device_type());
                    println!("  Connection: {:?}", device.connection_type());
                    println!("  Vendor ID: 0x{:04x}", device.vendor_id());
                    println!("  Unique ID: {}", device.unique_id());
                    println!("  ---");
                }
                ExtensionServiceEvent::Removed(device) => {
                    println!("📦 ExtensionServiceEvent::Removed received!");
                    println!("  Device: {}", device.name());
                    println!("  Path: {:?}", device.path());
                    println!("  ---");
                }
            }
        }
        
        println!("Event receiver stream ended");
    });

    println!("ExtensionService receiver is running...");
    println!("Press Ctrl+C to stop");

    // Wait for either task to complete
    tokio::select! {
        result = receiver_task => {
            match result {
                Ok(_) => println!("Receiver task completed"),
                Err(e) => eprintln!("Receiver task failed: {}", e),
            }
        }
        result = event_handler_task => {
            match result {
                Ok(_) => println!("Event handler task completed"),
                Err(e) => eprintln!("Event handler task failed: {}", e),
            }
        }
    }

    Ok(())
}
