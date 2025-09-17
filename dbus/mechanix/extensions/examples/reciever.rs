use extensions::proxy::reciever::ExtensionServiceReceiver;
use extensions::events::ExtensionServiceEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create receiver and get event channel
    let (receiver, mut event_receiver) = ExtensionServiceReceiver::new().await?;
    
    // Start receiver in background
    tokio::spawn(async move {
        receiver.start_service().await.ok();
    });
    
    // Listen for signals
    while let Some(event) = event_receiver.recv().await {
        match event {
            ExtensionServiceEvent::Added(device) => {
                println!("Device added: {}", device.name());
            }
            ExtensionServiceEvent::Removed(device) => {
                println!("Device removed: {}", device.name());
            }
        }
    }
    
    Ok(())
}
