use serde::{Deserialize, Serialize};
use tracing::{debug, info};

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
pub struct Modules {
    pub clock: Clock,
    pub bluetooth: Bluetooth,
    pub wireless: Wireless,
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
pub struct Battery {
    pub icon: BatteryIconPaths,
    pub charging_icon: BatteryIconPaths,
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
/// Icon paths for battery module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BatteryIconPaths {
    pub level_100: Option<String>,
    pub level_90: Option<String>,
    pub level_80: Option<String>,
    pub level_70: Option<String>,
    pub level_60: Option<String>,
    pub level_50: Option<String>,
    pub level_40: Option<String>,
    pub level_30: Option<String>,
    pub level_20: Option<String>,
    pub level_10: Option<String>,
    pub level_0: Option<String>,
    pub not_found: Option<String>,
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
                icon: BatteryIconPaths {
                    level_100: None,
                    level_90: None,
                    level_80: None,
                    level_70: None,
                    level_60: None,
                    level_50: None,
                    level_40: None,
                    level_30: None,
                    level_20: None,
                    level_10: None,
                    level_0: None,
                    not_found: None,
                },
                charging_icon: BatteryIconPaths {
                    level_100: None,
                    level_90: None,
                    level_80: None,
                    level_70: None,
                    level_60: None,
                    level_50: None,
                    level_40: None,
                    level_30: None,
                    level_20: None,
                    level_10: None,
                    level_0: None,
                    not_found: None,
                },
            },
        }
    }
}
