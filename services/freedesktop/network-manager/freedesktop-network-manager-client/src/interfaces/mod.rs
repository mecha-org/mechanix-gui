//! # Network Manager Interface
//!
//! This module provides abstractions and types for interacting with a system's wireless/network manager.
//!
//! ## Overview
//!
//! - Defines the [`NetworkManagerInterface`] trait for common wireless operations such as enabling/disabling wireless,
//!   scanning for networks, connecting, and disconnecting.
//! - Contains types representing wireless events, access point information, and status reporting.
//!
//! ## Usage
//!
//! Implement the [`NetworkManagerInterface`] trait for your backend (e.g., D-Bus, mock).
//!
//! # Examples
//!
//! ```ignore
//! use your_crate::network_manager::{NetworkManagerInterface, WifiEvent};
//! # struct MyNetworkManager;
//! # #[async_trait::async_trait]
//! # impl NetworkManagerInterface for MyNetworkManager { /* ... */ }
//! #
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let nm = MyNetworkManager::new();
//! nm.set_wifi(true).await?;
//! let networks = nm.list_networks().await?;
//! # Ok(())
//! # }
//! ```
use anyhow::Result;
use async_trait::async_trait;
use wireless::{RawAccessPointInfo, WifiStatus};
use crate::proxies::ProxyError;

pub mod wireless;

/// Represents events emitted by the network manager.
///
/// This enum is used to communicate asynchronous events such as state changes
/// from the network manager to interested consumers.
#[derive(Debug, Clone)]
pub enum WifiEvent {
    /// Indicates that the wireless state has changed.
    /// Carries the new status as a `WifiStatus`.
    StateChanged(WifiStatus),
    // Add more events as needed
}

/// Defines the interface for interacting with a network manager.
///
/// This trait abstracts over different implementations (e.g., via D-Bus, mock for testing)
/// and provides asynchronous methods for common wireless management operations.
///
/// Types implementing this trait must be thread-safe (`Send + Sync`).
///
#[async_trait]
pub trait NetworkManagerInterface: Send + Sync {
    /// Enable the wireless device.
    /// When enabled, all managed interfaces are re-enabled and available to be activated.
    /// # Errors
    /// Return an error if the operation fails.
    async fn enable_wifi(&self) -> Result<(), ProxyError>;

    /// Disable the wireless device.
    /// When disabled, all interfaces that NM manages are deactivated.
    /// # Errors
    /// Return an error if the operation fails.
    async fn disable_wifi(&self) -> Result<(), ProxyError>;

    /// Get the list of available wireless networks.
    ///
    /// # Returns
    /// A vector of `RawAccessPointInfo` representing all visible access points.
    ///
    /// # Errors
    /// Return an error if the operation fails.
    async fn list_networks(&self) -> Result<Vec<RawAccessPointInfo>, ProxyError>;

    /// Select and connect to a specific network by SSID and optional password.
    ///
    /// # Arguments
    /// * `ssid` - The SSID of the network to connect to.
    /// * `password` - The password for the network, if required.
    ///
    /// # Returns
    /// On success, returns a tuple containing the new connection's object path
    /// and the active connection's object path, both as `String`.
    ///
    /// # Errors
    /// Return an error if the operation fails.
    async fn connect_to_network(
        &self,
        ssid: &str,
        password: Option<String>,
    ) -> Result<(String, String), ProxyError>;

    /// Disconnect from the current wireless network.
    ///
    /// # Errors
    /// Return an error if the operation fails.
    async fn disconnect(&self) -> Result<(), ProxyError>;

    // /// Get the current wireless connection status.
    // Async fn current_status(&self) -> Result<WifiStatus>;

    // /// Subscribe to wireless-related events (state changes, network list changes, etc.).
    // /// Returns a stream of events.
    // Async fn subscribe_events(&self) -> Result<BoxStream<'static, WifiEvent>>;
}
