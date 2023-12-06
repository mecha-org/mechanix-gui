use crate::BluetoothState;
use anyhow::Result;

pub struct BluetoothService {}

impl BluetoothService {
    pub async fn get_bluetooth_status() -> Result<BluetoothState> {
        //add mctl libs code here

        Ok(BluetoothState::Connected)
    }
}
