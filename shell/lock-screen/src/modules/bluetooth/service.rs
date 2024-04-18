use crate::{
    errors::{LockScreenError, LockScreenErrorCodes},
    types::BluetoothStatus,
};
use anyhow::{bail, Result};
use mechanix_zbus_client::bluetooth::BluetoothService as BluetoothZbusClient;
use tracing::{debug, error, info};

pub struct BluetoothService {}

impl BluetoothService {
    pub async fn get_bluetooth_status() -> Result<BluetoothStatus> {
        let task = "get_bluetooth_status";

        let bluetooth_on = match BluetoothZbusClient::status().await {
            Ok(v) => v,
            Err(e) => bail!(LockScreenError::new(
                LockScreenErrorCodes::GetBluetoothStatusError,
                e.to_string(),
            )),
        };

        if bluetooth_on == 0 {
            return Ok(BluetoothStatus::Off);
        };

        let bluetooth_connected = match BluetoothZbusClient::is_connected().await {
            Ok(v) => v,
            Err(e) => bail!(LockScreenError::new(
                LockScreenErrorCodes::GetBluetoothStatusError,
                e.to_string(),
            )),
        };

        if bluetooth_connected == 1 {
            return Ok(BluetoothStatus::Connected);
        } else {
            return Ok(BluetoothStatus::On);
        };
    }
}
