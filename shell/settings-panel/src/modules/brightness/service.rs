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

        Ok((brightness as f32 / 254. * 100.) as u8)
    }

    pub async fn set_brightness_value(value: u8) -> Result<()> {
        let task = "set_brightness_value";
        println!(
            "BrightnessService::set_brightness_value() {:?} converted value {:?}",
            value,
            (value as f32 / 100. * 254.) as u8
        );
        match Display::set_brightness_percentage((value as f32 / 100. * 254.).max(5.) as u8).await {
            Ok(v) => v,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::SetBrightnessError,
                e.to_string(),
            )),
        };

        Ok(())
    }
}
