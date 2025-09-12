mod display;

use anyhow::Result;
use log::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let display = display::DisplayInterface::new();
    match display.watch_brightness().await {
        Ok(_) => {}
        Err(e) => {
            error!("watch_brightness() failed: {}", e);
        }
    }
    // Wait for SIGINT (Ctrl+C)
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            info!("Received SIGINT, shutting down");
        }
        Err(e) => error!("Failed to receive SIGINT: {}", e),
    }
    Ok(())
}
