use crate::errors::{SettingsPanelError, SettingsPanelErrorCodes};
use crate::{
    modules::wireless::errors::{WirelessServiceError, WirelessServiceErrorCodes},
    types::{WirelessConnectedState, WirelessInfo, WirelessStatus},
};
use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::wireless::WirelessService as WirelessZbusClient;
use tracing::{debug, error, info};

pub struct WirelessService {}

impl WirelessService {
    pub async fn get_wireless_status() -> Result<WirelessStatus> {
        let task = "get_wireless_status";
        let wireless_on = match WirelessZbusClient::wireless_status().await {
            Ok(v) => v,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::GetWirelessStatusError,
                e.to_string(),
            )),
        };

        if !wireless_on {
            return Ok(WirelessStatus::Off);
        };

        let wireless_info = match WirelessZbusClient::info().await {
            Ok(v) => v,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::GetWirelessStatusError,
                e.to_string(),
            )),
        };

        let signal = wireless_info.signal.parse::<i32>().unwrap();
        let frequency = wireless_info.frequency;
        let ssid = wireless_info.name;
        let fmtd_wireless_info = WirelessInfo { ssid, frequency };

        let wireless_status = if signal <= -80 {
            WirelessStatus::Connected(WirelessConnectedState::Low, fmtd_wireless_info)
        } else if signal <= -60 {
            WirelessStatus::Connected(WirelessConnectedState::Weak, fmtd_wireless_info)
        } else if signal <= -40 {
            WirelessStatus::Connected(WirelessConnectedState::Good, fmtd_wireless_info)
        } else {
            WirelessStatus::Connected(WirelessConnectedState::Strong, fmtd_wireless_info)
        };

        Ok(wireless_status)
    }

    pub async fn enable_wireless() -> Result<bool> {
        let success = match WirelessZbusClient::enable_wireless().await {
            Ok(_) => true,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::EnableWireless,
                e.to_string(),
            )),
        };
        Ok(success)
    }

    pub async fn disable_wireless() -> Result<bool> {
        let success = match WirelessZbusClient::disable_wireless().await {
            Ok(_) => true,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::DisableWireless,
                e.to_string(),
            )),
        };
        Ok(success)
    }
}
