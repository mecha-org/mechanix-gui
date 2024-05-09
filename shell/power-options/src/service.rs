use crate::errors::{PowerOptionsError, PowerOptionsErrorCodes};
use anyhow::{bail, Result};
use mechanix_zbus_client::power::Power as PowerZbusClient;
use tracing::{debug, error, info};

pub struct PowerService {}

impl PowerService {
    pub async fn shutdown() -> Result<()> {
        println!("PowerService::shutdown()");
        match PowerZbusClient::shutdown().await {
            Ok(_) => true,
            Err(e) => bail!(PowerOptionsError::new(
                PowerOptionsErrorCodes::Shutdown,
                e.to_string(),
            )),
        };
        Ok(())
    }

    pub async fn restart() -> Result<()> {
        println!("PowerService::restart()");
        match PowerZbusClient::shutdown().await {
            Ok(_) => true,
            Err(e) => bail!(PowerOptionsError::new(
                PowerOptionsErrorCodes::Restart,
                e.to_string(),
            )),
        };
        Ok(())
    }
}
