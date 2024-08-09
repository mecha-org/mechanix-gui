use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.mechanix.services.Security",
    default_service = "org.mechanix.services.Security",
    default_path = "/org/mechanix/services/Security"
)]
trait SecurityBusInterface {
    async fn change_password(&self, old: String, secret: String, new: String) -> Result<bool>;
    async fn is_password_set(&self) -> Result<bool>;
    async fn authenticate_user(&self, password: String, secret: String) -> Result<bool>;
}

pub struct Security;

impl Security {
    pub fn is_pin_enabled() -> Result<bool> {
        let connection = zbus::blocking::Connection::system()?;
        let proxy: std::result::Result<SecurityBusInterfaceProxyBlocking, zbus::Error> = SecurityBusInterfaceProxyBlocking::new(&connection);
        let reply = proxy?.is_password_set()?;
        Ok(reply)
    }

    pub async fn authenticate(password: String, secret: String) -> Result<bool> {
        let connection = Connection::system().await?;
        let proxy = SecurityBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.authenticate_user(password, secret).await?;
        Ok(reply)
    }

    pub fn change_password(old: String, secret: String, new: String) -> Result<bool> {
        let connection = zbus::blocking::Connection::system()?;
        let proxy = SecurityBusInterfaceProxyBlocking::new(&connection)?;
        let reply = proxy.change_password(old, secret, new)?;
        Ok(reply)
    }

    pub fn remove_pin_lock(pin: String, secret: String) -> Result<bool> {
        // let connection = zbus::blocking::Connection::system()?;
        // let proxy: std::result::Result<SecurityBusInterfaceProxyBlocking, zbus::Error> = SecurityBusInterfaceProxyBlocking::new(&connection);
        // let reply = proxy?.remove_pin(pin, secret)?;
        // Ok(reply)
        Ok(true)
    }
}
