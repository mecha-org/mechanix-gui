use std::fmt;

use tracing::error;

/// # battery module Error Codes
///
/// Implements standard errors for the battery module
#[derive(Debug, Default, Clone, Copy)]
pub enum BatteryServiceErrorCodes {
    #[default]
    UnknownError,
    GetBatteryInfoError,
}

impl fmt::Display for BatteryServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BatteryServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            BatteryServiceErrorCodes::GetBatteryInfoError => write!(f, "GetBatteryInfoError"),
        }
    }
}

/// # BatteryServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`BatteryServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct BatteryServiceError {
    pub code: BatteryServiceErrorCodes,
    pub message: String,
}

impl BatteryServiceError {
    pub fn new(code: BatteryServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for BatteryServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
