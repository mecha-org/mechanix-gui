//! Upower errors

use crate::proxies::ProxyError;

/// Represents errors that can occur when interacting with UPower.
#[derive(Clone, Debug,  thiserror::Error)]
#[non_exhaustive]
pub enum UpowerError {
    /// A generic error with no additional context.
    #[error("generic error")]
    Generic,

    /// An error originating from the proxy layer.
    /// This wraps a `ProxyError` to provide additional context.
    #[error("proxy error: {0}")]
    ProxyError(#[from] ProxyError),

    /// Indicates a failure to create a UPower device proxy.
    /// The associated `String` provides additional details about the error.
    #[error("failed to create upower device proxy {0:?}")]
    CreateDeviceProxyError(String),

    /// Indicates a failure to connect to the system bus.
    /// The associated `String` provides additional details about the error.
    #[error("failed to initialize the system bus {0:?}")]
    InitSystemBusError(String),

    /// Represents an invalid battery level error.
    #[error("invalid battery level {0:?}")]
    InvalidBatteryLevel(String),

    /// Represents an invalid power source type error.
    #[error("invalid power source type")]
    InvalidPowerSourceType,

    /// Represents a failure to retrieve the battery state.
    #[error("failed to get battery state")]
    InvalidBatteryState,
}