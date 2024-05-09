use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::sound::{NotificationStream, Sound};
use tracing::{debug, error, info};

use crate::errors::{SettingsPanelError, SettingsPanelErrorCodes};

pub struct SoundService {}

impl SoundService {
    pub async fn get_sound_value() -> Result<u8> {
        let task = "get_sound_value";

        let sound = match Sound::get_sound_percentage().await {
            Ok(v) => {
                println!("SoundService::get_sound_value() {}", v);
                v
            }
            Err(e) => {
                println!("SoundService::get_sound_value() error {}", e.to_string());
                bail!(SettingsPanelError::new(
                    SettingsPanelErrorCodes::GetSoundError,
                    e.to_string(),
                ))
            }
        };

        Ok((sound as f32) as u8)
    }

    pub async fn set_sound_value(value: u8) -> Result<()> {
        let task = "set_sound_value";
        println!(
            "SoundService::set_sound_value() {:?} converted value {:?}",
            value,
            (value as f32) as u8
        );
        match Sound::set_sound_percentage((value as f32).max(5.) as u8).await {
            Ok(v) => v,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::SetSoundError,
                e.to_string(),
            )),
        };

        Ok(())
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let stream = Sound::get_notification_stream().await?;
        Ok(stream)
    }
}
