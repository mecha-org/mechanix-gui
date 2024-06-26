use std::fmt;

use tracing::error;

/// # Keyboard error Codes
///
/// Implements standard errors for the Keyboard
#[derive(Debug, Default, Clone, Copy)]
pub enum KeyboardErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    CommandExecuteError,
}

impl fmt::Display for KeyboardErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyboardErrorCodes::UnknownError => write!(f, "UnknownError"),
            KeyboardErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            KeyboardErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            KeyboardErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            KeyboardErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            KeyboardErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
        }
    }
}

/// # KeyboardError
///
/// Implements a standard error type for all Keyboard related errors
/// includes the error code (`KeyboardErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct KeyboardError {
    pub code: KeyboardErrorCodes,
    pub message: String,
}

impl KeyboardError {
    pub fn new(code: KeyboardErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for KeyboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
