use home_button::HomeButtonSettings;
use idle_notify::IdleNotifySettings;
use lock_button::LockButtonSettings;

pub mod home_button;
pub mod idle_notify;
pub mod lock_button;
pub mod notifier;
pub mod session;
use anyhow::bail;
use anyhow::Result;
use notifier::NotifierSettings;
use serde::{Deserialize, Serialize};
use session::SessionSettings;
use std::{env, fs::File, path::PathBuf};
use tracing::{debug, info};

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct DesktopServerSettings {
    pub idle_notify: IdleNotifySettings,
    pub session: SessionSettings,
    pub lock_button: LockButtonSettings,
    pub home_button: HomeButtonSettings,
    pub notifier: NotifierSettings,
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
/// Reads the `settings.yml` and parsers to DesktopServerSettings
///
/// **Important**: Ensure all fields are present in the yml due to strict parsing
pub fn read_settings_yml() -> Result<DesktopServerSettings> {
    let mut file_path = PathBuf::from(
        std::env::var("MECHANIX_DESKTOP_SERVER_SETTINGS_PATH")
            .unwrap_or(String::from("settings.yml")),
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
            bail!(DesktopServerSettingsError::new(
                DesktopServerSettingsErrorCodes::SettingsReadError,
                format!("Cannot read the settings.yml in the path - {}", e),
                true
            ));
        }
    };

    // read and parse
    let config: DesktopServerSettings = match serde_yaml::from_reader(settings_file_handle) {
        Ok(config) => config,
        Err(e) => {
            bail!(DesktopServerSettingsError::new(
                DesktopServerSettingsErrorCodes::SettingsParseError,
                format!("Error parsing the settings.yml - {}", e),
                true
            ));
        }
    };

    info!("settings read is {:?}", config);

    Ok(config)
}

use std::fmt;

use tracing::error;

/// # Desktop server settings Error Codes
///
/// Implements standard errors for the desktop server settings
#[derive(Debug, Default, Clone, Copy)]
pub enum DesktopServerSettingsErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
}

impl fmt::Display for DesktopServerSettingsErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DesktopServerSettingsErrorCodes::UnknownError => write!(f, "UnknownError"),
            DesktopServerSettingsErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            DesktopServerSettingsErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
        }
    }
}

/// # DesktopServerSettingsError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`DesktopServerSettingsErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct DesktopServerSettingsError {
    pub code: DesktopServerSettingsErrorCodes,
    pub message: String,
}

impl DesktopServerSettingsError {
    pub fn new<S: Into<String> + Clone>(
        code: DesktopServerSettingsErrorCodes,
        message: S,
        _capture_error: bool,
    ) -> Self {
        error!(
            "Error: (code: {:?}, message: {})",
            code,
            message.clone().into()
        );
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for DesktopServerSettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
