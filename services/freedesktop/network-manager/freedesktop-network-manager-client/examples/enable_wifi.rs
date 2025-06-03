//! Basic example: Enable Wifi using freedesktop-network-manager-client

use std::sync::mpsc;
use std::thread;
use freedesktop_network_manager_client::service::NetworkManagerService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending NetworkManager requests

    let network_manager = NetworkManagerService::new().await?;
    // Spawn the NetworkManager handler in a background task
    
    let receiver = match network_manager.toggle_wireless(true).await {
        Ok(()) => {
            println!("Wifi enabled");
            network_manager.subscribe_device_events().await
        }
        Err(e) => {
            println!("Error enabling wifi: {}", e);
            return Ok(());
        }
    };

    Ok(())
}
