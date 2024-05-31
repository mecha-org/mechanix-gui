use crate::errors::StatusBarError;
use crate::errors::StatusBarErrorCodes::{GetBatteryError, GetBatteryStatusError};
use anyhow::{bail, Result};
use upower::BatteryStatus;

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_level() -> Result<(u8, BatteryStatus)> {
        let task = "get_battery_level";

        let battery = match upower::get_battery().await {
            Ok(battery) => battery,
            Err(e) => bail!(StatusBarError::new(GetBatteryError, e.to_string(), false,)),
        };

        let percentage = match battery.percentage().await {
            Ok(p) => p,
            Err(e) => bail!(StatusBarError::new(
                GetBatteryStatusError,
                e.to_string(),
                false,
            )),
        };

        let state = match battery.state().await {
            Ok(s) => s,
            Err(e) => bail!(StatusBarError::new(
                GetBatteryStatusError,
                e.to_string(),
                false,
            )),
        };

        let battery_status = BatteryStatus::try_from(state).unwrap();

        Ok((percentage as u8, battery_status))
    }
}
