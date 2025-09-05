mod display;
mod error;

use crate::display::DisplayInterface;
use crate::error::ServerError;
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use zbus::ConnectionBuilder;

pub const CONNECTION_BUS_NAME: &str = "org.mechanix.services.Display";
pub const DEFAULT_BRIGHTNESS_FILE_PATH: &str = "/etc/brightness";
pub const SERVED_AT: &str = "/org/mechanix/services/Display";

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    env_logger::init();
    let display_brightness_path =
        std::env::var("DISPLAY_BRIGHTNESS_FILE_PATH").unwrap_or(DEFAULT_BRIGHTNESS_FILE_PATH.to_string());
    debug!("display brightness path: {}", display_brightness_path);
    // Build the connection first on the system bus (requires DBus policy to be installed)
    let conn = match ConnectionBuilder::system() {
        Ok(builder) => match builder.name(CONNECTION_BUS_NAME) {
            Ok(named_builder) => match named_builder.build().await {
                Ok(conn) => conn,
                Err(e) => {
                    error!(
                        "Failed to acquire DBus name '{}' on system bus: {}",
                        CONNECTION_BUS_NAME, e
                    );
                    warn!("If the error is AccessDenied, ensure the DBus policy is installed and readable: /etc/dbus-1/system.d/org.mechanix.Display.conf or /usr/share/dbus-1/system.d/org.mechanix.Display.conf, then 'sudo systemctl reload dbus'.");
                    return Err(ServerError::FailedBuildConnection(e));
                }
            },
            Err(e) => {
                error!(
                    "Failed to set DBus name '{}' on system bus: {}",
                    CONNECTION_BUS_NAME, e
                );
                return Err(ServerError::FailedBuildConnection(e));
            }
        },
        Err(e) => {
            error!("Failed to create system bus connection builder: {}", e);
            return Err(ServerError::FailedBuildConnection(e));
        }
    };

    debug!("D-Bus connection built");
    debug!("D-Bus server registered at {}", SERVED_AT);
    let config_server = DisplayInterface {
        path: display_brightness_path,
    };
    if let Err(e) = conn.object_server().at(SERVED_AT, config_server).await {
        error!("Failed to start D-Bus server: {}", e);
        return Err(ServerError::FailedStartDBusServer(e));
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
