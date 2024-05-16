use crate::{
    errors::{SettingsPanelError, SettingsPanelErrorCodes},
    modules::battery::errors::{BatteryServiceError, BatteryServiceErrorCodes},
    types::BatteryLevel,
};

use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_system_dbus_client::power::{NotificationStream, Power};
use tracing::{debug, error, info};

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_level() -> Result<(u8, String)> {
        let task = "get_battery_level";

        let capacity = match Power::get_battery_percentage().await {
            Ok(p) => p,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::DisableBluetooth,
                e.to_string(),
            )),
        };

        let status = match Power::get_battery_status().await {
            Ok(s) => s,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::DisableBluetooth,
                e.to_string(),
            )),
        };

        Ok((capacity as u8, status))
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let stream = Power::get_notification_stream().await?;
        Ok(stream)
    }
}
