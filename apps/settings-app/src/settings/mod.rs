use crate::constants::{
    ABOUT_ICON, ADD_ICON, APPEARANCE_ICON, BACKGROUND_IMAGE, BACK_ICON, BASE_SETTINGS_PATH,
    BATTERY_ICON, BLUETOOTH_ICON, CONNECTED_ICON, DATE_TIME_ICON, DELETE_ICON, DISPLAY_ICON,
    HOME_DIR_CONFIG_PATH, INFO_ICON, LANGUAGE_ICON, LOCK_ICON, RIGHT_ARROW_ICON,
    SECURED_WIRELESS_ERROR, SECURED_WIRELESS_LOW, SECURED_WIRELESS_OFF, SECURED_WIRELESS_ON,
    SECURED_WIRELESS_STRONG, SECURED_WIRELESS_WEAK, SOUND_ICON, TICK_ICON, UPDATE_ICON,
    USR_SHARE_PATH, WIRELESS_ERROR, WIRELESS_GOOD, WIRELESS_LOW, WIRELESS_NOT_FOUND, WIRELESS_OFF,
    WIRELESS_ON, WIRELESS_SETTIGNS, WIRELESS_STRONG, WIRELESS_WEAK,
};
use crate::errors::{SettingsAppError, SettingsAppErrorCodes};
use anyhow::bail;
use anyhow::Result;
use mctk_core::renderables::types::AbsoluteLength;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Launcher Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the launcher,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MainSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
    pub fonts: CustomFonts,
}

impl Default for MainSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Launcher"),
            layout: LayoutSettings::default(),
            modules: Modules::default(),
            fonts: CustomFonts::default(),
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
            id: Some(String::from("Launcher")),
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
/// the layout of options in the launcher.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayoutSettings {
    pub left: Vec<String>,   //Items that will in left side of launcher
    pub center: Vec<String>, //Items that will in center of launcher
    pub right: Vec<String>,  //Items that will in right side of launcher
}

// #[derive(Debug, Deserialize, Clone, Serialize)]
// pub struct AppListSettings {
//     pub include_only: Vec<String>,
//     pub exclude: Vec<String>,
//     pub include: Vec<String>,
// }

// impl Default for AppListSettings {
//     fn default() -> Self {
//         Self {
//             include_only: vec![],
//             exclude: vec![],
//             include: vec![],
//             custom: vec![],
//         }
//     }
// }

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct App {
    pub app_id: String,
    pub name: String,
    pub icon: Option<String>,
    pub run_command: Vec<String>,
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
pub struct ClearModule {
    pub icon: DefaultIconPaths,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DefaultIconPaths {
    pub default: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackgroundIconPath {
    pub default: String,
}
impl Default for BackgroundIconPath {
    fn default() -> Self {
        Self {
            default: BACKGROUND_IMAGE.to_owned(),
        }
    }
}

/// # Modules
///
/// Options that will be visible in launcher
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Modules {
    pub wireless: WirelessModule,
    pub bluetooth: Bluetooth,
    pub display: Display,
    pub appearance: Appearance,
    pub battery: Battery,
    pub sound: Sound,
    pub lock: Lock,
    pub date_time: DateTime,
    pub language: Language,
    pub update: Update,
    pub about: About,
    pub footer: Footer,
    pub see_options: SeeOptions,
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
            left: vec![],
            center: vec![],
            right: vec![],
        }
    }
}

// // Wireless module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct WirelessModule {
    pub icon: WirelessIconPaths,
    pub secured_icon: SecuredWirelessIconPaths,
    pub title: String,
}
impl Default for WirelessModule {
    fn default() -> Self {
        WirelessModule {
            icon: WirelessIconPaths::default(),
            secured_icon: SecuredWirelessIconPaths::default(),
            title: "Wireless".to_string(),
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
    pub error: String,
    pub not_found: String,
    pub wireless_settings: String,
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
            error: WIRELESS_ERROR.to_owned(),
            not_found: WIRELESS_NOT_FOUND.to_owned(),
            wireless_settings: WIRELESS_SETTIGNS.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct SecuredWirelessIconPaths {
    pub off: String,
    pub on: String,
    pub low: String,
    pub weak: String,
    pub strong: String,
    pub error: String,
}
impl Default for SecuredWirelessIconPaths {
    fn default() -> Self {
        SecuredWirelessIconPaths {
            off: SECURED_WIRELESS_OFF.to_owned(),
            on: SECURED_WIRELESS_ON.to_owned(),
            low: SECURED_WIRELESS_LOW.to_owned(),
            weak: SECURED_WIRELESS_WEAK.to_owned(),
            strong: SECURED_WIRELESS_STRONG.to_owned(),
            error: SECURED_WIRELESS_ERROR.to_owned(),
        }
    }
}

/// Bluetooth module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Bluetooth {
    pub icon: String,
}
impl Default for Bluetooth {
    fn default() -> Self {
        Bluetooth {
            icon: BLUETOOTH_ICON.to_owned(),
        }
    }
}

/// Display module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Display {
    pub icon: String,
}
impl Default for Display {
    fn default() -> Self {
        Display {
            icon: DISPLAY_ICON.to_owned(),
        }
    }
}

/// Appearance module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Appearance {
    pub icon: String,
}
impl Default for Appearance {
    fn default() -> Self {
        Appearance {
            icon: APPEARANCE_ICON.to_owned(),
        }
    }
}

/// Battery module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Battery {
    pub icon: String,
}
impl Default for Battery {
    fn default() -> Self {
        Battery {
            icon: BATTERY_ICON.to_owned(),
        }
    }
}

/// Sound module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Sound {
    pub icon: String,
}
impl Default for Sound {
    fn default() -> Self {
        Sound {
            icon: SOUND_ICON.to_owned(),
        }
    }
}

/// Lock module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Lock {
    pub icon: String,
}
impl Default for Lock {
    fn default() -> Self {
        Lock {
            icon: LOCK_ICON.to_owned(),
        }
    }
}

/// date time module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct DateTime {
    pub icon: String,
}
impl Default for DateTime {
    fn default() -> Self {
        DateTime {
            icon: DATE_TIME_ICON.to_owned(),
        }
    }
}

/// Language module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Language {
    pub icon: String,
}
impl Default for Language {
    fn default() -> Self {
        Language {
            icon: LANGUAGE_ICON.to_owned(),
        }
    }
}

/// Update module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Update {
    pub icon: String,
}
impl Default for Update {
    fn default() -> Self {
        Update {
            icon: UPDATE_ICON.to_owned(),
        }
    }
}

/// About module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct About {
    pub icon: String,
}
impl Default for About {
    fn default() -> Self {
        About {
            icon: ABOUT_ICON.to_owned(),
        }
    }
}

/// SeeOptions module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct SeeOptions {
    pub right_arrow_icon: String,
    pub info_icon: String,
    pub connected_icon: String,
}
impl Default for SeeOptions {
    fn default() -> Self {
        SeeOptions {
            right_arrow_icon: RIGHT_ARROW_ICON.to_owned(),
            info_icon: INFO_ICON.to_owned(),
            connected_icon: CONNECTED_ICON.to_owned(),
        }
    }
}

/// Footer module
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Footer {
    pub back_icon: String,
    pub tick_icon: String,
    pub delete_icon: String,
    pub add_icon: String,
}
impl Default for Footer {
    fn default() -> Self {
        Footer {
            back_icon: BACK_ICON.to_owned(),
            tick_icon: TICK_ICON.to_owned(),
            delete_icon: ADD_ICON.to_owned(),
            add_icon: DELETE_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CustomFonts {
    pub paths: Vec<String>,
}
impl Default for CustomFonts {
    fn default() -> Self {
        Self { paths: vec![] }
    }
}

impl Default for Modules {
    fn default() -> Self {
        Self {
            wireless: WirelessModule::default(),
            bluetooth: Bluetooth::default(),
            battery: Battery::default(),
            display: Display::default(),
            appearance: Appearance::default(),
            see_options: SeeOptions::default(),
            sound: Sound::default(),
            lock: Lock::default(),
            date_time: DateTime::default(),
            language: Language::default(),
            update: Update::default(),
            about: About::default(),
            footer: Footer::default(),
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

fn is_valid_file(path: &str) -> Option<PathBuf> {
    let path_buf = PathBuf::from(path);
    println!(
        "CHECKING PATH {} EXIST ===>  {:?}",
        path,
        path_buf.is_file()
    );
    if path_buf.is_file() {
        Some(path_buf)
    } else {
        None
    }
}

fn find_config_path() -> Option<PathBuf> {
    // from env
    if let Ok(env_path) = std::env::var("MECHA_LAUNCHER_SETTINGS_PATH") {
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
    if let Some(home_dir) = dirs::home_dir() {
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
/// Reads the `settings.yml` and parsers to MainSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<MainSettings> {
    let file_path = find_config_path();

    if file_path.is_none() {
        bail!(SettingsAppError::new(
            SettingsAppErrorCodes::SettingsReadError,
            format!("settings.yml path not found",),
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
            bail!(SettingsAppError::new(
                SettingsAppErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: MainSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(SettingsAppError::new(
                SettingsAppErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
