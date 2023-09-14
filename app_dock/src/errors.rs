use std::fmt;

use tracing::error;

/// # App dock Error Codes
/// 
/// Implements standard errors for the App dock
#[derive(Debug, Default, Clone, Copy)]
pub enum AppDockErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
}

impl fmt::Display for AppDockErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppDockErrorCodes::UnknownError => write!(f, "UnknownError"),
            AppDockErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            AppDockErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            AppDockErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            AppDockErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
        }
    }
}

/// # AppDockError
/// 
/// Implements a standard error type for all App dock related errors
/// includes the error code (`AppDockErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct AppDockError {
    pub code: AppDockErrorCodes,
    pub message: String,
}

impl AppDockError {
    pub fn new(code: AppDockErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message,
        }
    }
}

impl std::fmt::Display for AppDockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}