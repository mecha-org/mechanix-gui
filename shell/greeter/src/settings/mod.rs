use crate::constants::{
    BACKGROUND_IMAGE, BACKSPACE_ICON, BACK_ICON, CLOSE_ICON, HIDE_ICON, HOME_DIR_PATH, HOME_ICON, LOCK_ICON, NEXT_ICON, PASSWORD_LENGTH, PEEK_PASSWORD_ICON, POWER_ICON, RESTART_ICON, SHOW_ICON, SHUTDOWN_ICON, SLEEP_ICON, SUBMIT_ICON, UNLOCK_ICON, UNPEEK_PASSWORD_ICON
};
use crate::errors::{GreeterError, GreeterErrorCodes};
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
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Greeter Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the Greeter,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
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
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct LayoutSettings {
    pub grid: Vec<String>, //Items that will in grid
}
impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            grid: ["1", "2", "3", "4", "5", "6", "7", "8", "", "9", "0"]
                .map(String::from)
                .to_vec(),
        }
    }
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

/// # Modules
///
/// Options that will be visible in Greeter
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
        Self {
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
        Self {
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
        Self {
            icon: BackIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct NextModule {
    pub icon: NextIconPath,
}
impl Default for NextModule {
    fn default() -> Self {
        Self {
            icon: NextIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct SubmitModule {
    pub icon: SubmitIconPath,
}
impl Default for SubmitModule {
    fn default() -> Self {
        Self {
            icon: SubmitIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct ShowModule {
    pub icon: ShowIconPath,
}
impl Default for ShowModule {
    fn default() -> Self {
        Self {
            icon: ShowIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct HideModule {
    pub icon: HideIconPath,
}
impl Default for HideModule {
    fn default() -> Self {
        Self {
            icon: HideIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackgroundModule {
    pub icon: BackgroundIconPath,
}
impl Default for BackgroundModule {
    fn default() -> Self {
        Self {
            icon: BackgroundIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct PowerModule {
    pub icon: PowerIconPath,
}
impl Default for PowerModule {
    fn default() -> Self {
        Self {
            icon: PowerIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct ShutdownModule {
    pub icon: ShutdownIconPath,
}
impl Default for ShutdownModule {
    fn default() -> Self {
        Self {
            icon: ShutdownIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct RestartModule {
    pub icon: RestartIconPath,
}
impl Default for RestartModule {
    fn default() -> Self {
        Self {
            icon: RestartIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct SleepModule {
    pub icon: SleepIconPath,
}
impl Default for SleepModule {
    fn default() -> Self {
        Self {
            icon: SleepIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct CloseModule {
    pub icon: CloseIconPath,
}
impl Default for CloseModule {
    fn default() -> Self {
        Self {
            icon: CloseIconPath::default(),
        }
    }
}

//  TODO
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct PasswordConfigsModule {
    pub keys_allowed: Vec<String>,
    pub password_length: usize,
}
impl Default for PasswordConfigsModule {
    fn default() -> Self {
        Self {
            keys_allowed: ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"].map(String::from).to_vec(),
            password_length: PASSWORD_LENGTH,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct PeekPasswordModule {
    pub icon: PeekPasswordIconPath,
}
impl Default for PeekPasswordModule {
    fn default() -> Self {
        Self {
            icon: PeekPasswordIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct UnPeekPasswordModule {
    pub icon: UnPeekPasswordIconPath,
}
impl Default for UnPeekPasswordModule {
    fn default() -> Self {
        Self {
            icon: UnPeekPasswordIconPath::default(),
        }
    }
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
pub struct NextIconPath {
    pub default: String,
}
impl Default for NextIconPath {
    fn default() -> Self {
        NextIconPath {
            default: NEXT_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct SubmitIconPath {
    pub default: String,
}
impl Default for SubmitIconPath {
    fn default() -> Self {
        SubmitIconPath {
            default: SUBMIT_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct ShowIconPath {
    pub default: String,
}
impl Default for ShowIconPath {
    fn default() -> Self {
        ShowIconPath {
            default: SHOW_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct HideIconPath {
    pub default: String,
}
impl Default for HideIconPath {
    fn default() -> Self {
        HideIconPath {
            default: HIDE_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackgroundIconPath {
    pub default: String,
}
impl Default for BackgroundIconPath {
    fn default() -> Self {
        BackgroundIconPath {
            default: BACKGROUND_IMAGE.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct PowerIconPath {
    pub default: String,
}
impl Default for PowerIconPath {
    fn default() -> Self {
        PowerIconPath {
            default: POWER_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct ShutdownIconPath {
    pub default: String,
}
impl Default for ShutdownIconPath {
    fn default() -> Self {
        ShutdownIconPath {
            default: SHUTDOWN_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct RestartIconPath {
    pub default: String,
}
impl Default for RestartIconPath {
    fn default() -> Self {
        RestartIconPath {
            default: RESTART_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct SleepIconPath {
    pub default: String,
}
impl Default for SleepIconPath {
    fn default() -> Self {
        SleepIconPath {
            default: SLEEP_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct PeekPasswordIconPath {
    pub default: String,
}
impl Default for PeekPasswordIconPath {
    fn default() -> Self {
        PeekPasswordIconPath {
            default: PEEK_PASSWORD_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct UnPeekPasswordIconPath {
    pub default: String,
}
impl Default for UnPeekPasswordIconPath {
    fn default() -> Self {
        UnPeekPasswordIconPath {
            default: UNPEEK_PASSWORD_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct CloseIconPath {
    pub default: String,
}
impl Default for CloseIconPath {
    fn default() -> Self {
        CloseIconPath {
            default: CLOSE_ICON.to_owned(),
        }
    }
}

/// # Modules
///
/// Options that will be visible in Greeter
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
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
    pub clock: Clock,
    pub bluetooth: Bluetooth,
    pub wireless: Wireless,
    pub battery: Battery,
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

impl Default for Modules {
    fn default() -> Self {
        Self {
            home: HomeModule::default(),
            back_space: BackSpaceModule::default(),
            lock: LockModule::default(),
            unlock: UnlockModule::default(),
            password_configs: PasswordConfigsModule::default(),
            back: BackModule::default(),
            next: NextModule::default(),
            submit: SubmitModule::default(),
            power: PowerModule::default(),
            shutdown: ShutdownModule::default(),
            restart: RestartModule::default(),
            sleep: SleepModule::default(),
            close: CloseModule::default(),
            peek_password: PeekPasswordModule::default(),
            un_peek_password: UnPeekPasswordModule::default(),
            show: ShowModule::default(),
            hide: HideModule::default(),
            background: BackgroundModule::default(),
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

fn is_empty_path(path: &PathBuf) -> bool {
    path.as_os_str().is_empty()
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

    if is_empty_path(&file_path) {
        let home_dir = dirs::home_dir().unwrap();
        file_path = home_dir.join(HOME_DIR_PATH);
    }
    println!("settings file location - {:?}", file_path);
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
