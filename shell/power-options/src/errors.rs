use std::fmt;

use tracing::error;

/// # Power options error Codes
///
/// Implements standard errors for the Power options
#[derive(Debug, Default, Clone, Copy)]
pub enum PowerOptionsErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    Shutdown,
    Restart,
    Logout,
}

impl fmt::Display for PowerOptionsErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PowerOptionsErrorCodes::UnknownError => write!(f, "UnknownError"),
            PowerOptionsErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            PowerOptionsErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            PowerOptionsErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            PowerOptionsErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            PowerOptionsErrorCodes::Shutdown => write!(f, "Shutdown"),
            PowerOptionsErrorCodes::Restart => write!(f, "Restart"),
            PowerOptionsErrorCodes::Logout => write!(f, "Logout"),
        }
    }
}

/// # PowerOptionsError
///
/// Implements a standard error type for all Power options related errors
/// includes the error code (`PowerOptionsErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct PowerOptionsError {
    pub code: PowerOptionsErrorCodes,
    pub message: String,
}

impl PowerOptionsError {
    pub fn new(code: PowerOptionsErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for PowerOptionsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
