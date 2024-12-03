use mechanix_desktop_dbus_server::SoundNotificationEvent;
use serde::{Deserialize, Serialize};
use tracing::info;
use zbus::{proxy, zvariant::Type, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Sound",
    default_service = "org.mechanix.services.Sound",
    default_path = "/org/mechanix/services/Sound"
)]
trait SoundBusInterface {
    async fn set_output_device_volume(&self, volume: f64, device: String) -> Result<()>;
    async fn get_output_device_volume(&self, device: String) -> Result<f64>;
    #[zbus(signal)]
    async fn notification(&self, event: SoundNotificationEvent) -> Result<()>;
}

pub struct Sound;

impl Sound {
    pub async fn get_sound_percentage(device: String) -> Result<f64> {
        println!("Sound proxy get_sound_percentage()");
        let connection = Connection::session().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_output_device_volume(device).await?;
        println!("get_sound_percentage reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn set_sound_percentage(value: f64, device: String) -> Result<()> {
        println!("Sound proxy set_sound_percentage() {:?}", value);
        let connection = Connection::session().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_output_device_volume(value, device).await?;
        Ok(reply)
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let connection = Connection::session().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let stream = proxy.receive_notification().await?;
        Ok(stream)
    }
}
