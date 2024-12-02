use std::fmt;

use tracing::error;

/// # wireless module Error Codes
///
/// Implements standard errors
#[derive(Debug, Default, Clone, Copy)]
pub enum WirelessServiceErrorCodes {
    #[default]
    UnknownError,
    GetWirelessNetworkStatus,
    GetCurrentWirelessNetwork,
    ScanNetworks,
    KnownNetworks,
    EnableWireless,
    DisableWireless,
}

impl fmt::Display for WirelessServiceErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WirelessServiceErrorCodes::UnknownError => write!(f, "UnknownError"),
            WirelessServiceErrorCodes::GetWirelessNetworkStatus => {
                write!(f, "GetWirelessNetworkStatusError")
            }
            WirelessServiceErrorCodes::GetCurrentWirelessNetwork => {
                write!(f, "GetCurrentWirelessNetworkError")
            }
            WirelessServiceErrorCodes::EnableWireless => {
                write!(f, "EnableWirelessError")
            }
            WirelessServiceErrorCodes::DisableWireless => {
                write!(f, "DisableWirelessError")
            }
            WirelessServiceErrorCodes::ScanNetworks => {
                write!(f, "ScanNetworksError")
            }
            WirelessServiceErrorCodes::KnownNetworks => {
                write!(f, "KnownNetworksError")
            }
        }
    }
}

/// # WirelessServiceError
///
/// Implements a standard error type for all status bar related errors
/// includes the error code (`WirelessServiceErrorCodes`) and a message
#[derive(Debug, Default)]
pub struct WirelessServiceError {
    pub code: WirelessServiceErrorCodes,
    pub message: String,
}

impl WirelessServiceError {
    pub fn new(code: WirelessServiceErrorCodes, message: String) -> Self {
        error!("Error: (code: {:?}, message: {})", code, message);
        Self { code, message }
    }
}

impl std::fmt::Display for WirelessServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}
