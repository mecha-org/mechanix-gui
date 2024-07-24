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

fn is_empty_path(path: &PathBuf) -> bool {
    path.as_os_str().is_empty()
}

/// # Reads Settings YML
///
/// Reads the `settings.yml` and parsers to StatusBarSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<StatusBarSettings> {
    // 1. args
    // 1. env
    // 3. .config/mechanix
    // 4. /usr/share

    // dir - settings.yml ? 4 ?

    let mut file_path = PathBuf::from(
        std::env::var("MECHA_STATUS_BAR_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
    ); // Get path of the library

    // read from args
    let file_path_in_args = read_settings_path_from_args();
    if file_path_in_args.is_some() {
        file_path = PathBuf::from(file_path_in_args.unwrap()); // this - 3
    }

    if is_empty_path(&file_path) {
        let home_dir = dirs::home_dir().unwrap();
        file_path = home_dir.join(".config/mechanix/status-bar/settings.yml");
    }

    println!("2 =======> CHECKING file_path : {:?} ", &file_path);

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
