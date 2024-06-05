use std::fmt;

use tracing::error;

/// # Settings panel Error Codes
///
/// Implements standard errors for the Settings panel
#[derive(Debug, Default, Clone, Copy)]
pub enum SettingsPanelErrorCodes {
    #[default]
    UnknownError,
    SettingsReadError,
    SettingsParseError,
    ThemeReadError,
    ThemeParseError,
    GetWirelessStatusError,
    GetBluetoothStatusError,
    GetCpuUsageError,
    GetMemoryInfoError,
    GetBrightnessError,
    SetBrightnessError,
    GetSoundError,
    SetSoundError,
    GetBatteryStatusError,
    EnableWireless,
    DisableWireless,
    EnableBluetooth,
    DisableBluetooth,
    GetBatteryError,
}

impl fmt::Display for SettingsPanelErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingsPanelErrorCodes::UnknownError => write!(f, "UnknownError"),
            SettingsPanelErrorCodes::SettingsReadError => write!(f, "SettingsReadError"),
            SettingsPanelErrorCodes::SettingsParseError => write!(f, "SettingsParseError"),
            SettingsPanelErrorCodes::ThemeReadError => write!(f, "ThemeReadError"),
            SettingsPanelErrorCodes::ThemeParseError => write!(f, "ThemeParseError"),
            SettingsPanelErrorCodes::GetBluetoothStatusError => {
                write!(f, "GetBluetoothStatusError")
            }
            SettingsPanelErrorCodes::GetWirelessStatusError => write!(f, "GetWirelessStatusError"),
            SettingsPanelErrorCodes::GetCpuUsageError => write!(f, "GetCpuUsageError"),
            SettingsPanelErrorCodes::GetMemoryInfoError => write!(f, "GetMemoryInfoError"),
            SettingsPanelErrorCodes::GetBrightnessError => write!(f, "GetBrightnessError"),
            SettingsPanelErrorCodes::SetBrightnessError => write!(f, "SetBrightnessError"),
            SettingsPanelErrorCodes::GetSoundError => write!(f, "GetSoundError"),
            SettingsPanelErrorCodes::SetSoundError => write!(f, "SetSoundError"),
            SettingsPanelErrorCodes::EnableWireless => write!(f, "EnableWireless"),
            SettingsPanelErrorCodes::DisableWireless => write!(f, "DisableWireless"),
            SettingsPanelErrorCodes::EnableBluetooth => write!(f, "EnableBluetooth"),
            SettingsPanelErrorCodes::DisableBluetooth => write!(f, "DisableBluetooth"),
            SettingsPanelErrorCodes::GetBatteryStatusError => write!(f, "GetBatteryStatusError"),
            SettingsPanelErrorCodes::GetBatteryError => write!(f, "GetBatteryError"),
        }
    }
}

/// # SettingsPanelError
///
/// Implements a standard error type for all Settings panel related errors
/// includes the error code (`SettingsPanelErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct SettingsPanelError {
    pub code: SettingsPanelErrorCodes,
    pub message: String,
}

impl SettingsPanelError {
    pub fn new(code: SettingsPanelErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for SettingsPanelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
