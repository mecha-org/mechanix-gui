use std::fmt;

use tracing::error;

/// # App Drawer Error Codes
///
/// Implements standard errors for the app drawer
#[derive(Debug, Default, Clone, Copy)]
pub enum AppDrawerErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    CommandExecuteError,
}

impl fmt::Display for AppDrawerErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppDrawerErrorCodes::UnknownError => write!(f, "UnknownError"),
            AppDrawerErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            AppDrawerErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            AppDrawerErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            AppDrawerErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            AppDrawerErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
        }
    }
}

/// # AppDrawerError
///
/// Implements a standard error type for all app drawer related errors
/// includes the error code (`AppDrawerErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct AppDrawerError {
    pub code: AppDrawerErrorCodes,
    pub message: String,
}

impl AppDrawerError {
    pub fn new(code: AppDrawerErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AppDrawerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
