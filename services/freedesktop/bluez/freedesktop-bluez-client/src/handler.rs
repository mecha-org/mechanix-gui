//! Module contains the `BluezClient` struct, which handles Bluetooth requests.

use super::interfaces::device::BluetoothDevice;
use crate::errors::BluezError;
use crate::proxies::BluezProxy;
use crate::service::BluetoothService;
use anyhow::{bail, Result};
use log::error;
use tokio::sync::oneshot;
use tokio::{select, sync::mpsc};
use zbus::Connection;

/// Represents different types of Bluetooth requests that can be handled by `BluezClient`.
#[derive(Debug)]
#[non_exhaustive]
pub enum BluezRequest {
    /// Request to power on Bluetooth.
    SetPoweredOn {
        /// Channel to send the result of the operation.
        reply_to: oneshot::Sender<Result<(), BluezError>>,
    },
    /// Request to power off Bluetooth.
    SetPoweredOff {
        /// Channel to send the result of the operation.
        reply_to: mpsc::Sender<Result<(), BluezError>>,
    },
    /// Request to scan for available Bluetooth devices.
    /// The result will be sent back via the provided channel.
    GetAvailableDevices {
        /// Duration for scanning for devices in milliseconds.
        discovery_duration: u64,
        /// Channel to send the result of the device scan.
        reply_to: mpsc::Sender<Result<Vec<BluetoothDevice>, BluezError>>,
    },
    /// Request to connect to a Bluetooth device by address.
    ConnectDevice {
        /// The address of the Bluetooth device to connect.
        device_address: String,
        /// Channel to send the result of the operation.
        reply_to: mpsc::Sender<Result<(), BluezError>>,
    },
    /// Request to disconnect from a Bluetooth device by address.
    DisconnectDevice {
        /// The address of the Bluetooth device to disconnect.
        device_address: String,
        /// Channel to send the result of the operation.
        reply_to: mpsc::Sender<Result<(), BluezError>>,
    },
    /// Request to get a list of currently connected Bluetooth devices.
    /// The result will be sent back via the provided channel.
    GetConnectedDevices {
        /// Channel to send the result of the connected devices query.
        reply_to: mpsc::Sender<Result<Vec<BluetoothDevice>, BluezError>>,
    },
}

/// Handles Bluetooth requests asynchronously using a Tokio mpsc channel.
pub struct BluezClient {
    /// The D-Bus system connection used for communicating with BlueZ.
    cn: Connection,
}

impl BluezClient {
    /// Creates a new `BluezClient` with a system D-Bus connection.
    ///
    /// # Errors
    ///
    /// Returns a [`BluezError`] if the system bus connection cannot be established.
    pub async fn new() -> Result<Self, BluezError> {
        let cn = Connection::system().await.map_err(|e| {
            BluezError::CreateSystemBusError(format!("failed to connect to system bus: {}", e)) //Dbus_connection_eror
        })?;
        Ok(Self { cn })
    }

    /// Runs the Bluetooth handler, processing incoming Bluetooth requests from the provided channel.
    ///
    /// # Arguments
    ///
    /// * `bt_request` - A receiver channel for incoming [`BluezRequest`]s.
    ///
    /// This method runs an infinite loop, handling each request as it arrives.
    /// It uses a `BluetoothService` to perform the actual Bluetooth operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the BlueZ proxy cannot be created or if a fatal error occurs in the loop.
    pub async fn run(&mut self, mut bt_request: mpsc::Receiver<BluezRequest>) -> Result<(), BluezError> {
        // Create a new Bluez proxy for Bluetooth operations.
        let proxy = match BluezProxy::new(&self.cn).await {
            Ok(n) => n,
            Err(e) => {
                error!("failed to create Bluez proxy: {}", e);
                return Err(BluezError::CreateBluezProxyError(format!("{}", e)))
            },
        };

        // Create a new Bluetooth service to handle requests.
        let service = BluetoothService::new(proxy);

        loop {
            select! {
                msg = bt_request.recv() => {
                    if let Some(request) = msg {
                        match request {
                            BluezRequest::SetPoweredOn { reply_to } => {
                                // Handle enabling Bluetooth.
                                let result = service.set_powered_on().await;
                                if let Err(e) = reply_to.send(result) {
                                    log::error!("failed to send the result of set_powered_on: {:?}", e);
                                }
                            },
                            BluezRequest::SetPoweredOff { reply_to } => {
                                // Handle disabling Bluetooth.
                                let result = service.set_powered_off().await;
                                if let Err(e) = reply_to.send(result).await {
                                    log::error!("failed to send the result of set_powered_off: {:?}", e);
                                }
                            },
                            BluezRequest::GetAvailableDevices { discovery_duration, reply_to } => {
                                // Handle scanning for available devices and send the result.
                                let result = service.get_available_devices(discovery_duration).await;
                                if let Err(e) = reply_to.send(result).await {
                                    log::error!("failed to send the result of available devices: {:?}", e);
                                }
                            },
                            BluezRequest::ConnectDevice { device_address, reply_to } => {
                                // Handle connecting to a device.
                                let result = service.connect(&device_address).await;
                                if let Err(e) = reply_to.send(result).await {
                                    log::error!("failed to send the result of connect to device {}: {:?}", device_address, e);
                                }
                            },
                            BluezRequest::DisconnectDevice { device_address, reply_to } => {
                                // Handle disconnecting from a device.
                                let result = service.disconnect(&device_address).await;
                                if let Err(e) = reply_to.send(result).await {
                                    log::error!("failed to send the result of disconnect from device {}: {:?}", device_address, e);
                                }
                            },
                            BluezRequest::GetConnectedDevices { reply_to } => {
                                // Handle retrieving connected devices and send the result.
                                let result = service.get_connected_devices().await;
                                if let Err(e) = reply_to.send(result).await {
                                    log::error!("failed to send the result of connected devices: {:?}", e);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
