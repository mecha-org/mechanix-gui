//! Bluetooth errors

use super::proxy::ProxyError;

/// An error that can occur while handling bluetooth operations.
#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BluetoothError {
    /// A generic, unspecified error.
    #[error("generic error")]
    Generic,

    /// An error originating from the proxy layer.
    #[error("proxy error: {0}")]
    ProxyError(#[from] ProxyError),

    /// Failure to create the system D-Bus connection.
    #[error("failed to create system bus: {0}")]
    CreateSystemBusError(String),

    /// Failure to create the BlueZ proxy.
    #[error("failed to create bluz proxy: {0}")]
    CreateBluezProxyError(String),

    /// Failure to create the adapter proxy.
    #[error("failed to create adapter proxy: {0}")]
    CreateAdapterProxyError(String),
}
