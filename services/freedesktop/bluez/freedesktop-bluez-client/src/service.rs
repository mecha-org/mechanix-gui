//! High-level Bluetooth Service Abstraction
//!
//! This module provides the [`BluetoothService`] struct, a generic, high-level wrapper
//! around any implementation of the [`BluezInterface`] trait. It exposes convenient,
//! asynchronous methods for common Bluetooth operations such as enabling/disabling the
//! adapter, scanning for devices, connecting/disconnecting devices, and listing connected devices.
//!
//! # Features
//! - Simple, unified API for Bluetooth operations.
//! - Works with any backend that implements [`BluezInterface`].
//! - All methods are asynchronous and return [`Result`] types for robust error handling.
//!
//! # Example
//!
//! ```ignore
//! freedesktop-bluez-client::bluetooth::proxy::BluezProxy;
//! freedesktop-bluez-client::bluetooth::service::BluetoothService;
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

use crate::errors::BluezError;
use std::sync::mpsc;

use super::interfaces::{device::BluetoothDevice, BluezInterface};
use crate::proxies::BluezProxy;
use anyhow::Result;
use futures::StreamExt;
use log::{error, info};
use zbus::export::ordered_stream::OrderedStreamExt;
use zbus::Connection;

#[derive(Clone)]
pub struct BluetoothService {
    proxy: BluezProxy<'static>,
}

#[derive(Debug, Clone)]
pub enum BluetoothEvent {
    DeviceAdded,
    DeviceRemoved,
}
impl BluetoothService {
    /// Creates a new `BluetoothService` wrapping the given Bluetooth interface implementation.
    pub async fn new() -> Result<Self, BluezError> {
        let conn = Connection::system()
            .await
            .map_err(|e| BluezError::InitBusError(e.to_string()))?;
        let proxy = BluezProxy::new(&conn)
            .await
            .map_err(|e| BluezError::CreateBluezProxyError(e.to_string()))?;
        info!("bluez proxy created");
        Ok(Self { proxy })
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
    pub async fn toggle_bluetooth(&self, enabled: bool) -> Result<(), BluezError> {
        self.proxy
            .toggle_bluetooth(enabled)
            .await
            .map_err(BluezError::from)
    }

    /// Scans for available Bluetooth devices.
    ///
    /// # Arguments
    ///
    /// * `discovery_duration` - The duration to scan for Bluetooth devices.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<BluetoothDevice>)` with a list of discovered devices.
    /// * `Err` if scanning fails.
    pub async fn get_available_devices(
        &self,
        discovery_duration: core::time::Duration,
    ) -> Result<Vec<BluetoothDevice>, BluezError> {
        self.proxy
            .get_available_devices(discovery_duration)
            .await
            .map_err(BluezError::from)
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
    pub async fn connect(&self, device_address: &str) -> Result<(), BluezError> {
        self.proxy
            .connect(device_address)
            .await
            .map_err(BluezError::from)
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
    pub async fn disconnect(&self, device_address: &str) -> Result<(), BluezError> {
        self.proxy
            .disconnect(device_address)
            .await
            .map_err(BluezError::from)
    }

    /// Retrieves a list of currently connected Bluetooth devices.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<BluetoothDevice>)` with a list of connected devices.
    /// * `Err` if retrieval fails.
    pub async fn get_connected_devices(&self) -> Result<Vec<BluetoothDevice>, BluezError> {
        self.proxy
            .get_connected_devices()
            .await
            .map_err(BluezError::from)
    }
    pub async fn stream_bluetooth_enabled_status(&self) -> mpsc::Receiver<bool> {
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match proxy.stream_bluetooth_enabled_status().await {
                    Ok(mut stream) => {
                        while let Some(event) = stream.next().await {
                            if let Ok(state) = event.get().await {
                                // Blocking send (uses thread park/unpark internally)
                                if let Err(e) = sender.send(state) {
                                    error!("failed to send strength event to receiver: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("failed to stream bluetooth enabled status: {}", e);
                    }
                }
            });
        });
        receiver
    }

    pub async fn stream_bluetooth_device_status(&self) -> mpsc::Receiver<BluetoothEvent> {
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match proxy.stream_bluetooth_events().await {
                    Ok((mut added, mut removed)) => {
                        loop {
                            tokio::select! {
                                // Handle InterfacesAdded events
                                Some(_event) = OrderedStreamExt::next(&mut added) => {
                                    let event = BluetoothEvent::DeviceAdded;
                                    // Add your device-added logic here
                                    if let Err(e) = sender.send(event) {
                                    error!("failed to send device added event to receiver: {}", e);
                                    continue;
                                    }
                                }
                                // Handle InterfacesRemoved events
                                Some(_event) = OrderedStreamExt::next(&mut removed) => {
                                    let event = BluetoothEvent::DeviceRemoved;
                                    if let Err(e) = sender.send(event) {
                                    error!("failed to send device removed event to receiver: {}", e);
                                    continue;
                                    }
                                }
                                // Exit condition
                                else => break,
                            }
                        }
                    }
                    Err(e) => error!("Failed to stream events: {}", e),
                }
            });
        });
        receiver
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::interfaces::MockBluezInterface;
    use anyhow::anyhow;
    use mockall::predicate::*;
    use crate::proxies::ProxyError;

    const DISCOVERY_DURATION: core::time::Duration = core::time::Duration::from_secs(5);
    #[tokio::test]
    async fn test_toggle_bluetooth_success() {
        let mut mock = MockBluezInterface::new();
        mock.expect_toggle_bluetooth().times(1).returning(|_| Ok(()));
        let service = BluetoothService::new().await.unwrap();
        let result = service.toggle_bluetooth(true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_powered_off_propagates_errors() {
        let mut mock = MockBluezInterface::new();
        mock.expect_toggle_bluetooth()
            .times(1)
            .returning(|_| Err(ProxyError::DbusCallFailed("Failed to power off".to_string())));

        let service = BluetoothService::new().await.unwrap();
        let result = service.toggle_bluetooth(false).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_available_devices() {
        let test_devices = vec![BluetoothDevice {
            address: "00:11:22:33:44:55".to_string(),
            name: "Test Device".to_string(),
            ..Default::default()
        }];

        let mut mock = MockBluezInterface::new();
        mock.expect_get_available_devices()
            .with(eq(DISCOVERY_DURATION)) // Assuming 5 seconds for discovery duration
            .times(1)
            .returning(move |_| Ok(test_devices.clone()));

        let service = BluetoothService::new().await.unwrap();
        let result = service.get_available_devices(DISCOVERY_DURATION).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].address, "00:11:22:33:44:55");
    }

    #[tokio::test]
    async fn test_connect_device() {
        let mut mock = MockBluezInterface::new();
        mock.expect_connect()
            .with(eq("00:11:22:33:44:55"))
            .times(1)
            .returning(|_| Ok(()));

        let service = BluetoothService::new().await.unwrap();
        let result = service.connect("00:11:22:33:44:55").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_disconnect_device() {
        let mut mock = MockBluezInterface::new();
        mock.expect_disconnect()
            .with(eq("00:11:22:33:44:55"))
            .times(1)
            .returning(|_| Ok(()));

        let service = BluetoothService::new().await.unwrap();
        let result = service.disconnect("00:11:22:33:44:55").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_connected_devices() {
        let mut mock = MockBluezInterface::new();
        mock.expect_get_connected_devices().returning(|| {
            Ok(vec![BluetoothDevice {
                address: "00:11:22:33:44:55".to_string(),
                name: "Connected Device".to_string(),
                ..Default::default()
            }])
        });

        let service = BluetoothService::new().await.unwrap();
        let devices = service.get_connected_devices().await.unwrap();
        assert!(!devices.is_empty());
        assert_eq!(devices[0].name, "Connected Device");
    }
}
*/
