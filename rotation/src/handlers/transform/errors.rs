use std::fmt;

use tracing::error;

/// # Output Error Codes
///
/// Implements standard errors for the Output
#[derive(Debug, Default, Clone, Copy)]
pub enum TransformHandlerErrorCodes {
    #[default]
    UnknownError,
    OutputManagemenConnectError,
    GetOutputMetaError,
    TransformError,
    OutputNotFoundByNameError,
    RotateOutputError,
}

impl fmt::Display for TransformHandlerErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransformHandlerErrorCodes::UnknownError => write!(f, "UnknownError"),
            TransformHandlerErrorCodes::OutputManagemenConnectError => {
                write!(f, "OutputManagemenConnectError")
            }
            TransformHandlerErrorCodes::GetOutputMetaError => {
                write!(f, "GetOutputMetaError")
            }
            TransformHandlerErrorCodes::TransformError => {
                write!(f, "TransformError")
            }
            TransformHandlerErrorCodes::OutputNotFoundByNameError => {
                write!(f, "OutputNotFoundByNameError")
            }
            TransformHandlerErrorCodes::RotateOutputError => {
                write!(f, "RotateOutputError")
            }
        }
    }
}

/// # TransformHandlerError
///
/// Implements a standard error type for all Output related errors
/// includes the error code (`TransformHandlerErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct TransformHandlerError {
    pub code: TransformHandlerErrorCodes,
    pub message: String,
}

impl TransformHandlerError {
    pub fn new(code: TransformHandlerErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for TransformHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
