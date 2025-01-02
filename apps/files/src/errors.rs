use std::fmt;

use tracing::error;

/// # App Drawer Error Codes
///
/// Implements standard errors for the app drawer
#[derive(Debug, Default, Clone, Copy)]
pub enum SettingsAppErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
}

impl fmt::Display for SettingsAppErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingsAppErrorCodes::UnknownError => write!(f, "UnknownError"),
            SettingsAppErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            SettingsAppErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
        }
    }
}

/// # SettingsAppError
///
/// Implements a standard error type for all app drawer related errors
/// includes the error code (`SettingsAppErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct SettingsAppError {
    pub code: SettingsAppErrorCodes,
    pub message: String,
}

impl SettingsAppError {
    pub fn new(code: SettingsAppErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for SettingsAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
