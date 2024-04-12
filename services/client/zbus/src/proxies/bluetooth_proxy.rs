use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Bluetooth",
    default_service = "org.mechanix.services.Bluetooth",
    default_path = "/org/mechanix/services/Bluetooth"
)]
trait Bluetooth {
    async fn status(&self) -> Result<i8>;
    async fn is_connected(&self) -> Result<i8>;
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
}
