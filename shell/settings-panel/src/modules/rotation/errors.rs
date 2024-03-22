use std::fmt;

use tracing::error;

/// # battery module Error Codes
///
/// Implements standard errors for the battery module
#[derive(Debug, Default, Clone, Copy)]
pub enum RotationServiceErrorCodes {
    #[default]
    UnknownError,
    CreateRotationControllerError,
    GetRotationStatusError,
}

impl fmt::Display for RotationServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotationServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            RotationServiceErrorCodes::CreateRotationControllerError => {
                write!(f, "CreateRotationControllerError")
            }
            RotationServiceErrorCodes::GetRotationStatusError => {
                write!(f, "GetRotationStatusError")
            }
        }
    }
}

/// # RotationServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`RotationServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct RotationServiceError {
    pub code: RotationServiceErrorCodes,
    pub message: String,
}

impl RotationServiceError {
    pub fn new(code: RotationServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for RotationServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
