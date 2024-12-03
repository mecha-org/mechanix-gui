use std::collections::HashMap;

use mctk_core::AssetParams;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use upower::BatteryStatus;

use crate::{
    settings::{
        BatteryIconPaths, BluetoothIcons, ChargingBatteryIconPaths, LgWirelessIconPaths,
        SmWirelessIconPaths, WirelessIcons,
    },
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

pub fn get_battery_icons_map(icon_paths: BatteryIconPaths) -> HashMap<String, AssetParams> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.not_found {
        assets.insert(
            BatteryLevel::NotFound.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_0 {
        assets.insert(
            BatteryLevel::Level0.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_10 {
        assets.insert(
            BatteryLevel::Level10.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_20 {
        assets.insert(
            BatteryLevel::Level20.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_30 {
        assets.insert(
            BatteryLevel::Level30.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_40 {
        assets.insert(
            BatteryLevel::Level40.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_50 {
        assets.insert(
            BatteryLevel::Level50.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_60 {
        assets.insert(
            BatteryLevel::Level60.to_string(),
            AssetParams::new(value.clone()),
        );
    }
    if let value = &icon_paths.level_70 {
        assets.insert(
            BatteryLevel::Level70.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_80 {
        assets.insert(
            BatteryLevel::Level80.to_string(),
            AssetParams::new(value.clone()),
        );
    }
    if let value = &icon_paths.level_90 {
        assets.insert(
            BatteryLevel::Level90.to_string(),
            AssetParams::new(value.clone()),
        );
    }
    if let value = &icon_paths.level_100 {
        assets.insert(
            BatteryLevel::Level100.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    assets
}

pub fn get_battery_icons_charging_map(
    icon_paths: ChargingBatteryIconPaths,
) -> HashMap<String, AssetParams> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.level_0 {
        assets.insert(
            BatteryLevel::ChargingLevel0.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_10 {
        assets.insert(
            BatteryLevel::ChargingLevel10.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_20 {
        assets.insert(
            BatteryLevel::ChargingLevel20.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_30 {
        assets.insert(
            BatteryLevel::ChargingLevel30.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_40 {
        assets.insert(
            BatteryLevel::ChargingLevel40.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_50 {
        assets.insert(
            BatteryLevel::ChargingLevel50.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_60 {
        assets.insert(
            BatteryLevel::ChargingLevel60.to_string(),
            AssetParams::new(value.clone()),
        );
    }
    if let value = &icon_paths.level_70 {
        assets.insert(
            BatteryLevel::ChargingLevel70.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    if let value = &icon_paths.level_80 {
        assets.insert(
            BatteryLevel::ChargingLevel80.to_string(),
            AssetParams::new(value.clone()),
        );
    }
    if let value = &icon_paths.level_90 {
        assets.insert(
            BatteryLevel::ChargingLevel90.to_string(),
            AssetParams::new(value.clone()),
        );
    }
    if let value = &icon_paths.level_100 {
        assets.insert(
            BatteryLevel::ChargingLevel100.to_string(),
            AssetParams::new(value.clone()),
        );
    }

    assets
}

pub fn get_bluetooth_icons_map(icons: BluetoothIcons) -> HashMap<String, AssetParams> {
    let mut assets = HashMap::new();

    let sm = icons.sm;
    let lg = icons.lg;
    //sm
    assets.insert(
        format!("sm{:?}", BluetoothStatus::NotFound.to_string()),
        AssetParams::new(sm.not_found),
    );
    assets.insert(
        format!("sm{:?}", BluetoothStatus::On.to_string()),
        AssetParams::new(sm.on),
    );

    assets.insert(
        format!("sm{:?}", BluetoothStatus::Off.to_string()),
        AssetParams::new(sm.off),
    );

    assets.insert(
        format!("sm{:?}", BluetoothStatus::Connected.to_string()),
        AssetParams::new(sm.connected),
    );

    //lg
    assets.insert(
        format!("lg{:?}", BluetoothStatus::NotFound.to_string()),
        AssetParams::new(lg.not_found),
    );
    assets.insert(
        format!("lg{:?}", BluetoothStatus::On.to_string()),
        AssetParams::new(lg.on),
    );

    assets.insert(
        format!("lg{:?}", BluetoothStatus::Off.to_string()),
        AssetParams::new(lg.off),
    );

    assets.insert(
        format!("lg{:?}", BluetoothStatus::Connected.to_string()),
        AssetParams::new(lg.connected),
    );

    assets
}

pub fn get_wireless_icons_map(icons: WirelessIcons) -> HashMap<String, AssetParams> {
    let mut assets = HashMap::new();

    let sm = icons.sm;
    let lg = icons.lg;

    //sm
    assets.insert(
        format!("sm{:?}", WirelessStatus::NotFound.to_string()),
        AssetParams::new(sm.not_found.clone()),
    );

    assets.insert(
        format!("sm{:?}", WirelessStatus::On.to_string()),
        AssetParams::new(sm.on.clone()),
    );
    assets.insert(
        format!("sm{:?}", WirelessStatus::Off.to_string()),
        AssetParams::new(sm.off.clone()),
    );
    assets.insert(
        format!(
            "sm{:?}",
            WirelessStatus::Connected(WirelessConnectedState::Weak).to_string()
        ),
        AssetParams::new(sm.weak.clone()),
    );
    assets.insert(
        format!(
            "sm{:?}",
            WirelessStatus::Connected(WirelessConnectedState::Low).to_string()
        ),
        AssetParams::new(sm.low.clone()),
    );

    assets.insert(
        format!(
            "sm{:?}",
            WirelessStatus::Connected(WirelessConnectedState::Good).to_string()
        ),
        AssetParams::new(sm.good.clone()),
    );
    assets.insert(
        format!(
            "sm{:?}",
            WirelessStatus::Connected(WirelessConnectedState::Strong).to_string()
        ),
        AssetParams::new(sm.strong.clone()),
    );

    assets.insert(
        format!("sm{:?}", WirelessStatus::NotFound.to_string()),
        AssetParams::new(sm.not_found.clone()),
    );

    //lg
    assets.insert(
        format!("lg{:?}", WirelessStatus::On.to_string()),
        AssetParams::new(lg.on.clone()),
    );
    assets.insert(
        format!("lg{:?}", WirelessStatus::Off.to_string()),
        AssetParams::new(lg.off.clone()),
    );
    assets.insert(
        format!(
            "lg{:?}",
            WirelessStatus::Connected(WirelessConnectedState::Weak).to_string()
        ),
        AssetParams::new(lg.weak.clone()),
    );
    assets.insert(
        format!(
            "lg{:?}",
            WirelessStatus::Connected(WirelessConnectedState::Low).to_string()
        ),
        AssetParams::new(lg.low.clone()),
    );

    assets.insert(
        format!(
            "lg{:?}",
            WirelessStatus::Connected(WirelessConnectedState::Good).to_string()
        ),
        AssetParams::new(lg.good.clone()),
    );
    assets.insert(
        format!(
            "lg{:?}",
            WirelessStatus::Connected(WirelessConnectedState::Strong).to_string()
        ),
        AssetParams::new(lg.strong.clone()),
    );
    assets.insert(
        format!("lg{:?}", WirelessStatus::NotFound.to_string()),
        AssetParams::new(lg.not_found.clone()),
    );

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

pub fn cubic_bezier(arr: &[f64; 4], t: f64) -> f64 {
    let ut = 1.0 - t;
    let a1 = arr[1] * ut + arr[2] * t;
    ((arr[0] * ut + arr[1] * t) * ut + a1 * t) * ut + (a1 * ut + (arr[2] * ut + arr[3] * t) * t) * t
}
