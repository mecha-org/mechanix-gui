use crate::constants::{
    BASE_SETTINGS_PATH, EDIT_CLEAR_ICON, HOME_DIR_CONFIG_PATH, KEYBOARD_MODE_ICON, KEY_ENTER_ICON, KEY_SHIFT_ICON, LAYOUT_EXAMPLE_PATH, TRIE_CACHED_FILE, TRIE_RAW_FILE, USR_SHARE_PATH
};
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
#[serde(default)]
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
#[serde(default)]
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

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Icons {
    pub backspace: String,
    pub enter: String,
    pub shift: String,
    pub symbolic: String,
}
impl Default for Icons {
    fn default() -> Self {
        Self {
            backspace: EDIT_CLEAR_ICON.to_owned(),
            enter: KEY_SHIFT_ICON.to_owned(),
            shift: KEY_ENTER_ICON.to_owned(),
            symbolic: KEYBOARD_MODE_ICON.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct Layouts {
    pub default: String,
}
impl Default for Layouts {
    fn default() -> Self {
        Self {
            default: LAYOUT_EXAMPLE_PATH.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(default)]
pub struct TrieConfigs {
    pub raw_file: Option<String>,
    pub cached_file: Option<String>,
}
impl Default for TrieConfigs {
    fn default() -> Self {
        Self {
            raw_file: Some(TRIE_RAW_FILE.to_owned()),
            cached_file: Some(TRIE_CACHED_FILE.to_owned()),
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
    if let Ok(env_path) = std::env::var("MECHANIX_KEYBOARD_SETTINGS_PATH") {
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
/// Reads the `settings.yml` and parsers to KeyboardSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<KeyboardSettings> {
    let file_path = find_config_path();

    if file_path.is_none() {
        bail!(KeyboardError::new(
            KeyboardErrorCodes::SettingsReadError,
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
