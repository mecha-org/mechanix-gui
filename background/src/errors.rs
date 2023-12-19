use std::fmt;

use tracing::error;

/// # Background Error Codes
///
/// Implements standard errors for the Background
#[derive(Debug, Default, Clone, Copy)]
pub enum BackgroundErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    CommandExecuteError,
}

impl fmt::Display for BackgroundErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BackgroundErrorCodes::UnknownError => write!(f, "UnknownError"),
            BackgroundErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            BackgroundErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            BackgroundErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
        }
    }
}

/// # BackgroundError
///
/// Implements a standard error type for all Background related errors
/// includes the error code (`BackgroundErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct BackgroundError {
    pub code: BackgroundErrorCodes,
    pub message: String,
}

impl BackgroundError {
    pub fn new(code: BackgroundErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for BackgroundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
