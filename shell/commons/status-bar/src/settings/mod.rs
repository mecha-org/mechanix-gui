use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::constants::{BATTERY_LEVEL_0, BATTERY_LEVEL_10, BATTERY_LEVEL_100, BATTERY_LEVEL_20, BATTERY_LEVEL_30, BATTERY_LEVEL_40, BATTERY_LEVEL_50, BATTERY_LEVEL_60, BATTERY_LEVEL_70, BATTERY_LEVEL_80, BATTERY_LEVEL_90, BATTERY_NOT_FOUND, BLUETOOTH_CONNECTED, BLUETOOTH_NOT_FOUND, BLUETOOTH_OFF, BLUETOOTH_ON, CHARGING_BATTERY_LEVEL_0, CHARGING_BATTERY_LEVEL_10, CHARGING_BATTERY_LEVEL_100, CHARGING_BATTERY_LEVEL_20, CHARGING_BATTERY_LEVEL_30, CHARGING_BATTERY_LEVEL_40, CHARGING_BATTERY_LEVEL_50, CHARGING_BATTERY_LEVEL_60, CHARGING_BATTERY_LEVEL_70, CHARGING_BATTERY_LEVEL_80, CHARGING_BATTERY_LEVEL_90, WIRELESS_GOOD, WIRELESS_LOW, WIRELESS_NOT_FOUND, WIRELESS_OFF, WIRELESS_ON, WIRELESS_STRONG, WIRELESS_WEAK
};

/// # StatusBar Settings
///
/// Struct representing the settings.yml configuration,
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct StatusBarSettings {
    pub layout: Layout,
    pub modules: Modules,
}

impl Default for StatusBarSettings {
    fn default() -> Self {
        Self {
            layout: Layout::default(),
            modules: Modules::default(),
        }
    }
}

/// # Layout Settings
///
/// Part of the settings.yml to control the behavior of
/// the layout of options in the status bar.
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Layout {
    pub left: Vec<String>,   //Items that will in left side of status bar
    pub center: Vec<String>, //Items that will in center of status bar
    pub right: Vec<String>,  //Items that will in right side of status bar
}
impl Default for Layout {
    fn default() -> Self {
        Self {
            left: ["clock"].map(String::from).to_vec(),
            center: ["window_title"].map(String::from).to_vec(),
            right: ["wireless", "bluetooth", "battery"].map(String::from).to_vec(),
        }
    }
}

/// # Modules
///
/// Options that will be visible in status bar
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Modules {
    pub clock: Clock,
    #[serde(default)]
    pub bluetooth: Bluetooth,
    #[serde(default)]
    pub wireless: Wireless,
    #[serde(default)]
    pub battery: Battery,
}

/// Clock module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Clock {
    pub format: String,
}

/// Bluetooth module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Bluetooth {
    #[serde(default)]
    pub icon: BluetoothIconPaths,
}
impl Default for Bluetooth {
    fn default() ->  Self {
        Bluetooth {
        icon: BluetoothIconPaths::default(),
    }}
}

/// Wireless module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Wireless {
    #[serde(default)]
    pub icon: WirelessIconPaths,
}
impl Default for Wireless {
    fn default() ->  Self {
        Wireless {
        icon: WirelessIconPaths::default(),
    }}
}


/// Battery module 
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Battery {
    #[serde(default)]
    pub icon: BatteryIconPaths,
    #[serde(default)] 
    pub charging_icon: ChargingBatteryIconPaths,
}
impl Default for Battery {
    fn default() ->  Self {
        Battery {
        icon: BatteryIconPaths::default(),
        charging_icon: ChargingBatteryIconPaths::default(),
    }}
}


/// Icon paths for bluetooth module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct BluetoothIconPaths {
    pub on: String,
    pub off: String,
    pub connected: String,
    pub not_found: String,
}
impl Default for BluetoothIconPaths {
    fn default() -> Self {
        BluetoothIconPaths {
            off: BLUETOOTH_OFF.to_owned(),
            on: BLUETOOTH_ON.to_owned(),
            connected: BLUETOOTH_CONNECTED.to_owned(),
            not_found: BLUETOOTH_NOT_FOUND.to_owned(),
        }
    
    }
}

/// Icon paths for wireless module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct WirelessIconPaths {
    pub off: String,
    pub on: String,
    pub low: String,
    pub weak: String,
    pub good: String,
    pub strong: String,
    pub not_found: String,
}
impl Default for WirelessIconPaths {
    fn default() -> Self {
        WirelessIconPaths {
            off: WIRELESS_OFF.to_owned(),
            on: WIRELESS_ON.to_owned(),
            low: WIRELESS_LOW.to_owned(),
            weak: WIRELESS_WEAK.to_owned(),
            good: WIRELESS_GOOD.to_owned(),
            strong: WIRELESS_STRONG.to_owned(),
            not_found: WIRELESS_NOT_FOUND.to_owned(),
        }
    
    }
}

// /// Icon paths for battery module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct BatteryIconPaths {
    pub level_100: String,
    pub level_90: String,
    pub level_80: String,
    pub level_70: String,
    pub level_60: String,
    pub level_50: String,
    pub level_40: String,
    pub level_30: String,
    pub level_20: String,
    pub level_10: String,
    pub level_0: String,
    pub not_found: String,
}

impl Default for BatteryIconPaths {
    fn default() ->  Self {
        BatteryIconPaths {
            level_100: BATTERY_LEVEL_100.to_owned(),
            level_90: BATTERY_LEVEL_90.to_owned(),
            level_80: BATTERY_LEVEL_80.to_owned(),
            level_70: BATTERY_LEVEL_70.to_owned(),
            level_60: BATTERY_LEVEL_60.to_owned(),
            level_50: BATTERY_LEVEL_50.to_owned(),
            level_40: BATTERY_LEVEL_40.to_owned(),
            level_30: BATTERY_LEVEL_30.to_owned(),
            level_20: BATTERY_LEVEL_20.to_owned(),
            level_10: BATTERY_LEVEL_10.to_owned(),
            level_0: BATTERY_LEVEL_0.to_owned(),
            not_found: BATTERY_NOT_FOUND.to_owned(),
        }
    }
}



#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct ChargingBatteryIconPaths {
    pub level_100: String,
    pub level_90: String,
    pub level_80: String,
    pub level_70: String,
    pub level_60: String,
    pub level_50: String,
    pub level_40: String,
    pub level_30: String,
    pub level_20: String,
    pub level_10: String,
    pub level_0: String,
    pub not_found: String,
}

impl Default for ChargingBatteryIconPaths {
    fn default() ->  Self {
        ChargingBatteryIconPaths {
            level_100: CHARGING_BATTERY_LEVEL_100.to_owned(),
            level_90: CHARGING_BATTERY_LEVEL_90.to_owned(),
            level_80: CHARGING_BATTERY_LEVEL_80.to_owned(),
            level_70: CHARGING_BATTERY_LEVEL_70.to_owned(),
            level_60: CHARGING_BATTERY_LEVEL_60.to_owned(),
            level_50: CHARGING_BATTERY_LEVEL_50.to_owned(),
            level_40: CHARGING_BATTERY_LEVEL_40.to_owned(),
            level_30: CHARGING_BATTERY_LEVEL_30.to_owned(),
            level_20: CHARGING_BATTERY_LEVEL_20.to_owned(),
            level_10: CHARGING_BATTERY_LEVEL_10.to_owned(),
            level_0: CHARGING_BATTERY_LEVEL_0.to_owned(),
            not_found: BATTERY_NOT_FOUND.to_owned(),
        }
    }
}


impl Default for Modules {
    fn default() -> Self {
        Self {
            clock: Clock {
                format: "%I:%M %p".to_string(),
            },
            bluetooth: Bluetooth {
                icon: BluetoothIconPaths::default(),
            },
            wireless: Wireless {
                icon: WirelessIconPaths::default(),
            },
            battery: Battery {
                icon: BatteryIconPaths::default(),
                charging_icon: ChargingBatteryIconPaths::default(),
            },
        }
    }
}
