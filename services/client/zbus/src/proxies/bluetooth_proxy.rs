use mechanix_zbus_services::BluetoothNotificationEvent;
use serde::{Deserialize, Serialize};
use zbus::{proxy, zvariant::Type, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Bluetooth",
    default_service = "org.mechanix.services.Bluetooth",
    default_path = "/org/mechanix/services/Bluetooth"
)]
trait Bluetooth {
    async fn status(&self) -> Result<i8>;
    async fn is_connected(&self) -> Result<i8>;
    async fn enable(&self) -> Result<()>;
    async fn disable(&self) -> Result<()>;
    #[zbus(signal)]
    async fn notification(&self, event: BluetoothNotificationEvent) -> Result<()>;
}

pub struct BluetoothService;

impl BluetoothService {
    pub async fn is_connected() -> Result<i8> {
        let connection = Connection::system().await?;
        let proxy = BluetoothProxy::new(&connection).await?;
        let reply = proxy.is_connected().await?;
        Ok(reply)
    }

    pub async fn status() -> Result<i8> {
        let connection = Connection::system().await?;
        let proxy = BluetoothProxy::new(&connection).await?;
        let reply = proxy.status().await?;
        Ok(reply)
    }

    pub async fn enable_bluetooth() -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = BluetoothProxy::new(&connection).await?;
        let reply = proxy.enable().await?;
        Ok(reply)
    }

    pub async fn disable_bluetooth() -> Result<()> {
        let connection = Connection::system().await?;
        let proxy = BluetoothProxy::new(&connection).await?;
        let reply = proxy.disable().await?;
        Ok(reply)
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let connection = Connection::system().await?;
        let proxy = BluetoothProxy::new(&connection).await?;
        let stream = proxy.receive_notification().await?;
        Ok(stream)
    }
}
