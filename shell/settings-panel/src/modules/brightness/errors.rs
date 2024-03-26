use std::fmt;

use tracing::error;

/// # Error Codes
///
/// Implements standard errors
#[derive(Debug, Default, Clone, Copy)]
pub enum BrightnessServiceErrorCodes {
    #[default]
    UnknownError,
    CreateBrightnessControllerError,
    GetBrightnessStatusError,
}

impl fmt::Display for BrightnessServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrightnessServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            BrightnessServiceErrorCodes::CreateBrightnessControllerError => {
                write!(f, "CreateBrightnessControllerError")
            }
            BrightnessServiceErrorCodes::GetBrightnessStatusError => {
                write!(f, "GetBrightnessStatusError")
            }
        }
    }
}

/// # BrightnessServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`BrightnessServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct BrightnessServiceError {
    pub code: BrightnessServiceErrorCodes,
    pub message: String,
}

impl BrightnessServiceError {
    pub fn new(code: BrightnessServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for BrightnessServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
