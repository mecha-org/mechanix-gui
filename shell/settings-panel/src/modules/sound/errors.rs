use std::fmt;

use tracing::error;

/// # Error Codes
///
/// Implements standard errors
#[derive(Debug, Default, Clone, Copy)]
pub enum SoundServiceErrorCodes {
    #[default]
    UnknownError,
    CreateSoundControllerError,
    GetSoundStatusError,
}

impl fmt::Display for SoundServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SoundServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            SoundServiceErrorCodes::CreateSoundControllerError => {
                write!(f, "CreateSoundControllerError")
            }
            SoundServiceErrorCodes::GetSoundStatusError => {
                write!(f, "GetSoundStatusError")
            }
        }
    }
}

/// # SoundServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`SoundServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct SoundServiceError {
    pub code: SoundServiceErrorCodes,
    pub message: String,
}

impl SoundServiceError {
    pub fn new(code: SoundServiceErrorCodes, message: String, _capture_error: bool) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for SoundServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
