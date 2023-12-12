use crate::errors::{ActionBarError, ActionBarErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # ActionBar Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the action bar,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ActionBarSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub layout: LayoutSettings,
    pub modules: Modules,
    pub css: CssConfigs,
}

impl Default for ActionBarSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("Action Bar"),
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
            id: Some(String::from("action-bar")),
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
    pub layer_shell: WindowLayerShellSettings,
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

/// # Layout Settings
///
/// Part of the settings.yml to control the behavior of
/// the layout of options in the action bar.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LayoutSettings {
    pub left: Vec<String>,   //Items that will in left side of action bar
    pub center: Vec<String>, //Items that will in center of action bar
    pub right: Vec<String>,  //Items that will in right side of action bar
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

/// # Modules
///
/// App that will be visible in action bar
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct HomeModule {
    pub title: String,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SettingsModule {
    pub title: String,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct KeyboardModule {
    pub title: String,
    pub icon: Option<String>,
}

/// # Modules
///
/// Options that will be visible in dock
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {
    pub home: HomeModule,
    pub settings: SettingsModule,
    pub keyboard: KeyboardModule,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            size: (480, 70),
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
            home: HomeModule {
                title: "Home".to_string(),
                icon: None,
            },
            settings: SettingsModule {
                title: "Settings".to_string(),
                icon: None,
            },
            keyboard: KeyboardModule {
                title: "Keyboard".to_string(),
                icon: None,
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

/// # Reads Settings YML
///
/// Reads the `settings.yml` and parsers to ActionBarSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<ActionBarSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_ACTION_BAR_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
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
            bail!(ActionBarError::new(
                ActionBarErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: ActionBarSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(ActionBarError::new(
                ActionBarErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
