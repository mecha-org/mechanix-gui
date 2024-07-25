use crate::constants::{BASE_SETTINGS_PATH, HOME_DIR_CONFIG_PATH, USR_SHARE_PATH};
use crate::errors::{StatusBarError, StatusBarErrorCodes};
use anyhow::bail;
use anyhow::Result;
use mechanix_status_bar_components::settings::Layout;
use mechanix_status_bar_components::settings::Modules;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    pub layout: Layout,
    pub modules: Modules,
    pub css: CssConfigs,
    pub fonts: HashMap<String, String>,
}

impl Default for StatusBarSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Status Bar"),
            layout: Layout::default(),
            modules: Modules::default(),
            css: CssConfigs::default(),
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
    pub height: Option<i32>,          // height of the window
    pub width: Option<i32>,           // width of the window
    pub position: (i32, i32),         // Default position to start window
    pub min_size: Option<(u32, u32)>, // Minimum size the window can be resized to
    pub max_size: Option<(u32, u32)>, // Maximum size the window can be resized to
    pub visible: bool,                // Sets visibility of the window
    pub resizable: bool,              // Enables or disables resizing
    pub decorations: bool,            // Enables or disables the title bar
    pub transparent: bool,            // Enables transparency
    pub always_on_top: bool,          // Forces window to be always on top
    pub icon_path: Option<String>,
    pub layer_shell: WindowLayerShellSettings,
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
pub struct WindowLayerShellSettings {
    pub margin: LayerShellMargin,
    pub anchor: LayerShellAnchor,
}

impl Default for WindowLayerShellSettings {
    fn default() -> Self {
        Self {
            margin: LayerShellMargin::default(),
            anchor: LayerShellAnchor::default(),
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayerShellMargin {
    pub top: Option<i32>,
    pub right: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
}

impl Default for LayerShellMargin {
    fn default() -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayerShellAnchor {
    pub top: Option<bool>,
    pub right: Option<bool>,
    pub bottom: Option<bool>,
    pub left: Option<bool>,
}

impl Default for LayerShellAnchor {
    fn default() -> Self {
        Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
        }
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            height: None,
            width: None,
            position: (0, 0),
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon_path: None,
            layer_shell: WindowLayerShellSettings::default(),
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
    if let Ok(env_path) = std::env::var("MECHA_STATUS_BAR_SETTINGS_PATH") {
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
/// Reads the `settings.yml` and parsers to StatusBarSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<StatusBarSettings> {
    let file_path = find_config_path().unwrap();

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
