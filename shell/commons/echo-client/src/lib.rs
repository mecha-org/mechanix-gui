use anyhow::{bail, Result};
use zbus::{Connection, Proxy};
use zvariant::OwnedValue;

pub struct EchoClient {}
impl EchoClient {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn echo<T>(
        destination: &str,
        path: &str,
        interface: &str,
        method_name: &str,
        params: T,
    ) -> Result<bool>
    where
        T: serde::ser::Serialize + zvariant::DynamicType,
    {
        let connection = match Connection::session().await {
            Ok(c) => c,
            Err(e) => {
                bail!("Error creating connection {}", e);
            }
        };

        match connection
            .call_method(
                Some(String::from(destination)),
                path,
                Some(String::from(interface)),
                method_name,
                &params,
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                bail!("Error while calling connection method {}", e);
            }
        };

        Ok(true)
    }

    pub async fn echo_property<T>(
        destination: &str,
        path: &str,
        interface: &str,
        property: &str,
    ) -> Result<T>
    where
        T: TryFrom<OwnedValue>,
        T::Error: Into<zbus::Error>,
    {
        let connection = match Connection::session().await {
            Ok(c) => c,
            Err(e) => {
                bail!("Error creating connection {}", e);
            }
        };

        let proxy = match Proxy::new(&connection, destination, path, interface).await {
            Ok(p) => p,
            Err(e) => {
                bail!("Error creating proxy {}", e);
            }
        };

        let x: T = match proxy.get_property(property).await {
            Ok(r) => r,
            Err(e) => {
                bail!("Error while calling connection method {}", e);
            }
        };

        Ok(x)
    }
}
