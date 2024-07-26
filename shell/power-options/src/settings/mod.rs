use crate::constants::{BACKGROUND_IMAGE, BASE_SETTINGS_PATH, HOME_DIR_CONFIG_PATH, LOGOUT_ICON, RESTART_ICON, SHUTDOWN_ICON, USR_SHARE_PATH};
use crate::errors::{PowerOptionsError, PowerOptionsErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Power options Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the Power options,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PowerOptionsSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub modules: Modules,
}

impl Default for PowerOptionsSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Power options"),
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
            id: Some(String::from("mechanix-power-options")),
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

/// # Modules Definitions
///
/// Options that will be visible in power options
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct ShutdownModule {
    pub icon: String,
}
impl Default for ShutdownModule {
    fn default() -> Self {
        Self { 
            icon: SHUTDOWN_ICON.to_owned(), 
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct RestartModule {
    pub icon: String,
}
impl Default for RestartModule {
    fn default() -> Self {
        Self { 
            icon: RESTART_ICON.to_owned(), 
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct LogoutModule {
    pub icon: String,
}
impl Default for LogoutModule {
    fn default() -> Self {
        Self { 
            icon: LOGOUT_ICON.to_owned(), 
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct BackgroundModule {
    pub icon: String,
}
impl Default for BackgroundModule {
    fn default() -> Self {
        Self { 
            icon: BACKGROUND_IMAGE.to_owned(),
        }
    }
}

/// # Modules
///
/// Options that will be visible in Power options
#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Modules {
    pub shutdown: ShutdownModule,
    pub restart: RestartModule,
    pub logout: LogoutModule,
    pub background: BackgroundModule,
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
            shutdown: ShutdownModule::default(),
            restart: RestartModule::default(),
            logout: LogoutModule::default(),
            background: BackgroundModule::default(),
        }
    }
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
    if let Ok(env_path) = std::env::var("MECHANIX_POWER_OPTIONS_SETTINGS_PATH") {
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
/// Reads the `settings.yml` and parsers to PowerOptionsSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<PowerOptionsSettings> {
    let file_path = find_config_path();

    if file_path.is_none() {
        bail!(PowerOptionsError::new(
            PowerOptionsErrorCodes::SettingsReadError,
            format!(
                "settings.yml path not found",
            ),
        ));
    }
     
    println!("settings file location - {:?}", file_path);

    // open file
    let settings_file_handle = match File::open(file_path.unwrap()) {
        Ok(file) => file,
        Err(e) => {
            println!("settings read error {:?}", e.to_string());
            bail!(PowerOptionsError::new(
                PowerOptionsErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: PowerOptionsSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            println!("settings parse error {:?}", e.to_string());
            bail!(PowerOptionsError::new(
                PowerOptionsErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    println!("config {:?}", config);

    Ok(config)
}
