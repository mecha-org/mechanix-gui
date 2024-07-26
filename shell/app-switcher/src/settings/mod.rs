use crate::constants::{
    APP_NAMESPACE, BASE_SETTINGS_PATH, BECK_ICON, CLOSE_ALL_ICON, HOME_DIR_CONFIG_PATH,
    NOT_FOUND_ICON, USR_SHARE_PATH,
};
use crate::errors::{AppSwitcherError, AppSwitcherErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # App Switcher Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the App Switcher,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct AppSwitcherSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub modules: Modules,
    pub exclude_apps: Vec<String>,
}

impl Default for AppSwitcherSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("App Switcher"),
            modules: Modules::default(),
            exclude_apps: vec![APP_NAMESPACE.to_string()],
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
            id: Some(String::from("app-switcher")),
            text_multithreading: false,
            antialiasing: false,
            try_opengles_first: true,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WindowSize {
    pub default: (i32, i32),
    pub minimized: (u32, u32),
    pub maximized: (u32, u32),
    pub other: (u32, u32),
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
/// the layout of options in the App Switcher.
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct LayoutSettings {
    pub left: Vec<String>,   //Items that will in left side of App Switcher
    pub center: Vec<String>, //Items that will in center of App Switcher
    pub right: Vec<String>,  //Items that will in right side of App Switcher
}

/// # Modules

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Back {
    pub icon: String,
}
impl Default for Back {
    fn default() -> Self {
        Self {
            icon: BECK_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct CloseAll {
    pub icon: String,
}
impl Default for CloseAll {
    fn default() -> Self {
        Self {
            icon: CLOSE_ALL_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct NotFound {
    pub icon: NotFoundIconPaths,
}
impl Default for NotFound {
    fn default() -> Self {
        Self {
            icon: NotFoundIconPaths::default(),
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct NotFoundIconPaths {
    pub default: String,
    pub small: String,
}
impl Default for NotFoundIconPaths {
    fn default() -> Self {
        Self {
            default: NOT_FOUND_ICON.to_owned(),
            small: NOT_FOUND_ICON.to_owned(),
        }
    }
}

/// # Modules
///
/// Options that will be visible in app switcher
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Modules {
    pub back: Back,
    pub close_all: CloseAll,
    pub not_found: NotFound,
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
            back: Back::default(),
            close_all: CloseAll::default(),
            not_found: NotFound::default(),
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
    println!("CHECKING PATH {} EXIST ===>  {:?}", path, path_buf.is_file());
    if path_buf.is_file() {
        Some(path_buf)
    } else {
        None
    }
}

fn find_config_path() -> Option<PathBuf> {
    // from env
    if let Ok(env_path) = std::env::var("MECHANIX_APP_SWITCHER_SETTINGS_PATH") {
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
/// Reads the `settings.yml` and parsers to AppSwitcherSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<AppSwitcherSettings> {
    let file_path = find_config_path();

    if file_path.is_none() {
        bail!(AppSwitcherError::new(
            AppSwitcherErrorCodes::SettingsReadError,
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
            bail!(AppSwitcherError::new(
                AppSwitcherErrorCodes::SettingsReadError,
                format!("cannot read the settings.yml in the path - {}", e),
            ));
        }
    };

    // read and parse
    let config: AppSwitcherSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(AppSwitcherError::new(
                AppSwitcherErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e),
            ));
        }
    };

    Ok(config)
}
