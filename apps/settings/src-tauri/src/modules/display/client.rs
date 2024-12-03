use serde::{Deserialize, Serialize};
use zbus::{proxy, zvariant::{ DeserializeDict, SerializeDict, Type}, Connection, Result};

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct NotificationEvent {
    pub brightness_percentage: u8,
}

#[proxy(
    interface = "org.mechanix.services.Display",
    default_service = "org.mechanix.services.Display",
    default_path = "/org/mechanix/services/Display"
)]
trait DisplayBusInterface {
    async fn get_brightness(&self) -> Result<u8>;
    async fn set_brightness(&self, value: u8) -> Result<()>;
    #[zbus(signal)]
    async fn notification(&self, event: NotificationEvent) -> Result<()>;
}

pub struct Display;

impl Display {
    pub async fn get_brightness_percentage() -> Result<u8> {
        println!("get_brightness_percentage...");

        let connection = Connection::system().await?;
        let proxy : DisplayBusInterfaceProxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_brightness().await?;
        println!("get_brightness_percentage reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn set_brightness_percentage(value: u8) -> Result<()> {
        println!("set_brightness_percentage value: {:?}", value);
        let connection = Connection::system().await?;
        let proxy : DisplayBusInterfaceProxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_brightness(value).await?;
        Ok(reply)
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let connection = Connection::system().await?;
        let proxy : DisplayBusInterfaceProxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let stream = proxy.receive_notification().await?;
        Ok(stream)
    }
}
