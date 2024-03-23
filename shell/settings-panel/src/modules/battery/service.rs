use crate::modules::battery::errors::{BatteryServiceError, BatteryServiceErrorCodes};

use anyhow::{bail, Result};
use chrono::{Local, Timelike};
use tracing::{debug, error, info};

use super::component::BatteryLevel;

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_status() -> Result<BatteryLevel> {
        let task = "get_battery_status";

        let battery_percentages = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        let capacity = battery_percentages[(Local::now().second() % 11) as usize];

        let battery_state = match capacity {
            0..=9 => BatteryLevel::Level0,
            10..=19 => BatteryLevel::Level10,
            20..=29 => BatteryLevel::Level20,
            30..=39 => BatteryLevel::Level30,
            40..=49 => BatteryLevel::Level40,
            50..=59 => BatteryLevel::Level50,
            60..=69 => BatteryLevel::Level60,
            70..=79 => BatteryLevel::Level70,
            80..=89 => BatteryLevel::Level80,
            90..=99 => BatteryLevel::Level90,
            100 => BatteryLevel::Level100,
            _ => BatteryLevel::Level100,
        };
        Ok(battery_state)
    }
}
