use crate::errors::{StatusBarError, StatusBarErrorCodes};
use crate::modules::wireless::errors::{WirelessServiceError, WirelessServiceErrorCodes};
use crate::types::WirelessConnectedState;
use crate::WirelessStatus;
use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::wireless::WirelessService as WirelessZbusClient;
use tracing::{debug, error, info};

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<WirelessStatus> {
        let task = "get_wireless_status";
        let wireless_on = match WirelessZbusClient::wifi_status().await {
            Ok(v) => v,
            Err(e) => bail!(StatusBarError::new(
                StatusBarErrorCodes::GetWirelessStatusError,
                e.to_string(),
                false,
            )),
        };

        if !wireless_on {
            return Ok(WirelessStatus::Off);
        };

        let wireless_info = match WirelessZbusClient::info().await {
            Ok(v) => v,
            Err(e) => bail!(StatusBarError::new(
                StatusBarErrorCodes::GetWirelessStatusError,
                e.to_string(),
                false,
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
}
