use crate::errors::{SettingsDrawerError, SettingsDrawerErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Settings Drawer Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the Settings drawer,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SettingsDrawerSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
}

impl Default for SettingsDrawerSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Settings Drawer"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
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
            id: Some(String::from("settings-drawer")),
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
/// the layout of options in the settings drawer.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayoutSettings {
    pub grid: Vec<String>, //Items that will in grid
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BluetoothIconPaths {
    pub on: Option<String>,
    pub off: Option<String>,
    pub connected: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WifiIconPaths {
    pub off: Option<String>,
    pub on: Option<String>,
    pub low: Option<String>,
    pub weak: Option<String>,
    pub good: Option<String>,
    pub strong: Option<String>,
}

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
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AutoRotateIconPaths {
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
///
/// Options that will be visible in settings drawer
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BluetoothModule {
    pub icon: BluetoothIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WifiModule {
    pub icon: WifiIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BatteryModule {
    pub icon: BatteryIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AutoRotateModule {
    pub icon: AutoRotateIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SettingsModule {
    pub icon: SettingsIconPaths,
    pub title: String,
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
/// Options that will be visible in settings drawer
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {
    pub wifi: WifiModule,
    pub bluetooth: BluetoothModule,
    pub battery: BatteryModule,
    pub auto_rotate: AutoRotateModule,
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
            bluetooth: BluetoothModule {
                icon: BluetoothIconPaths {
                    on: None,
                    off: None,
                    connected: None,
                },
                title: "Bluetooth".to_string(),
            },
            wifi: WifiModule {
                icon: WifiIconPaths {
                    off: None,
                    on: None,
                    low: None,
                    weak: None,
                    good: None,
                    strong: None,
                },
                title: "Wifi".to_string(),
            },
            battery: BatteryModule {
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
                },
                title: "Battery".to_string(),
            },
            auto_rotate: AutoRotateModule {
                icon: AutoRotateIconPaths {
                    portrait: None,
                    landscape: None,
                },
                title: "Auto Rotate".to_string(),
            },
            settings: SettingsModule {
                icon: SettingsIconPaths { default: None },
                title: "Settings".to_string(),
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
    if args.len() > 2 && (args[1] == "-s" || args[1] == "--settings") {
        debug!("Using settings path from argument - {}", args[2]);
        return Some(String::from(args[2].clone()));
    }
    None
}

/// # Reads Settings YML
///
/// Reads the `settings.yml` and parsers to SettingsDrawerSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<SettingsDrawerSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_SETTINGS_DRAWER_SETTINGS_PATH")
            .unwrap_or(String::from("settings.yml")),
    ); // Get path of the library

    // read from args
    let file_path_in_args = read_settings_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

    info!(
        task = "read_settings",
        "settings file location - {:?}", file_path
    );

    // open file
    let settings_file_handle = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            bail!(SettingsDrawerError::new(
                SettingsDrawerErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: SettingsDrawerSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(SettingsDrawerError::new(
                SettingsDrawerErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
