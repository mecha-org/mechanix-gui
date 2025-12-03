//! Basic example: Listen stream to wireless network power event.

use futures::StreamExt;
use networkmanager::service::NetworkManagerService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let network_manager = NetworkManagerService::new().await?;
    let mut receiver = network_manager.stream_wireless_enabled_status().await;

    // Spawn the NetworkManager handler in a background task
    let handler = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            println!("event: {:?}", result);
        }
    });

    // Wait for the handler to complete
    handler.await?;

    Ok(())
}