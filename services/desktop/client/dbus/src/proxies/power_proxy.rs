use serde::{Deserialize, Serialize};
use tracing::info;
use zbus::{proxy, zvariant::Type, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Power",
    default_service = "org.mechanix.services.Power",
    default_path = "/org/mechanix/services/Power"
)]
trait PowerBusInterface {
    async fn session_logout(&self) -> Result<()>;
}

pub struct Power;

impl Power {
    pub async fn logout() -> Result<()> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.session_logout().await?;
        Ok(reply)
    }
}
