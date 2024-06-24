use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Security",
    default_service = "org.mechanix.services.Security",
    default_path = "/org/mechanix/services/Security"
)]
trait SecurityBusInterface {
    async fn set_pin_lock(&self) -> Result<bool>;
    async fn remove_pin_lock(&self) -> Result<bool>;
    async fn is_pin_lock_enabled(&self) -> Result<bool>;
    async fn authenticate_pin(&self, pin: String) -> Result<bool>;
}

pub struct Security;

impl Security {
    pub async fn is_pin_enabled() -> Result<bool> {
        let connection = Connection::session().await?;
        let proxy = SecurityBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.is_pin_lock_enabled().await?;
        Ok(reply)
    }

    pub async fn authenticate(pin: String) -> Result<bool> {
        let connection = Connection::session().await?;
        let proxy = SecurityBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.authenticate_pin(pin).await?;
        Ok(reply)
    }

    pub async fn set_pin_lock() -> Result<bool> {
        let connection = Connection::session().await?;
        let proxy = SecurityBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.set_pin_lock().await?;
        Ok(reply)
    }

    pub async fn remove_pin_lock() -> Result<bool> {
        let connection = Connection::session().await?;
        let proxy = SecurityBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.remove_pin_lock().await?;
        Ok(reply)
    }
}
