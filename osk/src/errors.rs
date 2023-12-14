use std::fmt;

use tracing::error;

/// # Osk Error Codes
///
/// Implements standard errors for the Osk
#[derive(Debug, Default, Clone, Copy)]
pub enum OskErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
}

impl fmt::Display for OskErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OskErrorCodes::UnknownError => write!(f, "UnknownError"),
            OskErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            OskErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
        }
    }
}

/// # OskError
///
/// Implements a standard error type for all Osk related errors
/// includes the error code (`OskErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct OskError {
    pub code: OskErrorCodes,
    pub message: String,
}

impl OskError {
    pub fn new(code: OskErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for OskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
