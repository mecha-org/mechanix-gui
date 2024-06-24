use std::fmt;

use tracing::error;

/// # Notification error Codes
///
/// Implements standard errors for the Notification
#[derive(Debug, Default, Clone, Copy)]
pub enum NotificationErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    CommandExecuteError,
}

impl fmt::Display for NotificationErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationErrorCodes::UnknownError => write!(f, "UnknownError"),
            NotificationErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            NotificationErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            NotificationErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            NotificationErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            NotificationErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
        }
    }
}

/// # NotificationError
///
/// Implements a standard error type for all Notification related errors
/// includes the error code (`NotificationErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct NotificationError {
    pub code: NotificationErrorCodes,
    pub message: String,
}

impl NotificationError {
    pub fn new(code: NotificationErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
