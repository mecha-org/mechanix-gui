//! TimeDate errors

use crate::proxies::ProxyError;

/// An error that can occur while handling timedate operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TimeDateError {
    /// A generic, unspecified error.
    #[error("generic error")]
    Generic,

    /// An error originating from the proxy layer.
    #[error("proxy error: {0}")]
    ProxyError(#[from] ProxyError),
    
    /// An error occurred while initializing the D-Bus connection.
    #[error("failed to init D-Bus connection: {0}")]
    InitBusError(String),
    
    /// An error occurred while creating the D-Bus proxy.
    #[error("failed to create D-Bus proxy: {0}")]
    CreateProxyError(String),
}
