use std::fmt;

use tracing::error;

/// # Error Codes
///
/// Implements standard errors
#[derive(Debug, Default, Clone, Copy)]
pub enum MemoryServiceErrorCodes {
    #[default]
    UnknownError,
    CreateMemoryControllerError,
    GetMemoryStatusError,
}

impl fmt::Display for MemoryServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemoryServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            MemoryServiceErrorCodes::CreateMemoryControllerError => {
                write!(f, "CreateMemoryControllerError")
            }
            MemoryServiceErrorCodes::GetMemoryStatusError => {
                write!(f, "GetMemoryStatusError")
            }
        }
    }
}

/// # MemoryServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`MemoryServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct MemoryServiceError {
    pub code: MemoryServiceErrorCodes,
    pub message: String,
}

impl MemoryServiceError {
    pub fn new(code: MemoryServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for MemoryServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
