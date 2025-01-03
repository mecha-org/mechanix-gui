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
    async fn get_available_governors(&self) -> Result<Vec<String>>;
    async fn set_cpu_governor(&self, governor: String) -> Result<()>;
    async fn get_current_cpu_governor(&self) -> Result<String>;
}

pub struct Power;

impl Power {
    pub async fn logout() -> Result<()> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.session_logout().await?;
        Ok(reply)
    }

    pub async fn get_available_governors() -> Result<Vec<String>> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_available_governors().await?;
        Ok(reply)
    }

    pub async fn set_cpu_governor(governor: String) -> Result<()> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_cpu_governor(governor).await?;
        Ok(reply)
    }

    pub async fn get_current_cpu_governor() -> Result<String> {
        let connection = Connection::session().await?;
        let proxy = PowerBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.get_current_cpu_governor().await?;
        Ok(reply)
    }
}
