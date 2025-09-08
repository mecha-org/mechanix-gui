use bevy::platform::collections::HashMap;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct DateSettings {
    pub time: String,
    pub date: String,
    pub day: String,
}

impl Default for DateSettings {
    fn default() -> Self {
        Self {
            time: "%H:%M".to_string(), // https://docs.rs/chrono/latest/chrono/format/strftime/index.html
            date: "%e %B".to_string(), //- https://docs.rs/chrono/latest/chrono/format/strftime/index.html
            day: "%A".to_string(), // https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct RightTrayStatus {
    pub headphones_connected: bool,
    pub monitor_connected: bool,
    pub terminal_active: bool,
    pub usb_connected: bool,
}

#[derive(Debug, Clone)]
pub enum WifiState {
    ConnectedStrong,
    Off,
    OnButNotConnected,
    // Connecting,
    ConnectedMedium,
    ConnectedWeak,
    ConnectedNoInternet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BluetoothState {
    On,
    Off,
    Connected,
    // Transferring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState {
    Charging,
    LowBattery,
    CriticallyLow,
    ChargedComplete,
    NoBattery,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MobileNetworkState {
    Full,
    Medium,
    Low,
    NoSimInserted,
    NoSignal,
}

#[derive(Resource, Debug, Clone)]
pub struct SystemStatus {
    pub bluetooth_state: BluetoothState,
    pub wifi_state: WifiState,
    pub mobile_network_state: MobileNetworkState,
    pub battery_state: BatteryState,
}

#[derive(Debug, Clone)]
pub struct StatusBarSettings {
    pub width: f32,
    pub height: f32,
    pub menus: HashMap<String, Vec<String>>,
    pub date: DateSettings,
}

impl Default for StatusBarSettings {
    fn default() -> Self {
        Self {
            width: 100.,
            height: 5.,
            //date menu is for date and time settings
            //system menu is for bluetooth, wireless, network and battery
            //right box includes system tray
            //center box includes empty space
            //logo includes logo
            //left box includes other options
            menus: HashMap::from([
                (
                    "sm".to_string(),
                    Vec::from([
                        "date".to_string(),
                        "center".to_string(),
                        "right".to_string(),
                        "system".to_string(),
                    ]),
                ),
                (
                    "lg".to_string(),
                    Vec::from([
                        "logo".to_string(),
                        "left".to_string(),
                        "center".to_string(),
                        "right".to_string(),
                        "system".to_string(),
                        "date".to_string(),
                    ]),
                ),
            ]),
            date: DateSettings::default(),
        }
    }
}
