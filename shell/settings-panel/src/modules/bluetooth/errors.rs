use std::fmt;

use tracing::error;

/// # battery module Error Codes
///
/// Implements standard errors for the battery module
#[derive(Debug, Default, Clone, Copy)]
pub enum BluetoothServiceErrorCodes {
    #[default]
    UnknownError,
    CreateBluetoothControllerError,
    GetBluetoothStatusError,
}

impl fmt::Display for BluetoothServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BluetoothServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            BluetoothServiceErrorCodes::CreateBluetoothControllerError => {
                write!(f, "CreateBluetoothControllerError")
            }
            BluetoothServiceErrorCodes::GetBluetoothStatusError => {
                write!(f, "GetBluetoothStatusError")
            }
        }
    }
}

/// # BluetoothServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`BluetoothServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct BluetoothServiceError {
    pub code: BluetoothServiceErrorCodes,
    pub message: String,
}

impl BluetoothServiceError {
    pub fn new(code: BluetoothServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for BluetoothServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
