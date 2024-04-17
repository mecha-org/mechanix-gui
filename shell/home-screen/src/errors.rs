use std::fmt;

use tracing::error;

/// # homescreen error Codes
///
/// Implements standard errors for the homescreen
#[derive(Debug, Default, Clone, Copy)]
pub enum HomescreenErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    CommandExecuteError,
}

impl fmt::Display for HomescreenErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HomescreenErrorCodes::UnknownError => write!(f, "UnknownError"),
            HomescreenErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            HomescreenErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            HomescreenErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            HomescreenErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            HomescreenErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
        }
    }
}

/// # HomescreenError
///
/// Implements a standard error type for all homescreen related errors
/// includes the error code (`HomescreenErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct HomescreenError {
    pub code: HomescreenErrorCodes,
    pub message: String,
}

impl HomescreenError {
    pub fn new(code: HomescreenErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for HomescreenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
