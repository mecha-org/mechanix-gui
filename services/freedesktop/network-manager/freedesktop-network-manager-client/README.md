# freedesktop-network-manager-client

A Rust library for interacting with NetworkManager via D-Bus on Linux. This crate provides an async, channel-based API
for enabling/disabling Wi-Fi, scanning for networks, subscribing to device state changes, and more.

## Features

- **Async API**: Built with Tokio for async/await support.
- **Wi-Fi Management**: Enable/disable wireless devices, scan for networks, and receive device state events.
- **Channel-based Requests**: Communicate with the NetworkManager handler using channels for flexible integration.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
freedesktop_network_manager_client = "latest_version"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

## Example

### Enable Wi-Fi

```rust
use freedesktop_network_manager_client::handler::{Client, NetworkManagerRequest};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending NetworkManager requests
    let (nm_tx, nm_rx) = mpsc::channel(10);

    // Spawn the NetworkManager handler in a background task
    let _handler = tokio::spawn(async move {
        let mut nm_handler = Client::new().await;
        let _ = nm_handler.run(nm_rx).await;
    });

    let (reply_to, mut receiver) = mpsc::channel(1);

    // Request to enable wireless device
    nm_tx
        .try_send(NetworkManagerRequest::EnableWirelessDevice { reply_to })
        .expect("Failed to send NetworkManager request");
    println!("NetworkManager enable request sent");

    // Wait for the response
    let handler = tokio::spawn(async move {
        while let Some(result) = receiver.recv().await {
            match result {
                Ok(()) => println!("Wireless device enabled"),
                Err(e) => eprintln!("Error enabling wireless device: {e}"),
            }
        }
    });

    handler.await?;
    Ok(())
```

## Error Handling

All operations return a `Result` with a custom `NetworkManagerError` enum, which can represent various error cases such
as D-Bus errors, proxy errors, or generic failures:

```rust
pub enum NetworkManagerError {
    Generic,
    ProxyError(ProxyError),
    CreateNmProxyError(String),
    GetListOfNetworksError(String),
    ConnectToNewNetworkError(String),
}
```