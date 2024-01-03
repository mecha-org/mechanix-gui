use std::fmt;

use tracing::error;

/// # homescreen error Codes
///
/// Implements standard errors for the homescreen
#[derive(Debug, Default, Clone, Copy)]
pub enum DesktopIniErrorCodes {
    #[default]
    UnknownError,
    DirectoryReadError,
    FileReadError,
    ParseInfoError,
}

impl fmt::Display for DesktopIniErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DesktopIniErrorCodes::UnknownError => write!(f, "UnknownError"),
            DesktopIniErrorCodes::DirectoryReadError => write!(f, "DirectoryReadError"),
            DesktopIniErrorCodes::FileReadError => write!(f, "FileReadError"),
            DesktopIniErrorCodes::ParseInfoError => write!(f, "ParseInfoError"),
        }
    }
}

/// # DesktopIniError
///
/// Implements a standard error type for all homescreen related errors
/// includes the error code (`DesktopIniErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct DesktopIniError {
    pub code: DesktopIniErrorCodes,
    pub message: String,
}

impl DesktopIniError {
    pub fn new(code: DesktopIniErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for DesktopIniError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
