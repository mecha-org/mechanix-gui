use std::fmt;

use tracing::error;

/// # battery module Error Codes
///
/// Implements standard errors for the battery module
#[derive(Debug, Default, Clone, Copy)]
pub enum WirelessServiceErrorCodes {
    #[default]
    UnknownError,
    GetWirelessNetworkStatusError,
    GetCurrentWirelessNetworkError,
}

impl fmt::Display for WirelessServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WirelessServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            WirelessServiceErrorCodes::GetWirelessNetworkStatusError => {
                write!(f, "GetWirelessNetworkStatusError")
            }
            WirelessServiceErrorCodes::GetCurrentWirelessNetworkError => {
                write!(f, "GetCurrentWirelessNetworkError")
            }
        }
    }
}

/// # WirelessServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`WirelessServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct WirelessServiceError {
    pub code: WirelessServiceErrorCodes,
    pub message: String,
}

impl WirelessServiceError {
    pub fn new(code: WirelessServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for WirelessServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
