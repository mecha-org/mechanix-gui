use std::collections::HashMap;

use mechanix_sound_ctl::Sound;
use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
};

pub struct SoundBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type, Debug)]
// `Type` treats `SourceInformationResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct SourceInformationResponse {
    pub name: String,
    pub description: String,
    pub prop_list: HashMap<String, String>,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug)]
// `Type` treats `SinkInformationResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct SinkInformationResponse {
    pub name: String,
    pub description: String,
}

#[interface(name = "org.mechanix.services.Sound")]
impl SoundBusInterface {
    pub async fn get_output_device_volume(&self, device: String) -> Result<f64, ZbusError> {
        let sound = Sound::new();
        let volume = match sound.get_output_device_volume(Some(device)).await {
            Ok(volume) => {
                println!("Volume: {:?}", volume);
                volume
            }
            Err(_) => return Err(ZbusError::Failed("Failed to get volume".to_string())),
        };
        Ok(volume)
    }

    pub async fn set_output_device_volume(
        &self,
        volume: f64,
        device: String,
    ) -> Result<(), ZbusError> {
        let sound = Sound::new();
        match sound.set_output_device_volume(volume, Some(device)).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(ZbusError::Failed("Failed to set volume".to_string())),
        }
    }

    pub async fn output_device_mute(&self, device: String) -> Result<(), ZbusError> {
        let sound = Sound::new();

        match sound.output_device_mute(Some(device)).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(ZbusError::Failed("Failed to mute sound".to_string())),
        }
    }

    pub async fn output_device_unmute(&self, device: String) -> Result<(), ZbusError> {
        let sound = Sound::new();
        match sound.output_device_unmute(Some(device)).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(ZbusError::Failed("Failed to unmute sound".to_string())),
        }
    }

    pub async fn output_device_toggle_mute(&self, device: String) -> Result<(), ZbusError> {
        let sound = Sound::new();

        match sound.output_device_toggle_mute(Some(device)).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(ZbusError::Failed("Failed to toggle mute".to_string())),
        }
    }

    pub async fn get_connected_output_devices(
        &self,
    ) -> Result<Vec<SourceInformationResponse>, ZbusError> {
        let sound = Sound::new();
        match sound.get_connected_output_device_list().await {
            Ok(_) => Ok(sound
                .get_connected_output_device_list()
                .await
                .unwrap()
                .iter()
                .map(|device| SourceInformationResponse {
                    name: device.name.clone(),
                    description: device.description.clone(),
                    prop_list: device
                        .prop_list
                        .iter()
                        .map(|(key, value)| (key.clone(), value.clone()))
                        .collect(),
                })
                .collect()),
            Err(_) => {
                return Err(ZbusError::Failed(
                    "Failed to get connected devices".to_string(),
                ))
            }
        }
    }

    pub async fn get_input_device_volume(&self, device: String) -> Result<f64, ZbusError> {
        let sound = Sound::new();

        let volume = match sound.get_input_device_volume(Some(device)).await {
            Ok(volume) => volume,
            Err(_) => return Err(ZbusError::Failed("Failed to get volume".to_string())),
        };
        Ok(volume)
    }

    pub async fn set_input_device_volume(
        &self,
        volume: f64,
        device: String,
    ) -> Result<(), ZbusError> {
        let sound = Sound::new();

        match sound.set_input_device_volume(volume, Some(device)).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(ZbusError::Failed("Failed to set volume".to_string())),
        }
    }

    pub async fn input_device_mute(&self, device: String) -> Result<(), ZbusError> {
        let sound = Sound::new();

        match sound.input_device_mute(Some(device)).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(ZbusError::Failed("Failed to mute sound".to_string())),
        }
    }

    pub async fn input_device_unmute(&self, device: String) -> Result<(), ZbusError> {
        let sound = Sound::new();

        match sound.input_device_unmute(Some(device)).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(ZbusError::Failed("Failed to unmute sound".to_string())),
        }
    }

    pub async fn input_device_toggle_mute(&self, device: String) -> Result<(), ZbusError> {
        let sound = Sound::new();

        match sound.input_device_toggle_mute(Some(device)).await {
            Ok(_) => Ok(()),
            Err(_) => return Err(ZbusError::Failed("Failed to toggle mute".to_string())),
        }
    }

    pub async fn get_connected_input_devices(
        &self,
    ) -> Result<Vec<SinkInformationResponse>, ZbusError> {
        let sound = Sound::new();
        match sound.get_connected_input_device_list().await {
            Ok(_) => Ok(sound
                .get_connected_input_device_list()
                .await
                .unwrap()
                .iter()
                .map(|device| SinkInformationResponse {
                    name: device.name.clone(),
                    description: device.description.clone(),
                })
                .collect()),
            Err(_) => {
                return Err(ZbusError::Failed(
                    "Failed to get connected devices".to_string(),
                ))
            }
        }
    }
}
