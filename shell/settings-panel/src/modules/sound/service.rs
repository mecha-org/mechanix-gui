use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_desktop_dbus_client::sound::Sound;
use tracing::{debug, error, info};

use crate::errors::{SettingsPanelError, SettingsPanelErrorCodes};

pub struct SoundService {}

impl SoundService {
    pub async fn get_sound_value(device: String) -> Result<u8> {
        let task = "get_sound_value";

        let sound = match Sound::get_sound_percentage(device).await {
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

        Ok(sound as u8)
    }

    pub async fn set_sound_value(value: u8, device: String) -> Result<()> {
        let task = "set_sound_value";
        println!(
            "SoundService::set_sound_value() {:?} converted value {:?}",
            value,
            (value as f32) as u8
        );
        match Sound::set_sound_percentage(value as f64, device).await {
            Ok(v) => v,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::SetSoundError,
                e.to_string(),
            )),
        };

        Ok(())
    }

    // pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
    //     let stream = Sound::get_notification_stream().await?;
    //     Ok(stream)
    // }
}
