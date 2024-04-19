use crate::{
    errors::GreeterError,
    modules::battery::errors::{BatteryServiceError, BatteryServiceErrorCodes},
};

use crate::errors::GreeterErrorCodes::GetBatteryStatusError;
use anyhow::{bail, Result};
use mechanix_zbus_client::power::Power;
use tracing::{debug, error, info};

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_level() -> Result<(u8, String)> {
        let task = "get_battery_level";

        let capacity = match Power::get_battery_percentage().await {
            Ok(p) => p,
            Err(e) => bail!(GreeterError::new(GetBatteryStatusError, e.to_string(),)),
        };

        let status = match Power::get_battery_status().await {
            Ok(s) => s,
            Err(e) => bail!(GreeterError::new(GetBatteryStatusError, e.to_string(),)),
        };

        Ok((capacity as u8, status))
    }
}
