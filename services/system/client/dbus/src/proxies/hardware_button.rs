use mechanix_system_dbus_server::KeyEvent;
use serde::{Deserialize, Serialize};
use tracing::info;
use zbus::{proxy, zvariant::Type, Connection, Result};

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct NotificationEvent {}

#[proxy(
    interface = "org.mechanix.services.HwButton",
    default_service = "org.mechanix.services.HwButton"
)]
trait HwButtonInterface {
    #[zbus(signal)]
    async fn notification(&self, event: KeyEvent) -> Result<(), zbus::Error>;
}

pub struct HwButton;

impl HwButton {
    pub async fn get_notification_stream(path: String) -> Result<NotificationStream<'static>> {
        let connection = Connection::system().await?;
        let proxy = HwButtonInterfaceProxy::new(&connection, path).await?;
        let stream = proxy.receive_notification().await?;
        Ok(stream)
    }
}
