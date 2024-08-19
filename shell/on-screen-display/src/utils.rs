use std::collections::HashMap;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use upower::BatteryStatus;

use crate::{
    settings::{BatteryIconPaths, BluetoothIconPaths, ChargingBatteryIconPaths, WirelessIconPaths},
    types::{BatteryLevel, BluetoothStatus, WirelessConnectedState, WirelessStatus},
};

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

pub fn get_battery_icons_map(icon_paths: BatteryIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.not_found {
        assets.insert(BatteryLevel::NotFound.to_string(), value.clone());
    }

    if let value = &icon_paths.level_0 {
        assets.insert(BatteryLevel::Level0.to_string(), value.clone());
    }

    if let value = &icon_paths.level_10 {
        assets.insert(BatteryLevel::Level10.to_string(), value.clone());
    }

    if let value = &icon_paths.level_20 {
        assets.insert(BatteryLevel::Level20.to_string(), value.clone());
    }

    if let value = &icon_paths.level_30 {
        assets.insert(BatteryLevel::Level30.to_string(), value.clone());
    }

    if let value = &icon_paths.level_40 {
        assets.insert(BatteryLevel::Level40.to_string(), value.clone());
    }

    if let value = &icon_paths.level_50 {
        assets.insert(BatteryLevel::Level50.to_string(), value.clone());
    }

    if let value = &icon_paths.level_60 {
        assets.insert(BatteryLevel::Level60.to_string(), value.clone());
    }
    if let value = &icon_paths.level_70 {
        assets.insert(BatteryLevel::Level70.to_string(), value.clone());
    }

    if let value = &icon_paths.level_80 {
        assets.insert(BatteryLevel::Level80.to_string(), value.clone());
    }
    if let value = &icon_paths.level_90 {
        assets.insert(BatteryLevel::Level90.to_string(), value.clone());
    }
    if let value = &icon_paths.level_100 {
        assets.insert(BatteryLevel::Level100.to_string(), value.clone());
    }

    assets
}

pub fn get_battery_icons_charging_map(
    icon_paths: ChargingBatteryIconPaths,
) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.level_0 {
        assets.insert(BatteryLevel::ChargingLevel0.to_string(), value.clone());
    }

    if let value = &icon_paths.level_10 {
        assets.insert(BatteryLevel::ChargingLevel10.to_string(), value.clone());
    }

    if let value = &icon_paths.level_20 {
        assets.insert(BatteryLevel::ChargingLevel20.to_string(), value.clone());
    }

    if let value = &icon_paths.level_30 {
        assets.insert(BatteryLevel::ChargingLevel30.to_string(), value.clone());
    }

    if let value = &icon_paths.level_40 {
        assets.insert(BatteryLevel::ChargingLevel40.to_string(), value.clone());
    }

    if let value = &icon_paths.level_50 {
        assets.insert(BatteryLevel::ChargingLevel50.to_string(), value.clone());
    }

    if let value = &icon_paths.level_60 {
        assets.insert(BatteryLevel::ChargingLevel60.to_string(), value.clone());
    }
    if let value = &icon_paths.level_70 {
        assets.insert(BatteryLevel::ChargingLevel70.to_string(), value.clone());
    }

    if let value = &icon_paths.level_80 {
        assets.insert(BatteryLevel::ChargingLevel80.to_string(), value.clone());
    }
    if let value = &icon_paths.level_90 {
        assets.insert(BatteryLevel::ChargingLevel90.to_string(), value.clone());
    }
    if let value = &icon_paths.level_100 {
        assets.insert(BatteryLevel::ChargingLevel100.to_string(), value.clone());
    }

    assets
}

pub fn get_bluetooth_icons_map(icon_paths: BluetoothIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.not_found {
        assets.insert(BluetoothStatus::NotFound.to_string(), value.clone());
    }

    if let value = &icon_paths.on {
        assets.insert(BluetoothStatus::On.to_string(), value.clone());
    }

    if let value = &icon_paths.off {
        assets.insert(BluetoothStatus::Off.to_string(), value.clone());
    }

    if let value = &icon_paths.connected {
        assets.insert(BluetoothStatus::Connected.to_string(), value.clone());
    }

    assets
}

pub fn get_wireless_icons_map(icon_paths: WirelessIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.not_found {
        assets.insert(WirelessStatus::NotFound.to_string(), value.clone());
    }

    if let value = &icon_paths.on {
        assets.insert(WirelessStatus::On.to_string(), value.clone());
    }

    if let value = &icon_paths.off {
        assets.insert(WirelessStatus::Off.to_string(), value.clone());
    }

    if let value = &icon_paths.weak {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Weak).to_string(),
            value.clone(),
        );
    }

    if let value = &icon_paths.low {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Low).to_string(),
            value.clone(),
        );
    }

    if let value = &icon_paths.good {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Good).to_string(),
            value.clone(),
        );
    }
    if let value = &icon_paths.strong {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Strong).to_string(),
            value.clone(),
        );
    }

    assets
}

pub fn fill_grid_with_true(rows: usize, cols: usize, mut num_true: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; cols]; rows];
    let mut rng = thread_rng();

    if num_true > rows * cols {
        println!("Number of true values exceeds grid size.");
        num_true = rows * cols;
    }

    let mut positions: Vec<(usize, usize)> = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .collect();
    positions.shuffle(&mut rng);

    for &(r, c) in positions.iter().take(num_true) {
        grid[r][c] = true;
    }

    grid
}
