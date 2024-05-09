use serde::{Deserialize, Serialize};
use tracing::info;
use zbus::{proxy, zvariant::Type, Connection, Result};

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct NotificationEvent {}

#[proxy(
    interface = "org.mechanix.services.Sound",
    default_service = "org.mechanix.services.Sound",
    default_path = "/org/mechanix/services/Sound"
)]
trait SoundBusInterface {
    async fn get_sound(&self) -> Result<u8>;
    async fn set_sound(&self, value: u8) -> Result<()>;
    #[zbus(signal)]
    async fn notification(&self, event: NotificationEvent) -> Result<()>;
}

pub struct Sound;

impl Sound {
    pub async fn get_sound_percentage() -> Result<u8> {
        let connection = Connection::system().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_sound().await?;
        println!("get_sound_percentage reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn set_sound_percentage(value: u8) -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_sound(value).await?;
        Ok(())
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let connection = Connection::system().await?;
        let proxy = SoundBusInterfaceProxy::new(&connection).await?;
        let stream = proxy.receive_notification().await?;
        Ok(stream)
    }
}
