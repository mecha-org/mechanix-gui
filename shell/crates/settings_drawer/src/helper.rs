use std::path::PathBuf;

use gpui::*;
use icons::prelude::*;

pub fn get_wireless_strength_icon(
    enabled: bool,
    signal_strength: u8,
    security: String,
    cx: &mut App,
) -> PathBuf {
    let SettingsDrawerIcons {
        connected_wireless_on,
        connected_wireless_low,
        connected_wireless_medium,
        connected_wireless_high,
        connected_wireless_low_locked,
        connected_wireless_medium_locked,
        connected_wireless_high_locked,
        connected_wireless_warning,
        wireless_none,
        wireless_low,
        wireless_medium,
        wireless_high,
        wireless_low_locked,
        wireless_medium_locked,
        wireless_high_locked,
        wireless_warning,
        ..
    } = Icons::global(cx).settings_drawer.clone();
    let level = if signal_strength == 0 {
        "none"
    } else if signal_strength <= 30 {
        "low"
    } else if signal_strength <= 60 {
        "medium"
    } else if signal_strength <= 100 {
        "high"
    } else {
        "unknown"
    };

    let is_protected = security == "Protected";

    if enabled {
        if is_protected {
            match level {
                "low" => connected_wireless_low_locked,
                "medium" => connected_wireless_medium_locked,
                "high" => connected_wireless_high_locked,
                _ => connected_wireless_warning,
            }
        } else {
            match level {
                "none" => connected_wireless_on,
                "low" => connected_wireless_low,
                "medium" => connected_wireless_medium,
                "high" => connected_wireless_high,
                _ => connected_wireless_warning,
            }
        }
    } else {
        if is_protected {
            match level {
                "low" => wireless_low_locked,
                "medium" => wireless_medium_locked,
                "high" => wireless_high_locked,
                _ => wireless_warning,
            }
        } else {
            match level {
                "none" => wireless_none,
                "low" => wireless_low,
                "medium" => wireless_medium,
                "high" => wireless_high,
                _ => wireless_warning,
            }
        }
    }
}

pub fn get_bluetooth_icon(connected: bool, cx: &mut App) -> PathBuf {
    let SettingsDrawerIcons {
        bluetooth_connected,
        bluetooth_off,
        ..
    } = Icons::global(cx).settings_drawer.clone();
    if connected {
        bluetooth_connected
    } else {
        bluetooth_off
    }
}
