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
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct ShutdownModule {
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct RestartModule {
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct LogoutModule {
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct BackgroundModule {
    pub icon: Option<String>,
}

/// # Modules
///
/// Options that will be visible in Power options
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {
    pub shutdown: ShutdownModule,
    pub restart: RestartModule,
    pub logout: LogoutModule,
    pub background: BackgroundModule,
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
            shutdown: ShutdownModule::default(),
            restart: RestartModule::default(),
            logout: LogoutModule::default(),
            background: BackgroundModule::default(),
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
/// Reads the `settings.yml` and parsers to PowerOptionsSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<PowerOptionsSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_POWER_OPTIONS_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
    ); // Get path of the library

    // read from args
    let file_path_in_args = read_settings_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap());
    }

    println!("settings file location - {:?}", file_path);

    // open file
    let settings_file_handle = match File::open(file_path) {
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
