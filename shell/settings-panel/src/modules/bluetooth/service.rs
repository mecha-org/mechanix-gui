use crate::modules::bluetooth::errors::{BluetoothServiceError, BluetoothServiceErrorCodes};
use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

use super::component::BluetoothStatus;

pub struct BluetoothService {}

impl BluetoothService {
    pub async fn get_bluetooth_status() -> Result<BluetoothStatus> {
        let task = "get_bluetooth_status";

        let bluetooth_state = vec![
            BluetoothStatus::NotFound,
            BluetoothStatus::Off,
            BluetoothStatus::On,
            BluetoothStatus::Connected,
        ];

        let current_state = *bluetooth_state
            .get((Local::now().second() % 4) as usize)
            .unwrap();

        Ok(current_state)
    }
}
