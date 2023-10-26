use crate::errors::{LauncherError, LauncherErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # App Manager Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the app manager,
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LauncherSettings {
    pub components: Components,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        Self {
            components: Components::default(),
        }
    }
}


#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Components {
    pub status_bar: Component,
    pub app_drawer: Component,
    pub settings_panel: Component,
    pub action_bar: Component,
    pub app_dock: Component,
    pub lock_screen: Component,
    pub app_widget: Component,
}

impl Default for Components {
    fn default() -> Self {
        Self {
            status_bar: Component::default(),
            app_drawer: Component::default(),
            settings_panel: Component::default(),
            action_bar: Component::default(),
            app_dock: Component::default(),
            lock_screen: Component::default(),
            app_widget: Component::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Component {
    pub enabled: bool,
    pub bin_path: String,
    pub settings_path: String
}


impl Default for Component {
    fn default() -> Self {
        Self {
            enabled: false,
            bin_path: "".to_string(),
            settings_path: "".to_string()
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
/// Reads the `settings.yml` and parsers to LauncherSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<LauncherSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_APP_MANAGER_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
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
            bail!(LauncherError::new(
                LauncherErrorCodes::SettingsReadError,
                format!(
                    "cannot read the settings.yml in the path - {}",
                    e.to_string()
                ),
            ));
        }
    };

    // read and parse
    let config: LauncherSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(LauncherError::new(
                LauncherErrorCodes::SettingsParseError,
                format!("error parsing the settings.yml - {}", e.to_string()),
            ));
        }
    };

    Ok(config)
}
