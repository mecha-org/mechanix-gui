use std::fmt;

use tracing::error;

/// # Status bar Error Codes
///
/// Implements standard errors for the status bar
#[derive(Debug, Default, Clone, Copy)]
pub enum StatusBarErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    InitNetworkManagerClient,
    InitBluetoothManagerClient,
    InitBatteryManagerClient,
    GetWirelessStatusError,
    GetBluetoothStatusError,
    GetBatteryStatusError,
    GetBatteryError,
}

impl fmt::Display for StatusBarErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusBarErrorCodes::UnknownError => write!(f, "UnknownError"),
            StatusBarErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            StatusBarErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            StatusBarErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            StatusBarErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            StatusBarErrorCodes::InitNetworkManagerClient => write!(f, "InitNetworkManagerClient"),
            StatusBarErrorCodes::InitBluetoothManagerClient => {
                write!(f, "InitBluetoothManagerClient")
            }
            StatusBarErrorCodes::InitBatteryManagerClient => write!(f, "InitBatteryManagerClient"),
            StatusBarErrorCodes::GetWirelessStatusError => write!(f, "GetWirelessStatusError"),
            StatusBarErrorCodes::GetBluetoothStatusError => write!(f, "GetBluetoothStatusError"),
            StatusBarErrorCodes::GetBatteryStatusError => write!(f, "GetBatteryStatusError"),
            StatusBarErrorCodes::GetBatteryError => write!(f, "GetBatteryError"),
        }
    }
}

/// # StatusBarError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`StatusBarErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct StatusBarError {
    pub code: StatusBarErrorCodes,
    pub message: String,
}

impl StatusBarError {
    pub fn new<S: Into<String> + Clone>(
        code: StatusBarErrorCodes,
        message: S,
        _capture_error: bool,
    ) -> Self {
        error!(
            "Error: (code: {:?}, message: {})",
            code,
            message.clone().into()
        );
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for StatusBarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
