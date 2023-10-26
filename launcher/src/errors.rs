use std::fmt;

use tracing::error;

/// # App Drawer Error Codes
/// 
/// Implements standard errors for the app drawer
#[derive(Debug, Default, Clone, Copy)]
pub enum LauncherErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
}

impl fmt::Display for LauncherErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LauncherErrorCodes::UnknownError => write!(f, "UnknownError"),
            LauncherErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            LauncherErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            LauncherErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            LauncherErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
        }
    }
}

/// # LauncherError
/// 
/// Implements a standard error type for all app drawer related errors
/// includes the error code (`LauncherErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct LauncherError {
    pub code: LauncherErrorCodes,
    pub message: String,
}

impl LauncherError {
    pub fn new(code: LauncherErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for LauncherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}