use std::fmt;

use tracing::error;

/// # Action bar Error Codes
/// 
/// Implements standard errors for the Action bar
#[derive(Debug, Default, Clone, Copy)]
pub enum ActionBarErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
}

impl fmt::Display for ActionBarErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionBarErrorCodes::UnknownError => write!(f, "UnknownError"),
            ActionBarErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            ActionBarErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            ActionBarErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            ActionBarErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
        }
    }
}

/// # ActionBarError
/// 
/// Implements a standard error type for all Action bar related errors
/// includes the error code (`ActionBarErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct ActionBarError {
    pub code: ActionBarErrorCodes,
    pub message: String,
}

impl ActionBarError {
    pub fn new(code: ActionBarErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message,
        }
    }
}

impl std::fmt::Display for ActionBarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}