use crate::{
    modules::bluetooth::errors::{BluetoothServiceError, BluetoothServiceErrorCodes},
    BluetoothState,
};
use anyhow::{bail, Result};
use mecha_bluetooth_ctl::BluetoothController;
use tracing::{debug, error, info};

pub struct BluetoothService {}

impl BluetoothService {
    pub async fn get_bluetooth_status() -> Result<BluetoothState> {
        let task = "get_bluetooth_status";

        let bluetooth_contoller = match BluetoothController::new().await {
            Ok(r) => r,
            Err(e) => {
                bail!(BluetoothServiceError::new(
                    BluetoothServiceErrorCodes::CreateBluetoothControllerError,
                    format!("error while creating bluetooth controller {}", e),
                    true
                ));
            }
        };

        let is_bluetooth_on = match bluetooth_contoller.bluetooth_status().await {
            Ok(v) => v,
            Err(e) => {
                bail!(BluetoothServiceError::new(
                    BluetoothServiceErrorCodes::GetBluetoothStatusError,
                    format!("error while getting bluetooth status {}", e),
                    true
                ));
            }
        };

        debug!(task, "bluetooth on is {}", is_bluetooth_on);

        if !is_bluetooth_on {
            return Ok(BluetoothState::Off);
        }

        Ok(BluetoothState::On)
    }
}
