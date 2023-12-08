use crate::{
    modules::battery::errors::{BatteryServiceError, BatteryServiceErrorCodes},
    BatteryState,
};

use anyhow::{bail, Result};
use mecha_battery_ctl::{Battery, PowerSupplyInfo};
use tracing::{debug, error, info};

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_status() -> Result<BatteryState> {
        let task = "get_battery_status";

        let battery = Battery::default();
        let battery_info = match battery.info() {
            Ok(v) => v,
            Err(e) => {
                bail!(BatteryServiceError::new(
                    BatteryServiceErrorCodes::GetBatteryInfoError,
                    format!("error while getting battery info {}", e),
                    true
                ));
            }
        };

        debug!(task, "battery info is {:?}", battery_info);

        let battery_state = match battery_info.capacity {
            0..=9 => BatteryState::Level0,
            10..=19 => BatteryState::Level10,
            20..=29 => BatteryState::Level20,
            30..=39 => BatteryState::Level30,
            40..=49 => BatteryState::Level40,
            50..=59 => BatteryState::Level50,
            60..=69 => BatteryState::Level60,
            70..=79 => BatteryState::Level70,
            80..=89 => BatteryState::Level80,
            90..=99 => BatteryState::Level90,
            100 => BatteryState::Level100,
            _ => BatteryState::Level100,
        };
        Ok(battery_state)
    }
}
