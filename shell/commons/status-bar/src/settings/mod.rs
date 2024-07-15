use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::constants::{
    BATTERY_LEVEL_0, BATTERY_LEVEL_10, BATTERY_LEVEL_100, BATTERY_LEVEL_20, BATTERY_LEVEL_30,
    BATTERY_LEVEL_40, BATTERY_LEVEL_50, BATTERY_LEVEL_60, BATTERY_LEVEL_70, BATTERY_LEVEL_80,
    BATTERY_LEVEL_90, BATTERY_NOT_FOUND, CHARGING_BATTERY_LEVEL_0, CHARGING_BATTERY_LEVEL_10,
    CHARGING_BATTERY_LEVEL_100, CHARGING_BATTERY_LEVEL_20, CHARGING_BATTERY_LEVEL_30,
    CHARGING_BATTERY_LEVEL_40, CHARGING_BATTERY_LEVEL_50, CHARGING_BATTERY_LEVEL_60,
    CHARGING_BATTERY_LEVEL_70, CHARGING_BATTERY_LEVEL_80, CHARGING_BATTERY_LEVEL_90,
    SYSTEM_MECHANIX_PATH,
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
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Layout {
    pub left: Vec<String>,   //Items that will in left side of status bar
    pub center: Vec<String>, //Items that will in center of status bar
    pub right: Vec<String>,  //Items that will in right side of status bar
}

/// # Modules
///
/// Options that will be visible in status bar
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Modules {
    pub clock: Clock,
    pub bluetooth: Bluetooth,
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
    pub icon: BluetoothIconPaths,
}

/// Wireless module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Wireless {
    pub icon: WirelessIconPaths,
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
pub struct BluetoothIconPaths {
    pub on: Option<String>,
    pub off: Option<String>,
    pub connected: Option<String>,
    pub not_found: Option<String>,
}

/// Icon paths for wireless module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WirelessIconPaths {
    pub off: Option<String>,
    pub on: Option<String>,
    pub low: Option<String>,
    pub weak: Option<String>,
    pub good: Option<String>,
    pub strong: Option<String>,
    pub not_found: Option<String>,
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
            level_100: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_100,
            level_90: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_90,
            level_80: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_80,
            level_70: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_70,
            level_60: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_60,
            level_50: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_50,
            level_40: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_40,
            level_30: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_30,
            level_20: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_20,
            level_10: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_10,
            level_0: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_LEVEL_0,
            not_found: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_NOT_FOUND,
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
            level_100: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_100,
            level_90: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_90,
            level_80: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_80,
            level_70: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_70,
            level_60: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_60,
            level_50: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_50,
            level_40: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_40,
            level_30: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_30,
            level_20: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_20,
            level_10: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_10,
            level_0: SYSTEM_MECHANIX_PATH.to_owned() + CHARGING_BATTERY_LEVEL_0,
            not_found: SYSTEM_MECHANIX_PATH.to_owned() + BATTERY_NOT_FOUND,
        }
    }
}


impl Default for Modules {
    fn default() -> Self {
        Self {
            clock: Clock {
                format: "[hour repr:12]:[minute] [period]".to_string(),
            },
            bluetooth: Bluetooth {
                icon: BluetoothIconPaths {
                    on: None,
                    off: None,
                    connected: None,
                    not_found: None,
                },
            },
            wireless: Wireless {
                icon: WirelessIconPaths {
                    off: None,
                    on: None,
                    low: None,
                    weak: None,
                    good: None,
                    strong: None,
                    not_found: None,
                },
            },
            battery: Battery {
                icon: BatteryIconPaths::default(),
                charging_icon: ChargingBatteryIconPaths::default(),
            },
        }
    }
}
