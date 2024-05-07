use crate::errors::{GreeterError, GreeterErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Greeter Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the Greeter,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GreeterSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
    pub css: CssConfigs,
}

impl Default for GreeterSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Greeter"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
            css: CssConfigs::default(),
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
            id: Some(String::from("greeter")),
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
/// the layout of options in the Greeter.
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
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
pub struct AutoRotateIconPaths {
    pub portrait: String,
    pub landscape: String,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DefaultIconPaths {
    pub default: Option<String>,
}

/// # Modules
///
/// Options that will be visible in Greeter
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HomeModule {
    pub icon: DefaultIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BackSpaceModule {
    pub icon: DefaultIconPaths,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LockModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UnlockModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BackModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NextModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SubmitModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ShowModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HideModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BackgroundModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PowerModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ShutdownModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RestartModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SleepModule {
    pub icon: DefaultIconPaths,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CloseModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PasswordConfigsModule {
    pub keys_allowed: Vec<String>,
    pub password_length: usize,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PeekPasswordModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UnPeekPasswordModule {
    pub icon: DefaultIconPaths,
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

/// Wireless module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WirelessModule {
    pub icon: WirelessIconPaths,
}

/// Battery module
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BatteryModule {
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

/// # Modules
///
/// Options that will be visible in Greeter
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {
    pub power: PowerModule,
    pub shutdown: ShutdownModule,
    pub restart: RestartModule,
    pub sleep: SleepModule,
    pub close: CloseModule,
    pub home: HomeModule,
    pub back_space: BackSpaceModule,
    pub lock: LockModule,
    pub unlock: UnlockModule,
    pub back: BackModule,
    pub next: NextModule,
    pub submit: SubmitModule,
    pub password_configs: PasswordConfigsModule,
    pub peek_password: PeekPasswordModule,
    pub un_peek_password: UnPeekPasswordModule,
    pub show: ShowModule,
    pub hide: HideModule,
    pub background: BackgroundModule,
    pub clock: ClockModule,
    pub bluetooth: BluetoothModule,
    pub wireless: WirelessModule,
    pub battery: BatteryModule,
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
            home: HomeModule {
                icon: DefaultIconPaths { default: None },
                title: "".to_string(),
            },
            back_space: BackSpaceModule {
                icon: DefaultIconPaths { default: None },
                title: "".to_string(),
            },
            lock: LockModule {
                icon: DefaultIconPaths { default: None },
            },
            unlock: UnlockModule {
                icon: DefaultIconPaths { default: None },
            },
            password_configs: PasswordConfigsModule {
                keys_allowed: vec![],
                password_length: 0,
            },
            back: BackModule {
                icon: DefaultIconPaths { default: None },
            },
            next: NextModule {
                icon: DefaultIconPaths { default: None },
            },
            submit: SubmitModule {
                icon: DefaultIconPaths { default: None },
            },
            power: PowerModule {
                icon: DefaultIconPaths { default: None },
            },
            shutdown: ShutdownModule {
                icon: DefaultIconPaths { default: None },
            },
            restart: RestartModule {
                icon: DefaultIconPaths { default: None },
            },
            sleep: SleepModule {
                icon: DefaultIconPaths { default: None },
            },
            close: CloseModule {
                icon: DefaultIconPaths { default: None },
            },
            peek_password: PeekPasswordModule {
                icon: DefaultIconPaths { default: None },
            },
            un_peek_password: UnPeekPasswordModule {
                icon: DefaultIconPaths { default: None },
            },
            show: ShowModule {
                icon: DefaultIconPaths { default: None },
            },
            hide: HideModule {
                icon: DefaultIconPaths { default: None },
            },
            background: BackgroundModule {
                icon: DefaultIconPaths { default: None },
            },
            clock: ClockModule {
                format: "[hour repr:12]:[minute] [period]".to_string(),
            },
            bluetooth: BluetoothModule {
                icon: BluetoothIconPaths {
                    on: None,
                    off: None,
                    connected: None,
                    not_found: None,
                },
            },
            wireless: WirelessModule {
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
/// Reads the `settings.yml` and parsers to GreeterSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<GreeterSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_GREETER_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
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
            bail!(GreeterError::new(
                GreeterErrorCodes::SettingsReadError,
                format!("cannot read the settings.yml in the path - {}", e),
            ));
        }
    };

    // read and parse
    let config: GreeterSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(GreeterError::new(
                GreeterErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e),
            ));
        }
    };

    Ok(config)
}
