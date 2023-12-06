use crate::BatteryState;
use anyhow::Result;

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_status() -> Result<BatteryState> {
        //add mctl libs code here

        Ok(BatteryState::Level60)
    }
}
