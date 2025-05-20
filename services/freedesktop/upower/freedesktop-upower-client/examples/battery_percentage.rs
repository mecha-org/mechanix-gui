//! Basic example: Get Battery Percentage using mechanix_debus_client

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

    // Optionally, gracefully shut down the handler (here, we just wait for it)
    // In a real application, you may want to send a shutdown signal.

    Ok(())
}
