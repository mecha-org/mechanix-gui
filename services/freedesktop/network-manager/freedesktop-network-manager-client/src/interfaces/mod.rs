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
use crate::proxies::wireless::{AccessPointAddedStream, AccessPointRemovedStream};
use crate::proxies::ProxyError;
use anyhow::Result;
use async_trait::async_trait;
use wireless::{RawAccessPointInfo, WifiStatus};
use zbus::proxy::PropertyStream;

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
pub trait NetworkManagerInterface: Send + Sync + Clone {
    /// Toggle the wireless device.
    /// When enabled, all managed interfaces are re-enabled and available to be activated.
    /// When disabled, all interfaces that NM manages are deactivated.
    /// # Arguments
    /// * `enabled` - If `true`, wireless will be enabled; if `false`, wireless will be disabled.
    ///
    /// # Errors
    /// Return an error if the operation fails.
    async fn toggle_wireless(&self, enabled: bool) -> Result<(), ProxyError>;


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
        password: &Option<String>,
    ) -> Result<(String, String), ProxyError>;

    /// Connect to a previously saved network.
    ///
    /// # Arguments
    /// * `ssid` - The name of the saved network to connect to.
    ///
    /// # Returns
    /// On success, returns a tuple containing the new connection's object path
    /// and the active connection's object path, both as `String`.
    ///
    /// # Errors
    /// Return an error if the operation fails.
    async fn connect_to_saved_network(
        &self,
        ssid: &str,
    ) -> Result<(), ProxyError>;

    /// Forget a previously saved network.
    ///
    /// # Arguments
    /// * `ssid` - The name of the saved network to forget.
    ///
    /// # Errors
    /// Return an error if the operation fails.
    async fn forget_saved_network(&self, ssid: &str) -> Result<(), ProxyError>;
    /// Disconnect from the current wireless network.
    ///
    /// # Errors
    /// Return an error if the operation fails.
    async fn disconnect(&self) -> Result<(), ProxyError>;
    async fn get_access_point_info(
        &self,
        object_path: &str,
    ) -> Result<RawAccessPointInfo, ProxyError>;
    /// Subscribe to NetworkManager WiFi state change events.
    ///
    /// This method sets up an event subscription that will send WiFi state updates
    /// through the provided channel whenever the NetworkManager reports state changes.
    ///
    /// # Arguments
    /// * `sender` - A tokio mpsc::Sender that will be used to send WifiState updates
    ///             to the subscriber. The sender should remain active for as long as
    ///             the subscription is needed.
    ///
    /// # Returns
    /// * `Ok(())` - If the subscription was successfully set up
    /// * `Err(ProxyError)` - If there was an error setting up the subscription
    async fn stream_device_events(&self) -> Result<PropertyStream<u32>, ProxyError>;
    async fn stream_access_point_events(
        &self,
    ) -> Result<(AccessPointAddedStream, AccessPointRemovedStream), ProxyError>;
    async fn stream_wireless_enabled_status(&self) -> Result<PropertyStream<bool>, ProxyError>;
    async fn stream_wireless_network_strength(&self) -> Result<PropertyStream<u8>, ProxyError>;
}
