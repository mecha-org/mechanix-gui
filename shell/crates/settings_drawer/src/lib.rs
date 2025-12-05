mod events;
pub mod services;
mod ui;

pub mod prelude {
    pub use crate::events::{AppEvents, BrightnessEvents, BtEvents, NmEvents, VolumeEvents};
    pub use crate::ui::SettingsDrawer;
}

use crate::ui::icon::IconName;

pub fn get_wireless_strength_icon(enable: bool, signal_strength: u8, security: String) -> IconName {
    if enable {
        match security.as_str() {
            "Open" => match signal_strength {
                0 => IconName::ConnectedWifiOn,
                0..=30 => IconName::ConnectedWifiLow,
                31..=60 => IconName::ConnectedWifiMedium,
                61..=100 => IconName::ConnectedWifiHigh,
                _ => IconName::ConnectedWifiWarning,
            },
            "Protected" => match signal_strength {
                0..=30 => IconName::ConnectedWifiLowLocked,
                31..=60 => IconName::ConnectedWifiMediumLocked,
                61..=100 => IconName::ConnectedWifiHighLocked,
                _ => IconName::ConnectedWifiWarning,
            },
            _ => IconName::ConnectedWifiWarning,
        }
    } else {
        match security.as_str() {
            "Open" => match signal_strength {
                0 => IconName::WifiOn,
                0..=30 => IconName::WifiLow,
                31..=60 => IconName::WifiMedium,
                61..=100 => IconName::WifiHigh,
                _ => IconName::WifiWarning,
            },
            "Protected" => match signal_strength {
                0..=30 => IconName::WifiLowLocked,
                31..=60 => IconName::WifiMediumLocked,
                61..=100 => IconName::WifiHighLocked,
                _ => IconName::WifiWarning,
            },
            _ => IconName::WifiWarning,
        }
    }
}

pub fn get_bluetooth_icon(connected: bool) -> IconName {
    if connected {
        IconName::BluetoothConnected
    } else {
        IconName::BluetoothOff
    }
}
