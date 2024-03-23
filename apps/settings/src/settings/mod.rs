use crate::errors::{LockScreenError, LockScreenErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # LockScreen Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the lock screen,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LockScreenSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
    pub widget_configs: WidgetConfigs,
    pub css: CssConfigs,
}

impl Default for LockScreenSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Lock Screen"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
            css: CssConfigs::default(),
            widget_configs: WidgetConfigs::default(),
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
            id: Some(String::from("lock-screen")),
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
/// the layout of options in the lock screen.
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
pub struct SettingsItem {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
}

impl Default for SettingsItem {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            title: "".to_string(),
            icon: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SettingsModule {
    pub title: String,
    pub icon: Option<String>,
    pub items: Vec<SettingsItem>,
}

impl Default for SettingsModule {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            icon: None,
            items: vec![],
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
/// Options that will be visible in lock screen
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
pub struct SubmitModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HomePasswordModule {
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

/// # Modules
///
/// Options that will be visible in lock screen
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {
    pub home: HomeModule,
    pub back_space: BackSpaceModule,
    pub lock: LockModule,
    pub unlock: UnlockModule,
    pub back: BackModule,
    pub submit: SubmitModule,
    pub home_password: HomePasswordModule,
    pub password_configs: PasswordConfigsModule,
    pub peek_password: PeekPasswordModule,
    pub un_peek_password: UnPeekPasswordModule,
    pub settings: SettingsModule,
    pub pages_settings: PagesSettings,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PagesSettings {
    pub network: NetworkPageSettings,
    pub bluetooth: BluetoothPageSettings,
    pub display: DisplayPageSettings,
    pub battery: BatteryPageSettings,
    pub sound: SoundPageSettings,
    pub security: SecurityPageSettings,
    pub dateandtime: DateTimePageSettings,
    pub about: AboutPageSettings,
}

impl Default for PagesSettings {
    fn default() -> Self {
        Self {
            network: NetworkPageSettings::default(),
            bluetooth: BluetoothPageSettings::default(),
            display: DisplayPageSettings::default(),
            battery: BatteryPageSettings::default(),
            sound: SoundPageSettings::default(),
            security: SecurityPageSettings::default(),
            dateandtime: DateTimePageSettings::default(),
            about: AboutPageSettings::default()
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NetworkPageSettings {
    pub network_icon: Option<String>,
    pub is_enabled: bool,
}

impl Default for NetworkPageSettings {
    fn default() -> Self {
        Self {
            network_icon: None,
            is_enabled: true,
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BluetoothPageSettings {
    pub bluetooth_icon: Option<String>,
    pub is_enabled: bool,
}

impl Default for BluetoothPageSettings {
    fn default() -> Self {
        Self {
            bluetooth_icon: None,
            is_enabled: true,
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DisplayPageSettings {
    pub display_icon: Option<String>,
    pub is_enabled: bool,
}

impl Default for DisplayPageSettings {
    fn default() -> Self {
        Self {
            display_icon: None,
            is_enabled: true,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BatteryPageSettings {
    pub display_icon: Option<String>,
    pub is_enabled: bool,
}

impl Default for BatteryPageSettings {
    fn default() -> Self {
        Self {
            display_icon: None,
            is_enabled: true,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SoundPageSettings {
    pub display_icon: Option<String>,
    pub is_enabled: bool,
}

impl Default for SoundPageSettings {
    fn default() -> Self {
        Self {
            display_icon: None,
            is_enabled: true,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SecurityPageSettings {
    pub display_icon: Option<String>,
    pub is_enabled: bool,
}

impl Default for SecurityPageSettings {
    fn default() -> Self {
        Self {
            display_icon: None,
            is_enabled: true,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DateTimePageSettings {
    pub display_icon: Option<String>,
    pub is_enabled: bool,
    pub window_size: (i32, i32),  
}

impl Default for DateTimePageSettings {
    fn default() -> Self {
        Self {
            display_icon: None,
            is_enabled: true,
            window_size: (1024, 768),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AboutPageSettings {
    pub display_icon: Option<String>,
    pub is_enabled: bool,
}

impl Default for AboutPageSettings {
    fn default() -> Self {
        Self {
            display_icon: None,
            is_enabled: true,
        }
    }
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
            submit: SubmitModule {
                icon: DefaultIconPaths { default: None },
            },
            home_password: HomePasswordModule {
                icon: DefaultIconPaths { default: None },
            },
            peek_password: PeekPasswordModule {
                icon: DefaultIconPaths { default: None },
            },
            un_peek_password: UnPeekPasswordModule {
                icon: DefaultIconPaths { default: None },
            },
            settings: SettingsModule::default(),
            pages_settings: PagesSettings::default(),
        }
    }
}

/// # Custom Widgets config
///
/// Custom Widgets config
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WidgetConfigs {
    pub menu_item: MenuItemWidgetConfigs,
    pub network_item: NetworkItemWidgetConfigs,
    pub radio_item: RadioItemWidgetConfigs,
    pub footer: FooterWidgetConfigs,
}

impl Default for WidgetConfigs {
    fn default() -> Self {
        Self {
            menu_item: MenuItemWidgetConfigs::default(),
            network_item: NetworkItemWidgetConfigs::default(),
            radio_item: RadioItemWidgetConfigs::default(),
            footer: FooterWidgetConfigs::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NetworkItemWidgetConfigs {
    pub connected_icon: Option<String>,
    pub private_icon: Option<String>,
    pub wifi_100_icon: Option<String>,
    pub info_icon: Option<String>,
}

impl Default for NetworkItemWidgetConfigs {
    fn default() -> Self {
        Self {
            connected_icon: None,
            private_icon: None,
            wifi_100_icon: None,
            info_icon: None,
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RadioItemWidgetConfigs {
    pub active_icon: Option<String>,
    pub inactive_icon: Option<String>,
}

impl Default for RadioItemWidgetConfigs {
    fn default() -> Self {
        Self {
            active_icon: None,
            inactive_icon: None,
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FooterWidgetConfigs {
    pub back_icon: Option<String>,
    pub next_icon: Option<String>,
    pub add_icon: Option<String>,
    pub trash_icon: Option<String>,
}

impl Default for FooterWidgetConfigs {
    fn default() -> Self {
        Self {
            back_icon: None,
            next_icon: None,
            add_icon: None,
            trash_icon: None,
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MenuItemWidgetConfigs {
    pub end_icon: Option<String>,
}

impl Default for MenuItemWidgetConfigs {
    fn default() -> Self {
        Self { end_icon: None }
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
/// Reads the `settings.yml` and parsers to LockScreenSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<LockScreenSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_LOCK_SCREEN_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
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
            bail!(LockScreenError::new(
                LockScreenErrorCodes::SettingsReadError,
                format!("cannot read the settings.yml in the path - {}", e),
            ));
        }
    };

    // read and parse
    let config: LockScreenSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e),
            ));
        }
    };

    Ok(config)
}
