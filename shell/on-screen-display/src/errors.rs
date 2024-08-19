use std::fmt;

use tracing::error;

/// # OnScreenDisplay error Codes
///
/// Implements standard errors for the OnScreenDisplay
#[derive(Debug, Default, Clone, Copy)]
pub enum OnScreenDisplayErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    CommandExecuteError,
    GetWirelessStatusError,
    GetBluetoothStatusError,
    GetBatteryStatusError,
    GetBatteryError,
}

impl fmt::Display for OnScreenDisplayErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OnScreenDisplayErrorCodes::UnknownError => write!(f, "UnknownError"),
            OnScreenDisplayErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            OnScreenDisplayErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            OnScreenDisplayErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            OnScreenDisplayErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            OnScreenDisplayErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
            OnScreenDisplayErrorCodes::GetWirelessStatusError => {
                write!(f, "GetWirelessStatusError")
            }
            OnScreenDisplayErrorCodes::GetBluetoothStatusError => {
                write!(f, "GetBluetoothStatusError")
            }
            OnScreenDisplayErrorCodes::GetBatteryStatusError => write!(f, "GetBatteryStatusError"),
            OnScreenDisplayErrorCodes::GetBatteryError => write!(f, "GetBatteryError"),
        }
    }
}

/// # OnScreenDisplayError
///
/// Implements a standard error type for all OnScreenDisplay related errors
/// includes the error code (`OnScreenDisplayErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct OnScreenDisplayError {
    pub code: OnScreenDisplayErrorCodes,
    pub message: String,
}

impl OnScreenDisplayError {
    pub fn new(code: OnScreenDisplayErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for OnScreenDisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
