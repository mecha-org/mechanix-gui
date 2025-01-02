use crate::constants::{self, BASE_SETTINGS_PATH, HOME_DIR_CONFIG_PATH};
use crate::errors::{SettingsAppError, SettingsAppErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the launcher,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MainSettings {
    pub title: String,
    pub app: AppSettings,
    pub window: WindowSettings, // Window Settings
    pub fonts: CustomFonts,
    pub icons: Icons,
}

impl Default for MainSettings {
    fn default() -> Self {
        Self {
            title: String::from("Files"),
            app: AppSettings::default(),
            window: WindowSettings::default(),
            fonts: CustomFonts::default(),
            icons: Icons::default(),
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
pub struct CustomFonts {
    pub paths: Vec<String>,
}
impl Default for CustomFonts {
    fn default() -> Self {
        Self { paths: vec![] }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Icons {
    pub fold_icon: String,
    pub file_icon: String,
    pub arrow_icon: String,
    pub back_icon: String,
    pub add_icon: String,
    pub dots_icon: String,
    pub pdf_icon: String,
    pub img_icon: String,
    pub unfold_dir_icon: String,
}
impl Default for Icons {
    fn default() -> Self {
        Self {
            fold_icon: String::from(constants::FOLD_ICON),
            file_icon: String::from(constants::FILE_ICON),
            arrow_icon: String::from(constants::ARROW_ICON),
            back_icon: String::from(constants::BACK_ICON),
            add_icon: String::from(constants::ADD_ICON),
            dots_icon: String::from(constants::DOTS_ICON),
            pdf_icon: String::from(constants::PDF_ICON),
            img_icon: String::from(constants::IMG_ICON),
            unfold_dir_icon: String::from(constants::UNFOLD_ICON),
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
            id: Some(String::from(constants::APP_ID)),
            text_multithreading: false,
            antialiasing: false,
            try_opengles_first: true,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(default)]
pub struct SecuredWirelessIconPaths {
    pub off: String,
    pub on: String,
    pub low: String,
    pub weak: String,
    pub strong: String,
    pub error: String,
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

    None
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
