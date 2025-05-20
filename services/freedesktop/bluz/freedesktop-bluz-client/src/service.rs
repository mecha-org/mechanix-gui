//! High-level Bluetooth Service Abstraction
//!
//! This module provides the [`BluetoothService`] struct, a generic, high-level wrapper
//! around any implementation of the [`BluetoothInterface`] trait. It exposes convenient,
//! asynchronous methods for common Bluetooth operations such as enabling/disabling the
//! adapter, scanning for devices, connecting/disconnecting devices, and listing connected devices.
//!
//! # Features
//! - Simple, unified API for Bluetooth operations.
//! - Works with any backend that implements [`BluetoothInterface`].
//! - All methods are asynchronous and return [`Result`] types for robust error handling.
//!
//! # Example
//!
//! ```ignore
//! mechanix_debus_client::bluetooth::proxy::BluezProxy;
//! mechanix_debus_client::bluetooth::service::BluetoothService;
//! # async fn example() -> anyhow::Result<()> {
//! let proxy = BluezProxy::new(/* ... */);
//! let service = BluetoothService::new(proxy);
//! service.set_powered_on().await?;
//! let devices = service.get_available_devices().await?;
//! # Ok(())
//! # }
//! ```
//!
//! This abstraction makes it easy to swap out or mock Bluetooth backends for testing or platform support.
use crate::error::BluetoothError;

use super::interface::{device::BluetoothDeviceProps, BluetoothInterface};
use anyhow::Result;

pub struct BluetoothService<T: BluetoothInterface> {
    nm: T,
}

impl<T: BluetoothInterface> BluetoothService<T> {
    /// Creates a new `BluetoothService` wrapping the given Bluetooth interface implementation.
    ///
    /// # Arguments
    ///
    /// * `nm` - An implementation of the [`BluetoothInterface`] trait.
    pub fn new(nm: T) -> Self {
        Self { nm }
    }

    /// Enables the Bluetooth adapter.
    ///
    /// This method attempts to power on the Bluetooth adapter, making it available
    /// for scanning, connecting, and other Bluetooth operations.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if enabling Bluetooth is successful.
    /// * `Err` if enabling Bluetooth fails.
    pub async fn set_powered_on(&self) -> Result<(), BluetoothError> {
        self.nm.set_powered_on().await.map_err(BluetoothError::from)
    }

    /// Disables the Bluetooth adapter.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if disabling Bluetooth is successful.
    /// * `Err` if disabling Bluetooth fails.
    pub async fn set_powered_off(&self) -> Result<(), BluetoothError> {
        self.nm
            .set_powered_off()
            .await
            .map_err(BluetoothError::from)
    }

    /// Scans for available Bluetooth devices.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<BluetoothDeviceProps>)` with a list of discovered devices.
    /// * `Err` if scanning fails.
    pub async fn get_available_devices(
        &self,
        discovery_duration: u64,
    ) -> Result<Vec<BluetoothDeviceProps>, BluetoothError> {
        self.nm
            .get_available_devices(discovery_duration)
            .await
            .map_err(BluetoothError::from)
    }

    /// Connects to a Bluetooth device by its address.
    ///
    /// # Arguments
    ///
    /// * `device_address` - The MAC address of the Bluetooth device to connect to.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the connection is successful.
    /// * `Err` if the connection fails.
    pub async fn connect(&self, device_address: &str) -> Result<(), BluetoothError> {
        self.nm
            .connect(device_address)
            .await
            .map_err(BluetoothError::from)
    }

    /// Disconnects from a Bluetooth device by its address.
    ///
    /// # Arguments
    ///
    /// * `device_address` - The MAC address of the Bluetooth device to disconnect from.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the disconnection is successful.
    /// * `Err` if the disconnection fails.
    pub async fn disconnect(&self, device_address: &str) -> Result<(), BluetoothError> {
        self.nm
            .disconnect(device_address)
            .await
            .map_err(BluetoothError::from)
    }

    /// Retrieves a list of currently connected Bluetooth devices.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<BluetoothDeviceProps>)` with a list of connected devices.
    /// * `Err` if retrieval fails.
    pub async fn get_connected_devices(&self) -> Result<Vec<BluetoothDeviceProps>, BluetoothError> {
        self.nm.get_connected_devices().await.map_err(BluetoothError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::MockBluetoothInterface;
    use anyhow::anyhow;
    use mockall::predicate::*;
    use crate::proxy::ProxyError;

    #[tokio::test]
    async fn test_set_powered_on_success() {
        let mut mock = MockBluetoothInterface::new();
        mock.expect_set_powered_on().times(1).returning(|| Ok(()));
        let service = BluetoothService::new(mock);
        let result = service.set_powered_on().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_powered_off_propagates_errors() {
        let mut mock = MockBluetoothInterface::new();
        mock.expect_set_powered_off()
            .times(1)
            .returning(|| Err(ProxyError::DbusCallFailed("Failed to power off".to_string())));

        let service = BluetoothService::new(mock);
        let result = service.set_powered_off().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_available_devices() {
        let test_devices = vec![BluetoothDeviceProps {
            address: "00:11:22:33:44:55".to_string(),
            name: "Test Device".to_string(),
            ..Default::default()
        }];

        let mut mock = MockBluetoothInterface::new();
        mock.expect_get_available_devices()
            .with(eq(5)) // Assuming 5 seconds for discovery duration
            .times(1)
            .returning(move |_| Ok(test_devices.clone()));

        let service = BluetoothService::new(mock);
        let result = service.get_available_devices(1000).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].address, "00:11:22:33:44:55");
    }

    #[tokio::test]
    async fn test_connect_device() {
        let mut mock = MockBluetoothInterface::new();
        mock.expect_connect()
            .with(eq("00:11:22:33:44:55"))
            .times(1)
            .returning(|_| Ok(()));

        let service = BluetoothService::new(mock);
        let result = service.connect("00:11:22:33:44:55").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_disconnect_device() {
        let mut mock = MockBluetoothInterface::new();
        mock.expect_disconnect()
            .with(eq("00:11:22:33:44:55"))
            .times(1)
            .returning(|_| Ok(()));

        let service = BluetoothService::new(mock);
        let result = service.disconnect("00:11:22:33:44:55").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_connected_devices() {
        let mut mock = MockBluetoothInterface::new();
        mock.expect_get_connected_devices().returning(|| {
            Ok(vec![BluetoothDeviceProps {
                address: "00:11:22:33:44:55".to_string(),
                name: "Connected Device".to_string(),
                ..Default::default()
            }])
        });

        let service = BluetoothService::new(mock);
        let devices = service.get_connected_devices().await.unwrap();
        assert!(!devices.is_empty());
        assert_eq!(devices[0].name, "Connected Device");
    }
}
