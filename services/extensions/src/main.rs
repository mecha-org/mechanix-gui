use tokio;
use extensions::interface::ExtensionService;
use anyhow::Result;
use log::{error, info, warn};
use std::process;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with better error handling
    if let Err(e) = env_logger::try_init() {
        eprintln!("Failed to initialize logger: {}", e);
        process::exit(1);
    }

    info!("Starting Extension Service...");

    tokio::select! {
        result = ExtensionService::start_service() => {
            match result {
                Ok(_) => {
                    info!("Extension service completed successfully");
                }
                Err(e) => {
                    error!("Extension service failed: {}", e);
                    process::exit(1);
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            warn!("Received shutdown signal, stopping extension service...");
            info!("Extension service stopped gracefully");
            process::exit(0);
        }
    }

    Ok(())
}
