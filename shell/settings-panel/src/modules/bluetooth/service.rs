use crate::{
    errors::{SettingsPanelError, SettingsPanelErrorCodes},
    modules::bluetooth::errors::{BluetoothServiceError, BluetoothServiceErrorCodes},
    types::BluetoothStatus,
};
use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use mechanix_zbus_client::bluetooth::BluetoothService as BluetoothZbusClient;
use tracing::{debug, error, info};

pub struct BluetoothService {}

impl BluetoothService {
    pub async fn get_bluetooth_status() -> Result<BluetoothStatus> {
        let task = "get_bluetooth_status";

        let bluetooth_on = match BluetoothZbusClient::status().await {
            Ok(v) => v,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::GetBluetoothStatusError,
                e.to_string(),
            )),
        };

        if bluetooth_on == 0 {
            return Ok(BluetoothStatus::Off);
        };

        let bluetooth_connected = match BluetoothZbusClient::is_connected().await {
            Ok(v) => v,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::GetBluetoothStatusError,
                e.to_string(),
            )),
        };

        if bluetooth_connected == 1 {
            return Ok(BluetoothStatus::Connected);
        } else {
            return Ok(BluetoothStatus::On);
        };
    }

    pub async fn enable_bluetooth() -> Result<()> {
        match BluetoothZbusClient::enable_bluetooth().await {
            Ok(_) => true,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::EnableBluetooth,
                e.to_string(),
            )),
        };
        Ok(())
    }

    pub async fn disable_bluetooth() -> Result<()> {
        match BluetoothZbusClient::disable_bluetooth().await {
            Ok(_) => true,
            Err(e) => bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::DisableBluetooth,
                e.to_string(),
            )),
        };
        Ok(())
    }
}
