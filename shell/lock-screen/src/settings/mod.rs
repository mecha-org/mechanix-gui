use crate::constants::{BACKGROUND_IMAGE, BACKSPACE_ICON, BACK_ICON, BASE_SETTINGS_PATH, HOME_DIR_CONFIG_PATH, HOME_ICON, LOCK_ICON, PASSWORD_LENGTH, UNLOCK_ICON, USR_SHARE_PATH};
use crate::errors::{LockScreenError, LockScreenErrorCodes};
use anyhow::bail;
use anyhow::Result; 
use mechanix_status_bar_components::settings::Battery;
use mechanix_status_bar_components::settings::BatteryIconPaths;
use mechanix_status_bar_components::settings::Bluetooth;
use mechanix_status_bar_components::settings::BluetoothIconPaths;
use mechanix_status_bar_components::settings::ChargingBatteryIconPaths;
use mechanix_status_bar_components::settings::Clock;
use mechanix_status_bar_components::settings::Wireless;
use mechanix_status_bar_components::settings::WirelessIconPaths;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # LockScreen Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the lock screen,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct LockScreenSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
    pub fonts: HashMap<String, String>,
}

impl Default for LockScreenSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Lock Screen"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
            fonts: HashMap::new(),
        }
    }
}

/// # Modules
///
/// Options that will be visible in lock screen
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Modules {
    pub clock: Clock,
    pub bluetooth: Bluetooth,
    pub wireless: Wireless,
    pub battery: Battery,
    pub home: HomeModule,
    pub back_space: BackSpaceModule,
    pub lock: LockModule,
    pub unlock: UnlockModule,
    pub back: BackModule,
    pub background: BackgroundModule,
    pub password_configs: PasswordConfigsModule,
}

impl Default for Modules {
    fn default() -> Self {
        Self {
            home: HomeModule::default(),
            back_space: BackSpaceModule::default(),
            back: BackModule::default(),
            lock: LockModule::default(),
            unlock: UnlockModule::default(),
            background: BackgroundModule::default(),
            password_configs: PasswordConfigsModule {
                keys_allowed: ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"].map(String::from).to_vec(),
                password_length: PASSWORD_LENGTH,
            },
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
    pub size: (u32, u32),             // Size of the window
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
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct LayoutSettings {
    pub grid: Vec<String>, //Items that will in grid
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AutoRotateIconPaths {
    pub portrait: String,
    pub landscape: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct HomeIconPath {
    pub default: String,
}
impl Default for HomeIconPath {
    fn default() -> Self {
        HomeIconPath {
            default: HOME_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackSpaceIconPath {
    pub default: String,
}
impl Default for BackSpaceIconPath {
    fn default() -> Self {
        BackSpaceIconPath {
            default: BACKSPACE_ICON.to_owned(),
        }
    }
}


#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct LockIconPath {
    pub default: String,
}
impl Default for LockIconPath {
    fn default() -> Self {
        LockIconPath {
            default: LOCK_ICON.to_owned(),
        }
    }
}


#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct UnlockIconPath {
    pub default: String,
}
impl Default for UnlockIconPath {
    fn default() -> Self {
        UnlockIconPath {
            default: UNLOCK_ICON.to_owned(),
        }
    }
}


#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackIconPath {
    pub default: String,
}
impl Default for BackIconPath {
    fn default() -> Self {
        BackIconPath {
            default: BACK_ICON.to_owned(),
        }
    }
}


#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackgroundImagePath {
    pub default: String,
}
impl Default for BackgroundImagePath {
    fn default() -> Self {
        BackgroundImagePath {
            default: BACKGROUND_IMAGE.to_owned(),
        }
    }
}


/// # Modules
///
/// Options that will be visible in lock screen
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct HomeModule {
    pub icon: HomeIconPath,
    pub title: String,
}
impl Default for HomeModule {
    fn default() -> Self {
        HomeModule {
            icon: HomeIconPath::default(),
            title: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackSpaceModule {
    pub icon: BackSpaceIconPath,
    pub title: String,
}
impl Default for BackSpaceModule {
    fn default() -> Self {
        BackSpaceModule {
            icon: BackSpaceIconPath::default(),
            title: "".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct LockModule {
    pub icon: LockIconPath,
}
impl Default for LockModule {
    fn default() -> Self {
        LockModule {
            icon: LockIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct UnlockModule {
    pub icon: UnlockIconPath,
}
impl Default for UnlockModule {
    fn default() -> Self {
        UnlockModule {
            icon: UnlockIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackModule {
    pub icon: BackIconPath,
}
impl Default for BackModule {
    fn default() -> Self {
        BackModule {
            icon: BackIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackgroundModule {
    pub icon: BackgroundImagePath,
}
impl Default for BackgroundModule {
    fn default() -> Self {
        BackgroundModule {
            icon: BackgroundImagePath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PasswordConfigsModule {
    pub keys_allowed: Vec<String>,
    pub password_length: usize,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            size: (480, 440),
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
        Self { 
            grid: ["1", "2", "3", "4", "5", "6", "7", "8", "Home", "9", "0", "Back Space"].map(String::from).to_vec()
        }
    }
}

/// # Reads Settings path from arg
///
/// Reads the `-s` or `--settings` argument for the path
pub fn read_settings_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && (args[1] == "-s" || args[1] == "--settings") {
        debug!("using settings path from argument - {}", args[2]);
        return Some(String::from(args[2].clone()));
    }
    None
}

fn is_valid_file(path: &str) -> Option<PathBuf> {
    let path_buf = PathBuf::from(path);
    println!("CHECKING PATH {} EXIST ===>  {:?}", path, path_buf.is_file());
    if path_buf.is_file() {
        Some(path_buf)
    } else {
        None
    }
}

fn find_config_path() -> Option<PathBuf> {

    // from env 
    if let Ok(env_path) = std::env::var("MECHANIX_LOCK_SCREEN_SETTINGS_PATH") {
        if let Some(path) = is_valid_file(&env_path) {
            return Some(path);
        }
    }   

    // read from args
    if let Some(arg) = read_settings_path_from_args() {
        if let Some(file_path_in_args) = is_valid_file(&arg) {
            return Some(PathBuf::from(file_path_in_args));
        }
    } 

    // read from local settings 
    if let settings_path = String::from("settings.yml") {
        if let Some(path) = is_valid_file(&settings_path) {
            return Some(path);
        } 
    }  

    // home config dir
    if let Some(home_dir) = dirs::home_dir(){
        let mut path = home_dir;
        path.push(&format!("{}{}", HOME_DIR_CONFIG_PATH, BASE_SETTINGS_PATH)); // Replace with your actual path
        if let Some(path) = is_valid_file(path.to_str().unwrap()) {
            return Some(path);
        }
    } 
    
    // default usr dir
    let default_path = format!("{}{}", USR_SHARE_PATH, BASE_SETTINGS_PATH);
    is_valid_file(&default_path) 

}

/// # Reads Settings YML
///
/// Reads the `settings.yml` and parsers to LockScreenSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<LockScreenSettings> {
    let file_path = find_config_path();

    if file_path.is_none() {
        bail!(LockScreenError::new(
            LockScreenErrorCodes::SettingsReadError,
            format!(
                "settings.yml path not found",
            ),
        ));
    }

    info!(
        task = "read_settings",
        "settings file location - {:?}", file_path
    );

    // open file
    let settings_file_handle = match File::open(file_path.unwrap()) {
        Ok(file) => file,
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: LockScreenSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(LockScreenError::new(
                LockScreenErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
