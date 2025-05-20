//! UPower D-Bus service handler for managing power-related device information and events.

use crate::errors::UpowerError;
use crate::interfaces::device::{BatteryLevel, BatteryState, PowerSourceType, WarningLevel};
use crate::proxies::DeviceProxy;
use crate::service::UpowerService;
use anyhow::{Result, bail};
use log::error;
use tokio::{select, sync::mpsc};
use zbus::Connection;

/// Enum representing different types of requests that can be sent to the UPower handler.
/// Each variant contains an MPSC channel sender for returning the operation result.
#[derive(Debug)]
#[non_exhaustive]
pub enum UpowerRequest {
    /// Retrieves the current battery level (0-100)
    GetBatteryLevel {
        reply_to: mpsc::Sender<Result<BatteryLevel, UpowerError>>,
    },
    /// Gets the battery warning level (e.g., low, critical)
    GetWarningLevel {
        reply_to: mpsc::Sender<Result<WarningLevel, UpowerError>>,
    },
    /// Fetches the exact battery percentage (0.0-100.0)
    GetPercentage { reply_to: mpsc::Sender<Result<f64, UpowerError>> },
    /// Returns the current battery state (e.g., charging, discharging)
    GetState {
        reply_to: mpsc::Sender<Result<BatteryState, UpowerError>>,
    },
    /// Gets the device power source type (e.g., battery, UPS)
    GetPowerSourceType {
        reply_to: mpsc::Sender<Result<PowerSourceType, UpowerError>>,
    },
    /// Placeholder for device state change event notifications
    GetDeviceStateChangeEvent {
        reply_to: mpsc::Sender<Result<String, UpowerError>>,
    },
}

/// Main handler for UPower D-Bus interactions using zbus-rs.
/// Manages the connection and processes incoming requests.
pub struct UpowerHandler {
    /// zbus system connection for D-Bus communication
    cn: Connection,
}

impl UpowerHandler {
    /// Creates a new UPower handler with a system D-Bus connection.
    pub async fn new() -> Result<Self, UpowerError> {
        let cn = Connection::system()
            .await
            .map_err(|e| UpowerError::CreateSystemBusError(format!("{}", e)))?;
        Ok(Self { cn })
    }

    /// Main event loop that processes incoming UPower requests.
    ///
    /// # Parameters
    /// - `request`: MPSC channel receiver for incoming requests
    ///
    /// # Returns
    /// - `Result<()>`: Ok on normal operation, Error on critical failures
    ///
    /// # Behavior
    /// - Creates DeviceProxy and UpowerService on startup
    /// - Uses Tokio's select! macro to handle incoming requests
    /// - Routes requests to appropriate service methods
    /// - Sends responses back through provided reply channels
    pub async fn run(&mut self, mut request: mpsc::Receiver<UpowerRequest>) -> Result<(),  UpowerError> {
        // Create the proxy and service with proper error handling
        let proxy = match DeviceProxy::new(&self.cn).await {
            Ok(n) => n,
            Err(e) => {
                error!("failed to create Device proxy: {}", e);
                return Err(UpowerError::CreateDeviceProxyError(format!("{}", e)))
            },
        };
        let service = UpowerService::new(proxy);

        loop {
            select! {
                msg = request.recv() => {
                    if let Some(request) = msg {
                        match request {
                            UpowerRequest::GetBatteryLevel { reply_to } => {
                                // Attempt to get the battery level and log any errors
                                let result = service.get_battery_level().await.map_err(|e| {
                                    log::error!("failed to get battery level: {}", e);
                                    e
                                });

                                // Attempt to send the result and log any errors
                                if let Err(e) = reply_to.send(result).await {
                                    log::error!("failed to send battery level: {}", e);
                                }
                            }
                            UpowerRequest::GetWarningLevel { reply_to } => {
                                // Attempt to get the warning level and log any errors
                                let result = service.get_warning_level().await.map_err(|e| {
                                    log::error!("failed to get warning level: {}", e);
                                    e
                                });

                                // Attempt to send the result and log any errors
                                let result = reply_to.send(result).await;
                                if let Err(e) = result {
                                    log::error!("failed to send warning level: {}", e);
                                }
                            }
                            UpowerRequest::GetPercentage { reply_to } => {
                                // Attempt to get the percentage and log any errors
                                let result = service.get_percentage().await.map_err(|e| {
                                    log::error!("failed to get percentage: {}", e);
                                    e
                                });

                                // Attempt to send the result and log any errors
                                let result = reply_to.send(result).await;
                                if let Err(e) = result {
                                    log::error!("failed to send percentage: {}", e);
                                }
                            }
                            UpowerRequest::GetState { reply_to } => {
                                // Attempt to get the state and log any errors
                                let result = service.get_state().await.map_err(|e| {
                                    log::error!("failed to get state: {}", e);
                                    e
                                });

                                // Attempt to send the result and log any errors
                                let result = reply_to.send(result).await;
                                if let Err(e) = result {
                                    log::error!("failed to send state: {}", e);
                                }
                            }
                            UpowerRequest::GetPowerSourceType { reply_to } => {
                                // Attempt to get the type and log any errors
                                let result = service.get_power_source_type().await.map_err(|e| {
                                    log::error!("failed to get type: {}", e);
                                    e
                                });

                                // Attempt to send the result and log any errors
                                let result = reply_to.send(result).await;
                                if let Err(e) = result {
                                    log::error!("failed to send type: {}", e);
                                }
                            }
                            UpowerRequest::GetDeviceStateChangeEvent { reply_to } => {
                                // TODO: Implement actual device state change event handling
                                // Currently returns a placeholder message
                                let _ = reply_to.send(Ok("Device state changed".to_string()));
                            }
                        }
                    }
                }
            }
        }
    }
}
