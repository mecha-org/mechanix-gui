use crate::errors::{StatusBarError, StatusBarErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # StatusBar Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the status bar,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct StatusBarSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
    pub css: CssConfigs
}

impl Default for StatusBarSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Status Bar"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
            css: CssConfigs::default()
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
            id: Some(String::from("status-bar")),
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
/// the layout of options in the status bar.
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct LayoutSettings {
    pub left: Vec<String>,   //Items that will in left side of status bar
    pub center: Vec<String>, //Items that will in center of status bar
    pub right: Vec<String>,  //Items that will in right side of status bar
}

/// # Modules
///
/// Options that will be visible in status bar
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {
    pub clock: ClockModule,
    pub bluetooth: BluetoothModule,
    pub wifi: WifiModule,
    pub battery: BatteryModule,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CssConfigs {
    pub default: String,
}

impl Default for CssConfigs {
    fn default() -> Self {
        Self { 
            default: "".to_string() 
        }
    }
}

/// Clock module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ClockModule {
    pub format: String,
}

/// Bluetooth module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BluetoothModule {
    pub icon: BluetoothIconPaths,
}

/// Wifi module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WifiModule {
    pub icon: WifiIconPaths,
}

/// Battery module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BatteryModule {
    pub icon: BatteryIconPaths,
}

/// Icon paths for bluetooth module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BluetoothIconPaths {
    pub on: Option<String>,
    pub off: Option<String>,
    pub connected: Option<String>,
}

/// Icon paths for wifi module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WifiIconPaths {
    pub off: Option<String>,
    pub on: Option<String>,
    pub low: Option<String>,
    pub weak: Option<String>,
    pub good: Option<String>,
    pub strong: Option<String>,
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

impl Default for Modules {
    fn default() -> Self {
        Self {
            clock: ClockModule {
                format: "[hour repr:12]:[minute] [period]".to_string(),
            },
            bluetooth: BluetoothModule {
                icon: BluetoothIconPaths {
                    on: None,
                    off: None,
                    connected: None,
                },
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
/// Reads the `settings.yml` and parsers to StatusBarSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<StatusBarSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_STATUS_BAR_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
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
            bail!(StatusBarError::new(
                StatusBarErrorCodes::SettingsReadError,
                format!("Cannot read the settings.yml in the path - {}", e),
                true
            ));
        }
    };

    // read and parse
    let config: StatusBarSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(StatusBarError::new(
                StatusBarErrorCodes::SettingsParseError,
                format!("Error parsing the settings.yml - {}", e),
                true
            ));
        }
    };

    info!("settings read is {:?}", config);

    Ok(config)
}
