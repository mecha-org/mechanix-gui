use std::fmt;

use tracing::error;

/// # PolkitAgent error Codes
///
/// Implements standard errors for the PolkitAgent
#[derive(Debug, Default, Clone, Copy)]
pub enum PolkitAgentErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    CommandExecuteError,
}

impl fmt::Display for PolkitAgentErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PolkitAgentErrorCodes::UnknownError => write!(f, "UnknownError"),
            PolkitAgentErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            PolkitAgentErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            PolkitAgentErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            PolkitAgentErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            PolkitAgentErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
        }
    }
}

/// # PolkitAgentError
///
/// Implements a standard error type for all PolkitAgent related errors
/// includes the error code (`PolkitAgentErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct PolkitAgentError {
    pub code: PolkitAgentErrorCodes,
    pub message: String,
}

impl PolkitAgentError {
    pub fn new(code: PolkitAgentErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for PolkitAgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
