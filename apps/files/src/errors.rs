use std::fmt;

use tracing::error;

/// # App Drawer Error Codes
///
/// Implements standard errors for the app drawer
#[derive(Debug, Default, Clone, Copy)]
pub enum FilesAppErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
}

impl fmt::Display for FilesAppErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FilesAppErrorCodes::UnknownError => write!(f, "UnknownError"),
            FilesAppErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            FilesAppErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
        }
    }
}

/// # SettingsAppError
///
/// Implements a standard error type for all app drawer related errors
/// includes the error code (`FilesAppErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct FilesAppError {
    pub code: FilesAppErrorCodes,
    pub message: String,
}

impl FilesAppError {
    pub fn new(code: FilesAppErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for FilesAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
