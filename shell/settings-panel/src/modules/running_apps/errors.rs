use std::fmt;

use tracing::error;

/// # Error Codes
///
/// Implements standard errors
#[derive(Debug, Default, Clone, Copy)]
pub enum RunningAppsServiceErrorCodes {
    #[default]
    UnknownError,
    CreateRunningAppsControllerError,
    GetRunningAppsStatusError,
}

impl fmt::Display for RunningAppsServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunningAppsServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            RunningAppsServiceErrorCodes::CreateRunningAppsControllerError => {
                write!(f, "CreateRunningAppsControllerError")
            }
            RunningAppsServiceErrorCodes::GetRunningAppsStatusError => {
                write!(f, "GetRunningAppsStatusError")
            }
        }
    }
}

/// # RunningAppsServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`RunningAppsServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct RunningAppsServiceError {
    pub code: RunningAppsServiceErrorCodes,
    pub message: String,
}

impl RunningAppsServiceError {
    pub fn new(code: RunningAppsServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for RunningAppsServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
