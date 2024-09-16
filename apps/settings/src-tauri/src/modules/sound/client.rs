use serde::{Deserialize, Serialize};
use tracing::info;
use zbus::{
    proxy,
    zvariant::{DeserializeDict, SerializeDict, Type},
    Connection, Result
};
use std::{collections::HashMap, sync::Arc, thread, time::Duration};


#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct NotificationEvent {
    pub is_mute: bool,
    pub volume_level: f64,
}

#[derive(DeserializeDict, Serialize, Type, Debug)]
// `Type` treats `SinkInformationResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct SinkInformationResponse {
    pub name: String,
    pub description: String,
    pub prop_list: HashMap<String, String>,
}

#[derive(DeserializeDict, Serialize, Type, Debug)]
// `Type` treats `SourceInformationResponse` is an alias for `a{sv}`.
#[zvariant(signature = "a{sv}")]
pub struct SourceInformationResponse {
    pub name: String,
    pub description: String,
    pub prop_list: HashMap<String, String>,
}

#[proxy(
    interface = "org.mechanix.services.Sound",
    default_service = "org.mechanix.services.Sound",
    default_path = "/org/mechanix/services/Sound"
)]
trait SoundBusInterface {
    async fn get_output_device_volume(&self, device: String) -> Result<f64>;
    async fn set_output_device_volume(&self, volume: f64, device: String) -> Result<()>;
    async fn get_input_device_volume(&self, device: String) -> Result<f64>;
    async fn set_input_device_volume(&self, volume: f64, device: String) -> Result<()>;
    // #[zbus(signal)]
    // async fn notification(&self, event: NotificationEvent) -> Result<()>;
    async fn get_connected_input_devices(&self) -> Result<Vec<SinkInformationResponse>>;
    async fn get_connected_output_devices(&self) -> Result<Vec<SourceInformationResponse>>;
    async fn input_device_toggle_mute(&self, device: String) -> Result<()>;
    async fn output_device_toggle_mute(&self, device: String) -> Result<()>;
}

pub struct Sound;

impl Sound {
    pub async fn get_output_sound_percentage(device: String) -> Result<f64> {
        println!("Sound proxy get_output_sound_percentage()");
        let connection = Connection::session().await?;
        let proxy : SoundBusInterfaceProxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_output_device_volume(device).await?;
        println!("get_output_sound_percentage reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn set_output_sound_percentage(value: f64, device: String) -> Result<()> {
        println!("Sound proxy set_output_sound_percentage() {:?}", value);
        let connection = Connection::session().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_output_device_volume(value, device).await?;
        Ok(reply)
    }

    pub async fn get_input_sound_percentage(device: String) -> Result<f64> {
        println!("Sound proxy get_input_sound_percentage()");
        let connection = Connection::session().await?;
        let proxy : SoundBusInterfaceProxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_input_device_volume(device).await?;
        println!("get_input_sound_percentage reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn set_input_sound_percentage(value: f64, device: String) -> Result<()> {
        println!("Sound proxy set_input_sound_percentage() {:?}", value);
        let connection = Connection::session().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_input_device_volume(value, device).await?;
        Ok(reply)
    }

    pub async fn get_input_devices() -> Result<Vec<SinkInformationResponse>> {
        let connection = Connection::session().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_connected_input_devices().await?;
        Ok(reply)
    }

    pub async fn get_output_devices() -> Result<Vec<SourceInformationResponse>> {
        let connection = Connection::session().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_connected_output_devices().await?;
        Ok(reply)
    }

    pub async fn input_device_toggle_mute(device: String) -> Result<()> {
        println!("modules::sound::input_device_toggle_mute()");
        let connection = Connection::session().await?;
        let proxy : SoundBusInterfaceProxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.input_device_toggle_mute(device).await?;
        Ok(reply)
    } 

    pub async fn output_device_toggle_mute(device: String) -> Result<()> {
        println!("modules::sound::output_device_toggle_mute()");
        let connection = Connection::session().await?;
        let proxy : SoundBusInterfaceProxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.output_device_toggle_mute(device).await?;
        Ok(reply)
    } 
}
