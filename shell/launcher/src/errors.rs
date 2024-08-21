use std::fmt;

use tracing::error;

/// # Launcher error Codes
///
/// Implements standard errors for the Launcher
#[derive(Debug, Default, Clone, Copy)]
pub enum LauncherErrorCodes {
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
    EnableWireless,
    DisableWireless,
    EnableBluetooth,
    DisableBluetooth,
    GetBrightnessError,
    SetBrightnessError,
    GetSoundError,
    SetSoundError,
}

impl fmt::Display for LauncherErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LauncherErrorCodes::UnknownError => write!(f, "UnknownError"),
            LauncherErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            LauncherErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            LauncherErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            LauncherErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            LauncherErrorCodes::CommandExecuteError => write!(f, "CommandExecuteError"),
            LauncherErrorCodes::GetWirelessStatusError => {
                write!(f, "GetWirelessStatusError")
            }
            LauncherErrorCodes::GetBluetoothStatusError => {
                write!(f, "GetBluetoothStatusError")
            }
            LauncherErrorCodes::GetBatteryStatusError => write!(f, "GetBatteryStatusError"),
            LauncherErrorCodes::GetBatteryError => write!(f, "GetBatteryError"),
            LauncherErrorCodes::EnableWireless => write!(f, "EnableWireless"),
            LauncherErrorCodes::DisableWireless => write!(f, "DisableWireless"),
            LauncherErrorCodes::EnableBluetooth => write!(f, "EnableBluetooth"),
            LauncherErrorCodes::DisableBluetooth => write!(f, "DisableBluetooth"),
            LauncherErrorCodes::GetBrightnessError => write!(f, "GetBrightnessError"),
            LauncherErrorCodes::SetBrightnessError => write!(f, "SetBrightnessError"),
            LauncherErrorCodes::GetSoundError => write!(f, "GetSoundError"),
            LauncherErrorCodes::SetSoundError => write!(f, "SetSoundError"),
        }
    }
}

/// # LauncherError
///
/// Implements a standard error type for all Launcher related errors
/// includes the error code (`LauncherErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct LauncherError {
    pub code: LauncherErrorCodes,
    pub message: String,
}

impl LauncherError {
    pub fn new(code: LauncherErrorCodes, message: String) -> Self {
        error!("error: (code: {:?}, message: {})", code, message);
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for LauncherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
