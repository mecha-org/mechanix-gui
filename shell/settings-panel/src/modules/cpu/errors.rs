use std::fmt;

use tracing::error;

/// # Error Codes
///
/// Implements standard errors
#[derive(Debug, Default, Clone, Copy)]
pub enum CpuServiceErrorCodes {
    #[default]
    UnknownError,
    CreateCpuControllerError,
    GetCpuStatusError,
}

impl fmt::Display for CpuServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            CpuServiceErrorCodes::CreateCpuControllerError => {
                write!(f, "CreateCpuControllerError")
            }
            CpuServiceErrorCodes::GetCpuStatusError => {
                write!(f, "GetCpuStatusError")
            }
        }
    }
}

/// # CpuServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`CpuServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct CpuServiceError {
    pub code: CpuServiceErrorCodes,
    pub message: String,
}

impl CpuServiceError {
    pub fn new(code: CpuServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for CpuServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
