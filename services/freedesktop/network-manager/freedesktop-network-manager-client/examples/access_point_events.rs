//! Basic example: Enable Wifi using freedesktop-network-manager-client

use freedesktop_network_manager_client::handler::{Client, NetworkManagerRequest};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending NetworkManager requests
    let (nm_tx, nm_rx) = mpsc::channel(10);

    // Spawn the NetworkManager handler in a background task
    let _handler = tokio::spawn(async move {
        let mut nm_handler = Client::new().await;
        // Run the handler event loop
        let _ = nm_handler.run(nm_rx).await;
    });

    let (reply_to, mut receiver) = mpsc::channel(5);
    // Example: Enable NetworkManager
    let request = NetworkManagerRequest::SubscribeAccessPointEvents { reply_to };
    nm_tx
        .try_send(request)
        .expect("Failed to send NetworkManager request");

    // Wait for the response
    let handler = tokio::spawn(async move {
        while let Some(result) = receiver.recv().await {
            match result {
                Ok(new_access_point) => println!("access_point event: {:?}", new_access_point),
                Err(e) => eprintln!("Error getting wifi state: {e}"),
            }
        }
    });

    _handler.await.unwrap();
    // Await the result and log it
    handler.await.unwrap();

    // (Optional) gracefully shut down or send more requests...

    // Wait for the handler to finish (in real code, you'd keep the handler running)
    // handler.await?;

    Ok(())
}
