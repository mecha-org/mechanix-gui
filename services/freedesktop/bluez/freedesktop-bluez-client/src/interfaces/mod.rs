//! Bluetooth Interface Abstraction
//!
//! This module defines the [`BluezInterface`] trait, which provides an asynchronous,
//! high-level abstraction for Bluetooth device management. It specifies essential
//! operations such as enabling/disabling Bluetooth, scanning for devices, connecting
//! and disconnecting devices, and retrieving connected devices.
//!
//! Key features:
//! - Asynchronous trait methods for non-blocking Bluetooth operations.
//! - Strongly-typed device representation via [`BluetoothDevice`].
//!
//! Implement this trait to provide platform-specific Bluetooth functionality.

use anyhow::Result;
use async_trait::async_trait;
use device::BluetoothDevice;
use mockall::automock;

use crate::proxies::ProxyError;
pub mod device;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait BluezInterface: Send + Sync {
    /// Power on the bluetooth.
    async fn set_powered_on(&self) -> Result<(), ProxyError>;

    /// Power off the bluetooth.
    async fn set_powered_off(&self) -> Result<(), ProxyError>;

    /// Scan the bluetooth devices.
    async fn get_available_devices(
        &self,
        discovery_duration: u64,
    ) -> Result<Vec<BluetoothDevice>, ProxyError>;

    /// Connect to a bluetooth device.
    async fn connect(&self, address: &str) -> Result<(), ProxyError>;

    /// Disconnect from a bluetooth device.
    async fn disconnect(&self, address: &str) -> Result<(), ProxyError>;

    /// Get connected devices.
    async fn get_connected_devices(&self) -> Result<Vec<BluetoothDevice>, ProxyError>;
}
