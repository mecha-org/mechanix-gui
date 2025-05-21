# freedesktop-upower-client

A Rust library for interacting with [UPower](https://upower.freedesktop.org/) via D-Bus on Linux. This crate provides an
async, channel-based API for querying battery status, percentage, warning levels, and more.

## Features

- **Async API**: Built with Tokio for async/await support.
- **Battery Monitoring**: Query battery level, percentage, state, warning level, and power source type.
- **Channel-based Requests**: Communicate with the UPower handler using channels for flexible integration.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
freedesktop_upower_client = "latest_version"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
log = "0.4"
```

## Example

### Get Battery Percentage

```rust
use anyhow::Error;
use log::{error, info};
use tokio::sync::mpsc;
use freedesktop_upower_client::error::UpowerError;
use freedesktop_upower_client::handler::{UpowerHandler, UpowerRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a channel for sending Upower requests
    let (upower_tx, upower_rx) = mpsc::channel(10);
    // Spawn the Upower handler in a background task
    let _handler = tokio::spawn(async move {
        let mut upower_handler = match UpowerHandler::new().await {
            Ok(handler) => handler,
            Err(e) => {
                error!("Failed to create Upower handler: {e}");
                return;
            }
        };
        if let Err(e) = upower_handler.run(upower_rx).await {
            error!("Upower handler exited with error: {e}");
        }
    });

    // Create a channel to receive the battery percentage result
    let (reply_tx, mut reply_rx) = mpsc::channel::<Result<f64, UpowerError>>(1);

    // Send the GetPercentage request
    let request = UpowerRequest::GetPercentage { reply_to: reply_tx };
    if let Err(e) = upower_tx.try_send(request) {
        error!("Failed to send Upower request: {e}");
        return Err(e.into());
    }
    info!("Sent Upower get battery percentage request");

    // Await the result and log it
    if let Some(result) = reply_rx.recv().await {
        match result {
            Ok(percentage) => println!("Battery percentage: {percentage}"),
            Err(e) => error!("Error getting battery percentage: {e}"),
        }
    } else {
        error!("Did not receive a response for battery percentage");
    }

    Ok(())

```

## Error Handling

All operations return a `Result` with a custom `UpowerError` enum, which can represent various error cases such as D-Bus
errors, proxy errors, or generic failures.

## Contributing

Contributions are welcome! Please open issues or pull requests for bug fixes, improvements, or new features.

