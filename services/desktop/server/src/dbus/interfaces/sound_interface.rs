use std::{collections::HashMap, sync::Arc, thread, time::Duration};

use mechanix_sound_ctl::Sound;
use tokio::{
    sync::{mpsc, Mutex},
    time,
};
use zbus::{
    fdo::Error as ZbusError,
    interface,
    zvariant::{DeserializeDict, SerializeDict, Type},
    Connection, SignalContext,
};

#[derive(Clone, Copy)]
pub struct SoundBusInterface {}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone)]
// `Type` treats `SourceInformationResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct SourceInformationResponse {
    pub name: String,
    pub description: String,
    pub prop_list: HashMap<String, String>,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone)]
// `Type` treats `SinkInformationResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct SinkInformationResponse {
    pub name: String,
    pub description: String,
    pub prop_list: HashMap<String, String>,
}

#[derive(DeserializeDict, SerializeDict, Type, Debug, Clone, PartialEq)]
// `Type` treats `NotificationEvent` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct SoundNotificationEvent {
    pub is_mute: bool,
    pub volume_level: f64,
}

#[interface(name = "org.mechanix.services.Sound")]
impl SoundBusInterface {
    pub async fn get_output_device_volume(
        &self,
        device: String,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<f64, ZbusError> {
        let sound = Sound::new();
        let (volume, is_muted) = match sound.get_output_device_volume(Some(device)).await {
            Ok((volume, is_muted)) => {
                let event = SoundNotificationEvent {
                    is_mute: is_muted,
                    volume_level: volume,
                };
                self.notification(&ctxt, event).await?;
                println!("Volume: {}, Muted: {}", volume, is_muted);
                (volume, is_muted)
            }
            Err(_) => return Err(ZbusError::Failed("Failed to get volume".to_string())),
        };
        Ok(volume)
    }

    #[zbus(signal)]
    async fn notification(
        &self,
        ctxt: &SignalContext<'_>,
        event: SoundNotificationEvent,
    ) -> Result<(), zbus::Error>;

    pub async fn set_output_device_volume(
        &self,
        volume: f64,
        device: String,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<(), ZbusError> {
        let sound = Sound::new();
        match sound.set_output_device_volume(volume, Some(device)).await {
            Ok(_) => {
                //trigger notification
                let event = SoundNotificationEvent {
                    is_mute: false,
                    volume_level: volume,
                };
                self.notification(&ctxt, event).await?;

                Ok(())
            }
            Err(_) => return Err(ZbusError::Failed("Failed to set volume".to_string())),
        }
    }

    pub async fn output_device_mute(
        &self,
        device: String,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<(), ZbusError> {
        let sound = Sound::new();

        match sound.output_device_mute(Some(device)).await {
            Ok(_) => {
                //trigger notification
                let event = SoundNotificationEvent {
                    is_mute: true,
                    volume_level: 0.00,
                };
                self.notification(&ctxt, event).await?;

                Ok(())
            }
            Err(_) => return Err(ZbusError::Failed("Failed to mute sound".to_string())),
        }
    }

    pub async fn output_device_unmute(
        &self,
        device: String,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
    ) -> Result<(), ZbusError> {
        let sound = Sound::new();
        match sound.output_device_unmute(Some(device.clone())).await {
            Ok(_) => {
                //trigger notification
                let event = SoundNotificationEvent {
                    is_mute: false,
                    volume_level: self
                        .get_output_device_volume(device.clone(), ctxt.clone())
                        .await?,
                };
                self.notification(&ctxt, event).await?;

                Ok(())
            }
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
}

// events for all signals to be emitted notification
pub async fn sound_event_notification_stream(
    sound_bus: &SoundBusInterface,
    conn: &zbus::Connection,
) -> Result<(), ZbusError> {
    let mut interval = time::interval(Duration::from_secs(1));
    let mut previous_volume: Option<f64> = None;
    let mut previous_mute_status: Option<bool> = None;

    loop {
        interval.tick().await;
        let sound_manager = Sound::new();
        let (current_volume, current_mute_status) =
            sound_manager.get_output_device_volume(None).await.unwrap();
        drop(sound_manager); // Release the lock

        let ctxt = SignalContext::new(conn, "/org/mechanix/services/Sound")?;

        // Check if the current volume or mute status has changed from the previous values
        if previous_volume != Some(current_volume)
            || previous_mute_status != Some(current_mute_status)
        {
            // If there's a change, emit the notification signal
            sound_bus
                .notification(
                    &ctxt,
                    SoundNotificationEvent {
                        is_mute: current_mute_status,
                        volume_level: current_volume,
                    },
                )
                .await?;

            // Update the previous values to the current ones
            previous_volume = Some(current_volume);
            previous_mute_status = Some(current_mute_status);
        }
    }
}
