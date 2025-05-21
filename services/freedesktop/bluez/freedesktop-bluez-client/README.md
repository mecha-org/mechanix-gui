# freedesktop-bluez-client

A Rust library for interacting with Bluetooth via D-Bus and BlueZ on Linux. This crate provides an ergonomic, async API
for enabling/disabling Bluetooth, scanning for devices, connecting/disconnecting, and more.

## Features

- **Async API**: Built with Tokio for async/await support.
- **Bluetooth Management**: Power on/off, scan, connect, disconnect, and query devices.
- **Channel-based Requests**: Communicate with the Bluetooth handler using channels for flexible integration.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
freedesktop_bluez_client = "0.1.0" # Use the latest version
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

## Example

### Enable Bluetooth

```rust
use freedesktop_bluez_client::error::BluezError;
use freedesktop_bluez_client::handler::{BluezRequest, BluezClient};
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
let (bt_tx, bt_rx) = mpsc::channel(10);
// Spawn the Bluetooth handler
let _handler = tokio::spawn(async move {
    let mut bt_handler = BluezClient::new().await.unwrap();
    let _ = bt_handler.run(bt_rx).await;
});

let (res_tx, res_rx) = oneshot::channel();

// Request to power on Bluetooth
bt_tx.try_send(BluezRequest::SetPoweredOn { reply_to: res_tx })
    .expect("Failed to send Bluetooth request");

if let Ok(result) = res_rx.await {
    match result {
        Ok(()) => println!("Bluetooth powered on"),
        Err(e) => println!("Error: {:?}", e),
    }
}
Ok(())

```

## Error Handling

All operations return a `Result` with a custom `BluezError` enum, which can represent various error cases such as
D-Bus errors, BlueZ proxy errors, or generic failures.

## Contributing

Contributions are welcome! Please open issues or pull requests for bug fixes, improvements, or new features.

