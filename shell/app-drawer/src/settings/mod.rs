use crate::constants::{
    BACK_ICON, BASE_SETTINGS_PATH, CLEAR_ICON, HOME_DIR_CONFIG_PATH, HOME_ICON, NOT_FOUND_ICON, NOT_FOUND_SMALL_ICON, SEARCH_ICON, USR_SHARE_PATH
};
use crate::errors::{AppDrawerError, AppDrawerErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # App Drawer Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the app drawer,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppDrawerSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
}

impl Default for AppDrawerSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("App Drawer"),
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
            id: Some(String::from("app-drawer")),
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
#[serde(default)]
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
/// the layout of options in the app drawer.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayoutSettings {
    pub left: Vec<String>,   //Items that will in left side of app drawer
    pub center: Vec<String>, //Items that will in center of app drawer
    pub right: Vec<String>,  //Items that will in right side of app drawer
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
pub struct SearchModule {
    pub icon: SearchIconPath,
}
impl Default for SearchModule {
    fn default() -> Self {
        Self {
            icon: SearchIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct HomeModule {
    pub icon: HomeIconPath,
}
impl Default for HomeModule {
    fn default() -> Self {
        Self {
            icon: HomeIconPath::default(),
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
pub struct ClearModule {
    pub icon: ClearIconPath,
}
impl Default for ClearModule {
    fn default() -> Self {
        Self {
            icon: ClearIconPath::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct NotFoundModule {
    pub icon: NotFoundIconPaths,
}
impl Default for NotFoundModule {
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
            small: NOT_FOUND_SMALL_ICON.to_owned(),
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
pub struct ClearIconPath {
    pub default: String,
}
impl Default for ClearIconPath {
    fn default() -> Self {
        ClearIconPath {
            default: CLEAR_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct SearchIconPath {
    pub default: String,
}
impl Default for SearchIconPath {
    fn default() -> Self {
        SearchIconPath {
            default: SEARCH_ICON.to_owned(),
        }
    }
}

/// # Modules
///
/// Options that will be visible in app drawer
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Modules {
    pub home: HomeModule,
    pub back: BackModule,
    pub search: SearchModule,
    pub clear: ClearModule,
    pub not_found: NotFoundModule,
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

impl Default for Modules {
    fn default() -> Self {
        Self {
            search: SearchModule::default(),
            home: HomeModule::default(),
            back: BackModule::default(),
            clear: ClearModule::default(),
            not_found: NotFoundModule::default(),
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
    if let Ok(env_path) = std::env::var("MECHANIX_APP_DRAWER_SETTINGS_PATH") {
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
/// Reads the `settings.yml` and parsers to AppDrawerSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<AppDrawerSettings> {
    let file_path = find_config_path().unwrap();

    info!(
        task = "read_settings",
        "settings file location - {:?}", file_path
    );

    // open file
    let settings_file_handle = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            bail!(AppDrawerError::new(
                AppDrawerErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: AppDrawerSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(AppDrawerError::new(
                AppDrawerErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
