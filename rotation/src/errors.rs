use std::fmt;

use tracing::error;

/// # Rotation Error Codes
///
/// Implements standard errors for the Rotation
#[derive(Debug, Default, Clone, Copy)]
pub enum RotationErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    CommandExecuteError,
    SettingsSerializeError,
    SettingsWriteError,
}

impl fmt::Display for RotationErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotationErrorCodes::UnknownError => write!(f, "UnknownError"),
            RotationErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            RotationErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            RotationErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
            RotationErrorCodes::SettingsSerializeError => write!(f, "SettingsSerializeError"),
            RotationErrorCodes::SettingsWriteError => write!(f, "SettingsWriteError"),
        }
    }
}

/// # RotationError
///
/// Implements a standard error type for all Rotation related errors
/// includes the error code (`RotationErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct RotationError {
    pub code: RotationErrorCodes,
    pub message: String,
}

impl RotationError {
    pub fn new(code: RotationErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for RotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
