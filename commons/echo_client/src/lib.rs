use anyhow::{bail, Result};
use zbus::Connection;

pub struct EchoClient {}
impl EchoClient {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn echo(
        destination: &str,
        path: &str,
        interface: &str,
        method_name: &str,
    ) -> Result<bool> {
        let connection = match Connection::session().await {
            Ok(c) => c,
            Err(e) => {
                bail!("Error creating connection {}", e);
            }
        };

        let res = match connection
            .call_method(
                Some(String::from(destination)),
                path,
                Some(String::from(interface)),
                method_name,
                &(),
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
}
