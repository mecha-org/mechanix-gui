mod display;
mod error;
mod interfaces;

use crate::display::DisplayInterface;
use crate::interfaces::hardware_buttons::{hw_buttons_notification_stream, HwButtonInterface};
use anyhow::Result;
use log::{debug, error, info};
use tokio::task::JoinHandle;
use zbus::connection;

pub const DISPLAY_CONNECTION_BUS_NAME: &str = "org.mechanix.services.Display";
pub const HW_BUTTONS_CONNECTION_BUS_NAME: &str = "org.mechanix.services.HwButton";

pub const BRIGHTNESS_PATH: &str = "/sys/class/backlight/32e60000.mipi_dsi.0/brightness";
pub const POWER_BUTTON_PATH: &str = "/dev/input/event0";
pub const HOME_BUTTON_PATH: &str = "/dev/input/event2";
pub const VOLUME_UP_BUTTON_PATH: &str = "/dev/input/event3";
pub const VOLUME_DOWN_BUTTON_PATH: &str = "/dev/input/event4";
pub const EXTENSION_PATH: &str = "/dev/input/event5";
pub const SERVED_AT: &str = "/org/mechanix/services/Display";

struct SystemPaths {
    display_brightness_path: String,
    power_button_path: String,
    home_button_path: String,
    volume_up_button_path: String,
    volume_down_button_path: String,
    extension_path: String
}

fn get_system_paths() -> SystemPaths {
    let display_brightness_path =
        std::env::var("BRIGHTNESS_PATH").unwrap_or(BRIGHTNESS_PATH.to_string());
    debug!("brightness path: {}", display_brightness_path);

    let power_button_path =
        std::env::var("POWER_BUTTON_PATH").unwrap_or(POWER_BUTTON_PATH.to_string());
    debug!("power button path: {}", power_button_path);

    let home_button_path =
        std::env::var("HOME_BUTTON_PATH").unwrap_or(HOME_BUTTON_PATH.to_string());
    debug!("home button path: {}", home_button_path);

    let volume_up_button_path =
        std::env::var("VOLUME_UP_BUTTON_PATH").unwrap_or(VOLUME_UP_BUTTON_PATH.to_string());
    debug!("volume up button path: {}", volume_up_button_path);

    let volume_down_button_path =
        std::env::var("VOLUME_DOWN_BUTTON_PATH").unwrap_or(VOLUME_DOWN_BUTTON_PATH.to_string());
    debug!("volume down button path: {}", volume_down_button_path);

    let extension_path =
        std::env::var("EXTENSION_PATH").unwrap_or(EXTENSION_PATH.to_string());
    debug!("extension path: {}", extension_path);


    SystemPaths {
        display_brightness_path,
        power_button_path,
        home_button_path,
        volume_up_button_path,
        volume_down_button_path,
        extension_path
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let paths = get_system_paths();

    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let display_config = DisplayInterface {
        path: paths.display_brightness_path,
    };

    let _display_bus_connection = connection::Builder::system()?
        .name(DISPLAY_CONNECTION_BUS_NAME)?
        .serve_at(SERVED_AT, display_config)?
        .build()
        .await?;

    let hw_button_bus = HwButtonInterface {};
    let _hw_button_bus_connection = connection::Builder::system()?
        .name(HW_BUTTONS_CONNECTION_BUS_NAME)?
        .serve_at("/org/mechanix/services/HwButton", hw_button_bus)?
        .build()
        .await?;

    let _hw_button_handle = tokio::spawn(async move {
        if let Err(e) = hw_buttons_notification_stream(
            &hw_button_bus,
            &_hw_button_bus_connection,
            String::from(paths.power_button_path),
            String::from(paths.home_button_path),
            String::from(paths.volume_up_button_path),
            String::from(paths.volume_down_button_path),
            String::from(paths.extension_path),
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
