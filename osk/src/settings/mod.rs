use crate::errors::{OskError, OskErrorCodes};
use anyhow::bail;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

/// # Osk Settings
///
/// Struct representing the settings.yml configuration file,
/// this file lets you control the behavior of the osk,
/// apply custom theme and fonts
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct OskSettings {
    pub bins: Bins,
}

impl Default for OskSettings {
    fn default() -> Self {
        Self {
            bins: Bins::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Bin {
    pub kind: Option<String>,
    pub bin: Option<String>,
    pub conf: Option<String>,
}

impl Default for Bin {
    fn default() -> Self {
        Self {
            kind: None,
            bin: None,
            conf: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Bins {
    pub keyboard: Bin,
}
impl Default for Bins {
    fn default() -> Self {
        Self {
            keyboard: Bin::default(),
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
/// Reads the `settings.yml` and parsers to OskSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<OskSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHA_OSK_SETTINGS_PATH").unwrap_or(String::from("settings.yml")),
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
            bail!(OskError::new(
                OskErrorCodes::SettingsReadError,
                format!("Cannot read the settings.yml in the path - {}", e),
                true
            ));
        }
    };

    // read and parse
    let config: OskSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(OskError::new(
                OskErrorCodes::SettingsParseError,
                format!("Error parsing the settings.yml - {}", e),
                true
            ));
        }
    };

    info!("settings read is {:?}", config);

    Ok(config)
}
