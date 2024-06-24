use policykit::types::Identity;
use tracing::info;
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
    async fn authenticate_polkit_request(
        &self,
        password: String,
        secret: String,
        cookie: String,
        identity: &Identity,
    ) -> Result<bool>;
}

pub struct Security;

impl Security {
    pub fn change_password(old: String, secret: String, new: String) -> Result<bool> {
        let connection = zbus::blocking::Connection::system()?;
        let proxy = SecurityBusInterfaceProxyBlocking::new(&connection)?;
        let reply = proxy.change_password(old, secret, new)?;
        Ok(reply)
    }
    pub fn is_password_set() -> Result<bool> {
        let connection = zbus::blocking::Connection::system()?;
        let proxy = SecurityBusInterfaceProxyBlocking::new(&connection)?;
        let reply = proxy.is_password_set()?;
        Ok(reply)
    }
    pub async fn authenticate(password: String, secret: String) -> Result<bool> {
        let connection = Connection::system().await?;
        let proxy = SecurityBusInterfaceProxy::new(&connection).await?;
        let reply = proxy.authenticate_user(password, secret).await?;
        Ok(reply)
    }
    pub fn authenticate_polkit(
        password: String,
        secret: String,
        cookie: String,
        identity: &Identity,
    ) -> Result<bool> {
        println!("authenticate_polkit()");
        let connection = zbus::blocking::Connection::system()?;
        println!("authenticate_polkit() connecttion created");
        let proxy = SecurityBusInterfaceProxyBlocking::new(&connection)?;
        println!("authenticate_polkit() proxy created");
        let reply = proxy.authenticate_polkit_request(password, secret, cookie, identity)?;
        println!("authenticate_polkit() authenticate bool reply {:?}", reply);
        Ok(reply)
    }
    pub async fn authenticate_polkit_2(
        password: String,
        secret: String,
        cookie: String,
        identity: &Identity,
    ) -> Result<bool> {
        let connection = Connection::system().await?;
        let proxy = SecurityBusInterfaceProxy::new(&connection).await?;

        let reply = proxy
            .authenticate_polkit_request(password, secret, cookie, identity)
            .await?;
        Ok(reply)
    }
}
