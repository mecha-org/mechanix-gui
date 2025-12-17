//! Represents errors that can occur when interacting with Mechanix Notification Service.


/// This enum uses the [`thiserror`](https://docs.rs/thiserror) crate to provide detailed, user-friendly error messages for each variant.
/// It is marked as `#[non_exhaustive]` to allow for future extension without breaking existing code.
///
/// # Variants
///
/// - `Generic`: A catch-all error for unspecified failures.
/// - `CreateNmProxyError`: Indicates failure to create a NetworkManager proxy, with additional context.
/// - `GetListOfNetworksError`: Indicates failure to retrieve the list of networks, with additional context.
/// - `ConnectToNewNetworkError`: Indicates failure to connect to a new network, with additional context.
///
/// Each variant carries a descriptive error message that is displayed when the error is formatted.

#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MechanixNotificationError {
    /// A generic, unspecified error.
    #[error("generic error")]
    Generic,
    /// Failed to create a NetworkManager proxy.
    ///
    /// The string contains additional context or the underlying error message.
    #[error("failed to create network_manager proxy {0:?}")]
    CreateProxyError(String),
    #[error("failed to get all notifications {0:?}")]
    GetAllNotificationsFailed(String),
}
