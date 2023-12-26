use std::fmt;

use tracing::error;

/// # Rotation Handler Error Codes
///
/// Implements standard errors for the Rotation
#[derive(Debug, Default, Clone, Copy)]
pub enum RotationHandlerErrorCodes {
    #[default]
    UnknownError,
    UnsupportedCompositorError,
    UnknownAccelerometerDeviceError,
    GetCurrentOrientationError,
    GetOldTransformStateError,
}

impl fmt::Display for RotationHandlerErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotationHandlerErrorCodes::UnknownError => write!(f, "UnknownError"),
            RotationHandlerErrorCodes::UnsupportedCompositorError => {
                write!(f, "UnsupportedCompositorError")
            }
            RotationHandlerErrorCodes::UnknownAccelerometerDeviceError => {
                write!(f, "UnknownAccelerometerDeviceError")
            }
            RotationHandlerErrorCodes::GetCurrentOrientationError => {
                write!(f, "GetCurrentOrientationError")
            }
            RotationHandlerErrorCodes::GetOldTransformStateError => {
                write!(f, "GetOldTransformStateError")
            }
        }
    }
}

/// # RotationHandlerError
///
/// Implements a standard error type for all Rotation related errors
/// includes the error code (`RotationHandlerErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct RotationHandlerError {
    pub code: RotationHandlerErrorCodes,
    pub message: String,
}

impl RotationHandlerError {
    pub fn new(code: RotationHandlerErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for RotationHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
