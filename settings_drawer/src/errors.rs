use std::fmt;

use tracing::error;

/// # Settings drawer Error Codes
/// 
/// Implements standard errors for the Settings drawer
#[derive(Debug, Default, Clone, Copy)]
pub enum SettingsDrawerErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
}

impl fmt::Display for SettingsDrawerErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingsDrawerErrorCodes::UnknownError => write!(f, "UnknownError"),
            SettingsDrawerErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            SettingsDrawerErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            SettingsDrawerErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            SettingsDrawerErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
        }
    }
}

/// # SettingsDrawerError
/// 
/// Implements a standard error type for all Settings drawer related errors
/// includes the error code (`SettingsDrawerErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct SettingsDrawerError {
    pub code: SettingsDrawerErrorCodes,
    pub message: String,
}

impl SettingsDrawerError {
    pub fn new(code: SettingsDrawerErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message,
        }
    }
}

impl std::fmt::Display for SettingsDrawerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}