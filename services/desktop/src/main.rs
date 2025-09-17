mod handlers;

use crate::handlers::display::DisplayInterface;
use crate::handlers::hw_buttons::HwButtonHandler;
use anyhow::Result;
use log::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();
    let display = DisplayInterface::new();
    let display_handler = tokio::spawn(async move {
        display.watch_brightness().await;
    });
    handles.push(display_handler);
    let hw_button = HwButtonHandler::new();
    let hw_button_handler = tokio::spawn(async move {
        hw_button.run().await;
    });
    handles.push(hw_button_handler);
    // Wait for SIGINT (Ctrl+C)
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!("Received SIGINT, shutting down");
        }
        Err(e) => error!("Failed to receive SIGINT: {}", e),
    }
    Ok(())
}
