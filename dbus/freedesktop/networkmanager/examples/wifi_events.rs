//! Basic example: Enable Wifi using freedesktop-network-manager-client

use networkmanager::service::NetworkManagerService;
use std::thread;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending NetworkManager requests

    let network_manager = NetworkManagerService::new().await?;
    // Spawn the NetworkManager handler in a background task
    // let _handler = tokio::spawn(async move {
    //     let mut nm_handler = Client::new().await;
    //     // Run the handler event loop
    //     let _ = nm_handler.run(nm_rx).await;
    // });

    let receiver = network_manager.stream_wireless_enabled_status().await;
    // Wait for the response
    let handler = thread::spawn(move || {
        // Process messages until the channel closes
        while let Ok(result) = receiver.recv() {
            println!("event: {:?}", result);
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
