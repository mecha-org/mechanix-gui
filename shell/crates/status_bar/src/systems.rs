use crate::{ClockUpdateTimer, components::*};
use bevy_plugins::{
    UPowerBatteryState,
    NetworkManagerDeviceState,
    bluetooth::{BluetoothDeviceConnectedStatus, BluetoothEnabledStatus},
    network_manager::{ActiveNetworkStrength, WirelessEnabled},
    upower::{DevicePercentage, DeviceState},
};
use chrono::{Datelike, Timelike};
use bevy_plugins::network_manager::NetworkManagerDeviceStatus;
use types::prelude::IconAssets;

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

pub fn get_current_datetime() -> String {
    let now = chrono::Local::now();
    format!(
        "{} {} {:02}:{:02}:{:02}",
        now.day(),
        now.format("%B"),
        now.hour(),
        now.minute(),
        now.second()
    )
}

pub fn update_clock(
    time: Res<Time>,
    mut timer: ResMut<ClockUpdateTimer>,
    mut query: Query<&mut Text, With<Clock>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut text in &mut query {
            text.0 = get_current_datetime();
        }
    }
}
pub fn update_wireless_state(
    mut query: Query<&mut ImageNode, With<Wireless>>,
    wifi_state: Res<WirelessEnabled>,
    icon_assets: Res<IconAssets>,
) {
    for mut image_node in &mut query {
        info!("WirelessEnabled is updated :{:?}", wifi_state);
        if wifi_state.0 {
            image_node.image = icon_assets.wifi_on.clone();
        } else {
            image_node.image = icon_assets.wifi_off.clone();
        }
    }
}
pub fn update_wireless_network_strength(
    icon_assets: Res<IconAssets>,
    mut query: Query<&mut ImageNode, With<Wireless>>,
    active_network_strength: Res<ActiveNetworkStrength>,
) {
    for mut wireless_icon in &mut query {
        wireless_icon.image = match active_network_strength.0 {
            0..=20 => icon_assets.wifi_low.clone(),
            21..=50 => icon_assets.wifi_low.clone(), // Ask for icon
            51..=75 => icon_assets.wifi_medium.clone(),
            76..=100 => icon_assets.wifi_high.clone(),
            _ => unreachable!(),
        }
    }
}

pub fn update_wireless_device_status(
    mut query: Query<&mut ImageNode, With<Wireless>>,
    wireless_enabled_status: Res<WirelessEnabled>,
    wireless_device_status: Res<NetworkManagerDeviceStatus>,
    icon_assets: Res<IconAssets>,
) {
    for mut image_node in &mut query {
        info!("enabled status:{:?}, device status: {:?}", wireless_enabled_status, wireless_device_status);
        if wireless_device_status.0 == NetworkManagerDeviceState::ConnectedLocal && wireless_enabled_status.0 {
            image_node.image = icon_assets.wifi_on.clone();
        }
    }
}
pub fn update_bluetooth_on_powered(
    icon_assets: Res<IconAssets>,
    mut query: Query<&mut ImageNode, With<Bluetooth>>,
    bluetooth_state: Res<BluetoothEnabledStatus>,
) {
    for mut icon in &mut query {
        if bluetooth_state.0 {
            icon.image = icon_assets.bluetooth_on.clone();
        } else {
            icon.image = icon_assets.bluetooth_off.clone();
        }
    }
}

pub fn update_bluetooth_on_connected(
    icon_assets: Res<IconAssets>,
    mut query: Query<&mut ImageNode, With<Bluetooth>>,
    connected_status: Res<BluetoothDeviceConnectedStatus>,
) {
    for mut icon in &mut query {
        if connected_status.0 {
            icon.image = icon_assets.bluetooth_connected.clone();
        } else {
            icon.image = icon_assets.bluetooth_on.clone();
        }
    }
}

pub fn update_power_icon(
    icon_assets: Res<IconAssets>,
    mut query: Query<&mut ImageNode, With<Battery>>,
    device_percentage: Res<DevicePercentage>,
    device_state: Res<DeviceState>,
) {
    for mut styled_text in &mut query {
        match (device_state.0.clone(), device_percentage.0.round() as u32) {
            (UPowerBatteryState::Charging, p) if p >= 95 => {
                styled_text.image = icon_assets.battery_100_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 90 => {
                styled_text.image = icon_assets.battery_90_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 80 => {
                styled_text.image = icon_assets.battery_80_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 70 => {
                styled_text.image = icon_assets.battery_70_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 60 => {
                styled_text.image = icon_assets.battery_60_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 50 => {
                styled_text.image = icon_assets.battery_50_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 40 => {
                styled_text.image = icon_assets.battery_40_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 30 => {
                styled_text.image = icon_assets.battery_30_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 20 => {
                styled_text.image = icon_assets.battery_20_charging.clone();
            }
            (UPowerBatteryState::Charging, p) if p >= 10 => {
                styled_text.image = icon_assets.battery_10_charging.clone();
            }
            (UPowerBatteryState::Charging, _) => {
                styled_text.image = icon_assets.battery_0_charging.clone();
            }

            (UPowerBatteryState::Discharging, p) if p >= 95 => {
                styled_text.image = icon_assets.battery_100.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 90 => {
                styled_text.image = icon_assets.battery_90.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 80 => {
                styled_text.image = icon_assets.battery_80.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 70 => {
                styled_text.image = icon_assets.battery_70.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 60 => {
                styled_text.image = icon_assets.battery_60.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 50 => {
                styled_text.image = icon_assets.battery_50.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 40 => {
                styled_text.image = icon_assets.battery_40.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 30 => {
                styled_text.image = icon_assets.battery_30.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 20 => {
                styled_text.image = icon_assets.battery_20.clone();
            }
            (UPowerBatteryState::Discharging, p) if p >= 10 => {
                styled_text.image = icon_assets.battery_10.clone();
            }
            (UPowerBatteryState::Discharging, _) => {
                styled_text.image = icon_assets.battery_empty.clone();
            }
            (UPowerBatteryState::Empty, _) => {
                styled_text.image = icon_assets.battery_empty.clone();
            }
            _ => {
                // Default fallback icon
                styled_text.image = icon_assets.battery_empty.clone();
            }
        }
    }
}
