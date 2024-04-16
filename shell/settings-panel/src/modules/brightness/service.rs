use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::display::Display;
use tracing::{debug, error, info};

use crate::errors::{SettingsPanelError, SettingsPanelErrorCodes};

pub struct BrightnessService {}

impl BrightnessService {
    pub async fn get_brightness_value() -> Result<u8> {
        let task = "get_brightness_value";

        let brightness = match Display::get_brightness_percentage().await {
            Ok(v) => {
                println!("BrightnessService::get_brightness_value() {}", v);
                v
            }
            Err(e) => {
                println!(
                    "BrightnessService::get_brightness_value() error {}",
                    e.to_string()
                );
                bail!(SettingsPanelError::new(
                    SettingsPanelErrorCodes::GetBrightnessError,
                    e.to_string(),
                ))
            }
        };

        Ok(brightness / 255 * 100)
    }

    pub async fn set_brightness_value(value: u8) -> Result<()> {
        let task = "set_brightness_value";
        match Display::set_brightness_percentage(value / 100 * 255).await {
            Ok(v) => v,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::SetBrightnessError,
                e.to_string(),
            )),
        };

        Ok(())
    }
}
