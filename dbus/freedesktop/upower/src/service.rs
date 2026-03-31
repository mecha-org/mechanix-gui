//! UPower helper service.
//!
//! Provides high-level, type-safe access to UPower device information
//! by wrapping an implementation of the `UpowerInterface` trait.

use crate::errors::UpowerError;
use crate::interfaces::device::{BatteryLevel, BatteryState, PowerSourceType, WarningLevel};
use crate::interfaces::UPowerInterface;
use crate::proxies::DeviceProxy;
use anyhow::Result;
use futures::executor::ThreadPool;
use futures::{SinkExt, StreamExt};
use log::{error, info};
use std::sync::LazyLock;
use zbus::Connection;

static THREAD_POOL: LazyLock<ThreadPool> =
    LazyLock::new(|| ThreadPool::new().expect("Failed to build pool"));
/// A service wrapper providing convenient methods for accessing UPower device data.
///
/// This struct is generic over any type implementing the [`UpowerInterface`] trait,
/// allowing for flexible backends (e.g., real D-Bus proxy or a mock for testing).
#[derive(Clone)]
pub struct UPowerService {
    proxy: DeviceProxy<'static>,
}

impl UPowerService {
    /// Constructs a new [`UpowerService`] from the given interface implementation.
    ///
    /// # Arguments
    /// * `upower` - An object implementing the [`UpowerInterface`] trait.
    ///
    /// # Example
    /// ```ignore
    /// use upower::service::UpowerService;
    /// let service = UpowerService::new(my_upower_impl);
    /// ```
    pub async fn new() -> Result<Self, UpowerError> {
        let cn = Connection::system()
            .await
            .map_err(|e| UpowerError::InitSystemBusError(e.to_string()))?;
        let proxy = match DeviceProxy::new(&cn).await {
            Ok(n) => n,
            Err(e) => {
                error!("failed to create Device proxy: {}", e);
                return Err(UpowerError::CreateDeviceProxyError(e.to_string()));
            }
        };
        Ok(Self { proxy })
    }

    /// Asynchronously retrieves the battery level as a strongly typed [`BatteryLevel`] enum.
    ///
    /// # Returns
    /// * `Ok(BatteryLevel)` on success.
    /// * `Err(UpowerError::InvalidBatteryLevel)` if the returned value is not recognized.
    /// * Propagates any error from the underlying interface.
    pub async fn get_battery_level(&self) -> Result<BatteryLevel, UpowerError> {
        match self.proxy.get_battery_level().await {
            Ok(level) => {
                // Convert the raw value to the BatteryLevel enum, or return a descriptive error.
                Ok(BatteryLevel::from(level))
            }
            Err(e) => Err(UpowerError::from(e)),
        }
    }

    /// Asynchronously retrieves the battery warning level.
    ///
    /// # Returns
    /// * `Ok(WarningLevel)` on success.
    /// * Propagates any error from the underlying interface.
    pub async fn get_warning_level(&self) -> Result<WarningLevel, UpowerError> {
        match self.proxy.get_warning_level().await {
            Ok(level) => Ok(WarningLevel::from(level)),
            Err(e) => Err(UpowerError::from(e)),
        }
    }

    /// Asynchronously retrieves the battery percentage (0.0-100.0).
    ///
    /// # Returns
    /// * `Ok(f64)` containing the battery percentage.
    /// * Propagates any error from the underlying interface.
    pub async fn get_percentage(&self) -> Result<f64, UpowerError> {
        self.proxy.get_percentage().await.map_err(Into::into)
    }

    /// Asynchronously retrieves the current battery status as a [`BatteryState`] enum.
    ///
    /// # Returns
    /// * `Ok(BatteryState)` on success.
    /// * `Err(UpowerError::InvalidBatteryState)` if the returned value is not recognized.
    /// * Propagates any error from the underlying interface.
    pub async fn get_state(&self) -> Result<BatteryState, UpowerError> {
        match self.proxy.get_state().await {
            Ok(state) => {
                // Convert the raw value to the BatteryState enum or return a descriptive error.
                Ok(BatteryState::from(state))
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Asynchronously retrieves the type of power source as a [`PowerSourceType`] enum.
    ///
    /// # Returns
    /// * `Ok(PowerSourceType)` on success.
    /// * `Err(UpowerError::InvalidPowerSourceType)` if the returned value is not recognized.
    /// * Propagates any error from the underlying interface.
    pub async fn get_power_source_type(&self) -> Result<PowerSourceType, UpowerError> {
        match self.proxy.get_power_source_type().await {
            Ok(type_) => {
                // Convert the raw value to the PowerSourceType enum, or return a descriptive error.
                let type_ = match PowerSourceType::try_from(type_) {
                    Ok(type_) => type_,
                    Err(_e) => return Err(UpowerError::InvalidPowerSourceType),
                };
                Ok(type_)
            }
            Err(e) => Err(e.into()),
        }
    }
    pub async fn stream_device_state(&self) -> futures::channel::mpsc::Receiver<BatteryState> {
        info!("service-action:: stream device state");
        let proxy = self.proxy.clone();
        let (mut sender, receiver) = futures::channel::mpsc::channel(250);
        THREAD_POOL.spawn_ok(async move {
            match proxy.stream_device_state().await {
                Ok(mut stream) => {
                    while let Some(event) = stream.next().await {
                        if let Ok(state) = event.get().await {
                            let state = BatteryState::from(state);
                            match sender.send(state).await {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("failed to send battery state: {}", e);
                                    continue;
                                }
                            };
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to stream to device state events: {}", e);
                }
            }
        });
        receiver
    }
    pub async fn stream_device_percentage(&self) -> futures::channel::mpsc::Receiver<f64> {
        info!("service-action:: stream device percentage");
        let proxy = self.proxy.clone();
        let (mut sender, receiver) = futures::channel::mpsc::channel(250);
        THREAD_POOL.spawn_ok(async move {
            match proxy.stream_device_percentage().await {
                Ok(mut stream) => {
                    while let Some(event) = stream.next().await {
                        if let Ok(state) = event.get().await {
                            match sender.send(state).await {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("failed to send device percentage: {}", e);
                                    continue;
                                }
                            };
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to stream to device percentage: {}", e);
                }
            }
        });
        receiver
    }
    pub async fn stream_battery_level(&self) -> futures::channel::mpsc::Receiver<BatteryLevel> {
        info!("service-action:: stream battery level");
        let proxy = self.proxy.clone();
        let (mut sender, receiver) = futures::channel::mpsc::channel(250);
        THREAD_POOL.spawn_ok(async move {
            match proxy.stream_battery_level().await {
                Ok(mut stream) => {
                    while let Some(event) = stream.next().await {
                        if let Ok(state) = event.get().await {
                            info!("battery level is updated: {:}", state);
                            let state = BatteryLevel::from(state);
                            match sender.send(state).await {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("failed to send battery level: {}", e);
                                    continue;
                                }
                            };
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to stream battery level events: {}", e);
                }
            }
        });
        receiver
    }
}
