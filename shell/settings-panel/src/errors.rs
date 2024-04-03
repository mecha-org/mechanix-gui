use std::fmt;

use tracing::error;

/// # Settings panel Error Codes
///
/// Implements standard errors for the Settings panel
#[derive(Debug, Default, Clone, Copy)]
pub enum SettingsPanelErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
}

impl fmt::Display for SettingsPanelErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingsPanelErrorCodes::UnknownError => write!(f, "UnknownError"),
            SettingsPanelErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            SettingsPanelErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            SettingsPanelErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            SettingsPanelErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
        }
    }
}

/// # SettingsPanelError
///
/// Implements a standard error type for all Settings panel related errors
/// includes the error code (`SettingsPanelErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct SettingsPanelError {
    pub code: SettingsPanelErrorCodes,
    pub message: String,
}

impl SettingsPanelError {
    pub fn new(code: SettingsPanelErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for SettingsPanelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
