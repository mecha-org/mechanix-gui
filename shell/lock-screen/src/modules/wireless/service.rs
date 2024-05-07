use crate::errors::{LockScreenError, LockScreenErrorCodes};
use crate::modules::wireless::errors::{WirelessServiceError, WirelessServiceErrorCodes};
use crate::types::{WirelessConnectedState, WirelessStatus};
use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::wireless::{NotificationStream, WirelessService as WirelessZbusClient};
use tracing::{debug, error, info};

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<WirelessStatus> {
        let task = "get_wireless_status";
        let wireless_on = match WirelessZbusClient::wireless_status().await {
            Ok(v) => v,
            Err(e) => bail!(LockScreenError::new(
                LockScreenErrorCodes::GetWirelessStatusError,
                e.to_string(),
            )),
        };

        if !wireless_on {
            return Ok(WirelessStatus::Off);
        };

        let wireless_info = match WirelessZbusClient::info().await {
            Ok(v) => v,
            Err(e) => bail!(LockScreenError::new(
                LockScreenErrorCodes::GetWirelessStatusError,
                e.to_string(),
            )),
        };

        let signal = wireless_info.signal.parse::<i32>().unwrap();

        let wireless_status = if signal <= -80 {
            WirelessStatus::Connected(WirelessConnectedState::Low)
        } else if signal <= -60 {
            WirelessStatus::Connected(WirelessConnectedState::Weak)
        } else if signal <= -40 {
            WirelessStatus::Connected(WirelessConnectedState::Good)
        } else {
            WirelessStatus::Connected(WirelessConnectedState::Strong)
        };

        Ok(wireless_status)
    }

    pub async fn get_notification_stream() -> Result<NotificationStream<'static>> {
        let stream = WirelessZbusClient::get_notification_stream().await?;
        Ok(stream)
    }
}
