use std::fmt;

use tracing::error;

/// # ActionBar Error Codes
///
/// Implements standard errors for the action bar
#[derive(Debug, Default, Clone, Copy)]
pub enum ActionBarErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    CommandExecuteError,
    CommandExecuteOutputError,
}

impl fmt::Display for ActionBarErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionBarErrorCodes::UnknownError => write!(f, "UnknownError"),
            ActionBarErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            ActionBarErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            ActionBarErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            ActionBarErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            ActionBarErrorCodes::CommandExecuteError => {
                write!(f, "CommandExecuteError")
            }
            ActionBarErrorCodes::CommandExecuteOutputError => {
                write!(f, "CommandExecuteOutputError")
            }
        }
    }
}

/// # ActionBarError
///
/// Implements a standard error type for all action bar related errors
/// includes the error code (`ActionBarErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct ActionBarError {
    pub code: ActionBarErrorCodes,
    pub message: String,
}

impl ActionBarError {
    pub fn new(code: ActionBarErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ActionBarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
