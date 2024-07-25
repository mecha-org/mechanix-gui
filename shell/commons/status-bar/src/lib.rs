use types::{BatteryLevel, BatteryStatus, BluetoothStatus, WirelessStatus};

pub mod errors;
pub mod gui;
pub mod modules;
pub mod settings;
pub mod types;
pub mod constants;

#[derive(Debug)]
pub enum StatusBarMessage {
    Clock { current_time: String },
    Window { title: String, activated: bool },
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: u8, status: BatteryStatus },
}

pub fn get_formatted_battery_level(level: &u8, status: &BatteryStatus) -> BatteryLevel {
    if *status == BatteryStatus::Unknown {
        BatteryLevel::NotFound
    } else if *status == BatteryStatus::Charging {
        match level {
            0..=9 => BatteryLevel::ChargingLevel10,
            10..=19 => BatteryLevel::ChargingLevel20,
            20..=34 => BatteryLevel::ChargingLevel30,
            35..=49 => BatteryLevel::ChargingLevel40,
            50..=59 => BatteryLevel::ChargingLevel50,
            60..=69 => BatteryLevel::ChargingLevel60,
            70..=79 => BatteryLevel::ChargingLevel70,
            80..=89 => BatteryLevel::ChargingLevel80,
            90..=94 => BatteryLevel::ChargingLevel90,
            95..=100 => BatteryLevel::ChargingLevel100,
            _ => BatteryLevel::NotFound,
        }
    } else {
        match *level {
            0..=9 => BatteryLevel::Level10,
            10..=19 => BatteryLevel::Level20,
            20..=34 => BatteryLevel::Level30,
            35..=49 => BatteryLevel::Level40,
            50..=59 => BatteryLevel::Level50,
            60..=69 => BatteryLevel::Level60,
            70..=79 => BatteryLevel::Level70,
            80..=89 => BatteryLevel::Level80,
            90..=94 => BatteryLevel::Level90,
            95..=100 => BatteryLevel::Level100,
            _ => BatteryLevel::NotFound,
        }
    }
}
