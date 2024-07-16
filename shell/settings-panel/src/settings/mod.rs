use crate::constants::{BATTERY_LEVEL_0, BATTERY_LEVEL_10, BATTERY_LEVEL_100, BATTERY_LEVEL_20, BATTERY_LEVEL_30, BATTERY_LEVEL_40, BATTERY_LEVEL_50, BATTERY_LEVEL_60, BATTERY_LEVEL_70, BATTERY_LEVEL_80, BATTERY_LEVEL_90, BATTERY_NOT_FOUND, BLUETOOTH_CONNECTED, BLUETOOTH_NOT_FOUND, BLUETOOTH_OFF, BLUETOOTH_ON, CHARGING_BATTERY_LEVEL_0, CHARGING_BATTERY_LEVEL_10, CHARGING_BATTERY_LEVEL_100, CHARGING_BATTERY_LEVEL_20, CHARGING_BATTERY_LEVEL_30, CHARGING_BATTERY_LEVEL_40, CHARGING_BATTERY_LEVEL_50, CHARGING_BATTERY_LEVEL_60, CHARGING_BATTERY_LEVEL_70, CHARGING_BATTERY_LEVEL_80, CHARGING_BATTERY_LEVEL_90, SYSTEM_MECHANIX_SETTINGS_PANEL_PATH, WIRELESS_GOOD, WIRELESS_LOW, WIRELESS_NOT_FOUND, WIRELESS_OFF, WIRELESS_ON, WIRELESS_STRONG, WIRELESS_WEAK};
use crate::errors::{SettingsPanelError, SettingsPanelErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Settings panel Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the Settings panel,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SettingsPanelSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
    pub fonts: HashMap<String, String>,
}

impl Default for SettingsPanelSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Settings panel"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
            fonts: HashMap::new(),
        }
    }
}

/// # App Settings
///
/// Struct part of settings.yml to control the application
/// behavior, includes optimizations and defaults
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppSettings {
    pub id: Option<String>,        // Process ID
    pub text_multithreading: bool, // Enable text multithreading
    pub antialiasing: bool,        // Enable antialiasing
    pub try_opengles_first: bool,  // Enable using OpenGL ES before OpenGL (only for flow)
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            id: Some(String::from("settings-panel")),
            text_multithreading: false,
            antialiasing: false,
            try_opengles_first: true,
        }
    }
}

/// # Window Settings
///
/// Part of the settings.yml to control the behavior of
/// the application window
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WindowSettings {
    pub size: (i32, i32),             // Size of the window
    pub position: (i32, i32),         // Default position to start window
    pub min_size: Option<(u32, u32)>, // Minimum size the window can be resized to
    pub max_size: Option<(u32, u32)>, // Maximum size the window can be resized to
    pub visible: bool,                // Sets visibility of the window
    pub resizable: bool,              // Enables or disables resizing
    pub decorations: bool,            // Enables or disables the title bar
    pub transparent: bool,            // Enables transparency
    pub always_on_top: bool,          // Forces window to be always on top
    pub icon_path: Option<String>,
}

/// # Layout Settings
///
/// Part of the settings.yml to control the behavior of
/// the layout of options in the settings panel.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayoutSettings {
    pub grid: Vec<String>, //Items that will in grid
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CssConfigs {
    pub default: String,
}

impl Default for CssConfigs {
    fn default() -> Self {
        Self {
            default: "".to_string(),
        }
    }
}

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
            off: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BLUETOOTH_OFF,
            on: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BLUETOOTH_ON,
            connected: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BLUETOOTH_CONNECTED,
            not_found: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BLUETOOTH_NOT_FOUND,
        }
    }
}


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
            off: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + WIRELESS_OFF,
            on: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + WIRELESS_ON,
            low: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + WIRELESS_LOW,
            weak: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + WIRELESS_WEAK,
            good: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + WIRELESS_GOOD,
            strong: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + WIRELESS_STRONG,
            not_found: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + WIRELESS_NOT_FOUND,
        }
    }
}


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
            level_100: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_100,
            level_90: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_90,
            level_80: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_80,
            level_70: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_70,
            level_60: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_60,
            level_50: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_50,
            level_40: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_40,
            level_30: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_30,
            level_20: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_20,
            level_10: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_10,
            level_0: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_LEVEL_0,
            not_found: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_NOT_FOUND,
        }
    }
}



#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)] 
pub struct BatteryChrgingIconPaths {
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
impl Default for BatteryChrgingIconPaths {
    fn default() ->  Self {
        BatteryChrgingIconPaths {
            level_100: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_100,
            level_90: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_90,
            level_80: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_80,
            level_70: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_70,
            level_60: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_60,
            level_50: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_50,
            level_40: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_40,
            level_30: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_30,
            level_20: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_20,
            level_10: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_10,
            level_0: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + CHARGING_BATTERY_LEVEL_0,
            not_found: SYSTEM_MECHANIX_SETTINGS_PANEL_PATH.to_owned() + BATTERY_NOT_FOUND,
        }
    }
}


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RotationIconPaths {
    pub portrait: Option<String>,
    pub landscape: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SettingsIconPaths {
    pub default: Option<String>,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CommonLowMediumHighPaths {
    pub low: Option<String>,
    pub medium: Option<String>,
    pub high: Option<String>,
}

/// # Modules Definitions
/// Options that will be visible in settings panel
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BluetoothModule {
    pub icon: BluetoothIconPaths,
    pub title: String,
}
impl Default for BluetoothModule {
    fn default() ->  Self {
        BluetoothModule {
        icon: BluetoothIconPaths::default(),
        title: "Bluetooth".to_string(),
    }}
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct WirelessModule {
    pub icon: WirelessIconPaths,
    pub title: String,
}
impl Default for WirelessModule {
    fn default() ->  Self {
        WirelessModule {
        icon: WirelessIconPaths::default(),
        title: "Wireless".to_string(),
    }}
}


#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BatteryModule {
    pub icon: BatteryIconPaths,
    pub title: String,
    pub charging_icon: BatteryChrgingIconPaths,
}
impl Default for BatteryModule {
    fn default() ->  Self {
        BatteryModule {
        title: "Battery".to_string(),
        icon: BatteryIconPaths::default(),
        charging_icon:  BatteryChrgingIconPaths::default()
    }}
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RotationModule {
    pub icon: RotationIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SettingsModule {
    pub icon: SettingsIconPaths,
    pub title: String,
    pub run_command: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RunningAppsModule {
    pub icon: CommonLowMediumHighPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CpuModule {
    pub icon: CommonLowMediumHighPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MemoryModule {
    pub icon: CommonLowMediumHighPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SoundModule {
    pub icon: CommonLowMediumHighPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BrightnessModule {
    pub icon: CommonLowMediumHighPaths,
    pub title: String,
}

/// # Modules
///
/// Options that will be visible in settings panel
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(default)] 
pub struct Modules {
    pub wireless: WirelessModule,
    pub bluetooth: BluetoothModule,
    pub battery: BatteryModule,
    pub rotation: RotationModule,
    pub settings: SettingsModule,
    pub running_apps: RunningAppsModule,
    pub cpu: CpuModule,
    pub memory: MemoryModule,
    pub sound: SoundModule,
    pub brightness: BrightnessModule,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            size: (1024, 768),
            position: (0, 0),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon_path: None,
        }
    }
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self { grid: vec![] }
    }
}

impl Default for Modules {
    fn default() -> Self {
        Self {
            bluetooth: BluetoothModule::default(),
            wireless: WirelessModule::default(),
            battery: BatteryModule::default(),
            rotation: RotationModule {
                icon: RotationIconPaths {
                    portrait: None,
                    landscape: None,
                },
                title: "Auto Rotate".to_string(),
            },
            settings: SettingsModule {
                icon: SettingsIconPaths { default: None },
                title: "Settings".to_string(),
                run_command: vec![],
            },
            running_apps: RunningAppsModule {
                icon: CommonLowMediumHighPaths {
                    low: None,
                    medium: None,
                    high: None,
                },
                title: "Running Apps".to_string(),
            },
            cpu: CpuModule {
                icon: CommonLowMediumHighPaths {
                    low: None,
                    medium: None,
                    high: None,
                },
                title: "CPU".to_string(),
            },
            memory: MemoryModule {
                icon: CommonLowMediumHighPaths {
                    low: None,
                    medium: None,
                    high: None,
                },
                title: "Memory".to_string(),
            },
            sound: SoundModule {
                icon: CommonLowMediumHighPaths {
                    low: None,
                    medium: None,
                    high: None,
                },
                title: "Sound".to_string(),
            },
            brightness: BrightnessModule {
                icon: CommonLowMediumHighPaths {
                    low: None,
                    medium: None,
                    high: None,
                },
                title: "Brightness".to_string(),
            },
        }
    }
}

/// # Reads Settings path from arg
///
/// Reads the `-s` or `--settings` argument for the path
pub fn read_settings_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    println!("args are {:?}", args);
    if args.len() > 1 && (args[1] == "-s" || args[1] == "--settings") {
        debug!("Using settings path from argument - {}", args[2]);
        return Some(args[2].clone());
    }
    None
}

/// # Reads Settings YML
///
/// Reads the `settings.yml` and parsers to SettingsPanelSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<SettingsPanelSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_SETTINGS_PANEL_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
    ); // Get path of the library

    // read from args
    let file_path_in_args = read_settings_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

    println!("settings file location - {:?}", file_path);

    // open file
    let settings_file_handle = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("settings read error {:?}", e.to_string());
            bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: SettingsPanelSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            println!("settings parse error {:?}", e.to_string());
            bail!(SettingsPanelError::new(
                SettingsPanelErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    println!("config {:?}", config);

    Ok(config)
}
