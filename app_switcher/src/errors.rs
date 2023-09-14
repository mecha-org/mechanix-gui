use std::fmt;

use tracing::error;

/// # App Switcher Error Codes
/// 
/// Implements standard errors for the App Switcher
#[derive(Debug, Default, Clone, Copy)]
pub enum AppSwitcherErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
}

impl fmt::Display for AppSwitcherErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppSwitcherErrorCodes::UnknownError => write!(f, "UnknownError"),
            AppSwitcherErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            AppSwitcherErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            AppSwitcherErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            AppSwitcherErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
        }
    }
}

/// # AppSwitcherError
/// 
/// Implements a standard error type for all App Switcher related errors
/// includes the error code (`AppSwitcherErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct AppSwitcherError {
    pub code: AppSwitcherErrorCodes,
    pub message: String,
}

impl AppSwitcherError {
    pub fn new(code: AppSwitcherErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AppSwitcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}