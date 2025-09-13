mod display;
mod error;
mod interfaces;

use crate::display::DisplayInterface;
use crate::interfaces::hardware_buttons::{hw_buttons_notification_stream, HwButtonInterface};
use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use tokio::task::JoinHandle;
use zbus::{connection, ConnectionBuilder};

pub const DISPLAY_CONNECTION_BUS_NAME: &str = "org.mechanix.services.Display";
pub const HW_BUTTONS_CONNECTION_BUS_NAME: &str = "org.mechanix.services.HwButton";

pub const BRIGHTNESS_PATH: &str = "/sys/class/backlight/backlight/brightness";
pub const POWER_BUTTON_PATH: &str = "/dev/input/event3";
pub const HOME_BUTTON_PATH: &str = "/dev/input/event3";
pub const SERVED_AT: &str = "/org/mechanix/services/Display";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let display_brightness_path = std::env::var("BRIGHTNESS_PATH")
        .unwrap_or(BRIGHTNESS_PATH.to_string());
    debug!("brightness path: {}", display_brightness_path);

    let power_button_path =
        std::env::var("POWER_BUTTON_PATH").unwrap_or(POWER_BUTTON_PATH.to_string());
    debug!("power button path: {}", power_button_path);

    let home_button_path =
        std::env::var("HOME_BUTTON_PATH").unwrap_or(HOME_BUTTON_PATH.to_string());
    debug!("home button path: {}", home_button_path);

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let display_config = DisplayInterface {
        path: display_brightness_path,
    };
    let _display_bus_connection = connection::Builder::system()?
        .name(DISPLAY_CONNECTION_BUS_NAME)?
        .serve_at(SERVED_AT, display_config)?
        .build()
        .await?;

    let hw_button_bus = HwButtonInterface {};
    let _hw_button_bus_connection = connection::Builder::system()?
        .name("org.mechanix.services.HwButton")?
        .serve_at("/org/mechanix/services/HwButton", hw_button_bus)?
        .build()
        .await?;

    let power_button_path = String::from(power_button_path);
    let home_button_path = String::from(home_button_path);
    let _hw_button_handle = tokio::spawn(async move {
        if let Err(e) = hw_buttons_notification_stream(
            &hw_button_bus,
            &_hw_button_bus_connection,
            power_button_path,
            home_button_path,
        )
        .await
        {
            error!("Error in power btn notification stream: {}", e);
        }
    });

    handles.push(_hw_button_handle);

    for handle in handles {
        handle.await?;
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
