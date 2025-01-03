use std::{collections::HashMap, time::Duration};

use mctk_core::AssetParams;
use networkmanager::WirelessModel;
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

pub fn get_forttated_wireless_status(wireless_model: &WirelessModel) -> WirelessStatus {
    let is_enabled = wireless_model.is_enabled.get();
    if !*is_enabled {
        return WirelessStatus::Off;
    }
    let connected = wireless_model.connected_network.get();
    if connected.is_none() {
        return WirelessStatus::On;
    }
    let connected = connected.as_ref().unwrap();
    let signal = connected.signal.parse::<i32>().unwrap();

    if signal <= -80 {
        return WirelessStatus::Connected(WirelessConnectedState::Low);
    } else if signal <= -60 {
        return WirelessStatus::Connected(WirelessConnectedState::Weak);
    } else if signal <= -40 {
        return WirelessStatus::Connected(WirelessConnectedState::Good);
    } else {
        return WirelessStatus::Connected(WirelessConnectedState::Strong);
    };
}

pub async fn get_ip_address() -> Option<String> {
    let mut ip_address: Option<String> = None;
    let mut networks = sysinfo::Networks::new_with_refreshed_list();
    tokio::time::sleep(Duration::from_millis(100)).await;
    networks.refresh(true);
    println!("get_ip_address networks are {:?}", networks);
    for (interface_name, network) in &networks {
        match interface_name.as_str() {
            "enp5s0" => {
                let ip_networks = network.ip_networks();
                println!("ip_networks are {:?}", ip_networks);
                for ip_network in ip_networks {
                    if ip_network.addr.is_ipv4() {
                        println!("enp5s0 {:?}", ip_network.addr.to_string());
                        ip_address = Some(ip_network.addr.to_string());
                        break;
                    }
                }
            }
            "wlan0" => {
                let ip_networks = network.ip_networks();
                println!("ip_networks are {:?}", ip_networks);
                for ip_network in ip_networks {
                    if ip_network.addr.is_ipv4() {
                        println!("wlan0 {:?}", ip_network.addr.to_string());
                        ip_address = Some(ip_network.addr.to_string());
                        break;
                    }
                }
            }
            _ => (),
        }
    }
    ip_address
}
