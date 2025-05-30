//! Basic example: Enable Wifi using freedesktop-network-manager-client

use std::sync::mpsc;
use std::thread;
use freedesktop_network_manager_client::service::NetworkManagerService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending NetworkManager requests

    let network_manager = NetworkManagerService::new().await?;
    // Spawn the NetworkManager handler in a background task
    
    let receiver = network_manager.subscribe_device_events().await;
    // Wait for the response
    let handler = thread::spawn(move || {
        // Process messages until the channel closes
        while let Ok(result) = receiver.recv() {
            println!("event: {}", result);
        }
        println!("Device handler thread exiting gracefully.");
    });

    // Wait for the thread to finish and handle errors
    if let Err(e) = handler.join() {
        eprintln!("Handler thread panicked: {:?}", e);
    }

    // (Optional) gracefully shut down or send more requests...

    // Wait for the handler to finish (in real code, you'd keep the handler running)
    // handler.await?;

    Ok(())
}
