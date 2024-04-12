use crate::{
    errors::StatusBarError,
    modules::battery::errors::{BatteryServiceError, BatteryServiceErrorCodes},
};

use crate::errors::StatusBarErrorCodes::GetBatteryStatusError;
use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::power::Power;
use tracing::{debug, error, info};

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_level() -> Result<(u8, String)> {
        let task = "get_battery_level";

        let capacity = match Power::get_battery_percentage().await {
            Ok(p) => p,
            Err(e) => bail!(StatusBarError::new(
                GetBatteryStatusError,
                e.to_string(),
                false,
            )),
        };

        let status = match Power::get_battery_status().await {
            Ok(s) => s,
            Err(e) => bail!(StatusBarError::new(
                GetBatteryStatusError,
                e.to_string(),
                false,
            )),
        };

        Ok((capacity as u8, status))
    }
}
