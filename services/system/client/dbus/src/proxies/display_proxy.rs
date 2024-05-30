use serde::{Deserialize, Serialize};
use tracing::info;
use zbus::{proxy, zvariant::Type, Connection, Result};

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
    // async fn get_screen_timeout(&self) -> Result<u32>;
    // async fn set_screen_timeout(&self, value: u32) ->Result<u32>;
    async fn set_backlight_on(&self) -> Result<()>;
    async fn set_backlight_off(&self) -> Result<()>;
    #[zbus(signal)]
    async fn notification(&self, event: NotificationEvent) -> Result<()>;
}

pub struct Display;

impl Display {
    pub async fn get_brightness_percentage() -> Result<u8> {
        let connection = Connection::system().await?;
        let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_brightness().await?;
        println!("get_brightness_percentage reply: {:?}", reply);
        Ok(reply)
    }

    pub async fn set_brightness_percentage(value: u8) -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_brightness(value).await?;
        Ok(())
    }

    pub async fn set_backlight_on() -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_backlight_on().await?;
        Ok(())
    }

    pub async fn set_backlight_off() -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_backlight_off().await?;
        Ok(())
    }

    // pub async fn get_screen_timeout() -> Result<String> {
    //     let connection = Connection::system().await?;
    //     let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
    //     let reply =  proxy.get_screen_timeout().await?;
    //     info!("get_screen_timeout reply: {:?}", reply);
    //     let result = format!("{}s", reply);
    //     Ok(result)
    // }

    // pub async fn set_screen_timeout(value: u32) -> Result<String> {
    //     let connection = Connection::system().await?;
    //     let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
    //     let reply = match proxy.set_screen_timeout(value).await {
    //         Ok(value)=> value,
    //         Err(e)=>{
    //             print!("error {:?}",e);
    //             0
    //         }
    //     };
    //     let result = format!("{}s", reply);
    //     Ok(result)
    // }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let connection = Connection::system().await?;
        let proxy = DisplayBusInterfaceProxy::new(&connection).await?;
        let stream = proxy.receive_notification().await?;
        Ok(stream)
    }
}
