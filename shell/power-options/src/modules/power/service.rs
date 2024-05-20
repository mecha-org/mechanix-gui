use crate::errors::{PowerOptionsError, PowerOptionsErrorCodes};
use anyhow::{bail, Result};
use mechanix_desktop_dbus_client::power::Power as PowerDesktopZbusClient;
use mechanix_system_dbus_client::power::Power as PowerZbusClient;
use tracing::{debug, error, info};

pub struct PowerService {}

impl PowerService {
    pub async fn power_off() -> Result<()> {
        println!("PowerService::power_off()");
        match PowerZbusClient::power_off().await {
            Ok(_) => true,
            Err(e) => bail!(PowerOptionsError::new(
                PowerOptionsErrorCodes::Shutdown,
                e.to_string(),
            )),
        };
        Ok(())
    }

    pub async fn reboot() -> Result<()> {
        println!("PowerService::reboot()");
        match PowerZbusClient::reboot().await {
            Ok(_) => true,
            Err(e) => bail!(PowerOptionsError::new(
                PowerOptionsErrorCodes::Restart,
                e.to_string(),
            )),
        };
        Ok(())
    }

    pub async fn logout() -> Result<()> {
        println!("PowerService::logout()");
        match PowerDesktopZbusClient::logout().await {
            Ok(_) => true,
            Err(e) => bail!(PowerOptionsError::new(
                PowerOptionsErrorCodes::Logout,
                e.to_string(),
            )),
        };
        Ok(())
    }
}
