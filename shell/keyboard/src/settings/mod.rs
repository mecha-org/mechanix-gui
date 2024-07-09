use crate::errors::{KeyboardError, KeyboardErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Keyboard Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the keyboard,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct KeyboardSettings {
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub title: String,          // Sets the window title
    pub icons: Icons,
    pub layouts: Layouts,
    pub trie: TrieConfigs,
}

impl Default for KeyboardSettings {
    fn default() -> Self {
        Self {
            app: AppSettings::default(),
            window: WindowSettings::default(),
            title: String::from("keyboard"),
            icons: Icons::default(),
            layouts: Layouts::default(),
            trie: TrieConfigs::default(),
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
            id: Some(String::from("keyboard")),
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

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DefaultIconPaths {
    pub default: Option<String>,
}

/// # Modules
///
/// Options that will be visible in keyboard
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Modules {}

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

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Icons {
    pub backspace: Option<String>,
    pub enter: Option<String>,
    pub shift: Option<String>,
    pub symbolic: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Layouts {
    pub default: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct TrieConfigs {
    pub raw_file: Option<String>,
    pub cached_file: Option<String>,
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
/// Reads the `settings.yml` and parsers to KeyboardSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<KeyboardSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHANIX_KEYBOARD_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
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
            bail!(KeyboardError::new(
                KeyboardErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: KeyboardSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(KeyboardError::new(
                KeyboardErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
