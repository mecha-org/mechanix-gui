//! UPower helper service.
//!
//! Provides high-level, type-safe access to UPower device information
//! by wrapping an implementation of the `UpowerInterface` trait.

use crate::errors::UpowerError;
use crate::interfaces::device::{BatteryLevel, BatteryState, PowerSourceType, WarningLevel};
use crate::interfaces::UPowerInterface;
use crate::proxies::DeviceProxy;
use anyhow::Result;
use futures::StreamExt;
use log::{error, info};
use std::sync::mpsc;
use zbus::Connection;

/// A service wrapper providing convenient methods for accessing UPower device data.
///
/// This struct is generic over any type implementing the [`UpowerInterface`] trait,
/// allowing for flexible backends (e.g., real D-Bus proxy or a mock for testing).
#[derive(Clone)]
pub struct UPowerService<> {
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
            .map_err(|e| UpowerError::InitSystemBusError(format!("{}", e)))?;
        let proxy = match DeviceProxy::new(&cn).await {
            Ok(n) => n,
            Err(e) => {
                error!("failed to create Device proxy: {}", e);
                return Err(UpowerError::CreateDeviceProxyError(format!("{}", e)));
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
                let level = match BatteryLevel::try_from(level) {
                    Ok(level) => level,
                    Err(e) => return Err(UpowerError::InvalidBatteryLevel(e.to_string())),
                };
                Ok(level)
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
        self.proxy.get_percentage().await.map_err(|e| e.into())
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
                let state = match BatteryState::try_from(state) {
                    Ok(state) => state,
                    Err(_e) => return Err(UpowerError::InvalidBatteryState.into()),
                };
                Ok(state)
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
                    Err(_e) => return Err(UpowerError::InvalidPowerSourceType.into()),
                };
                Ok(type_)
            }
            Err(e) => Err(e.into()),
        }
    }
    pub async fn stream_device_state(&self) -> mpsc::Receiver<BatteryState> {
        info!("service-action:: stream device state");
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match proxy.stream_device_state().await {
                    Ok(mut stream) => {
                        while let Some(event) = stream.next().await {
                            if let Ok(state) = event.get().await {
                                let state = BatteryState::from(state);
                                if let Err(e) = sender.send(state) {
                                    error!("failed to send battery state: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to stream to device state events: {}", e);
                    }
                }
            });
        });
        receiver
    }
    pub async fn stream_device_percentage(&self) -> mpsc::Receiver<f64> {
        info!("service-action:: stream device percentage");
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match proxy.stream_device_percentage().await {
                    Ok(mut stream) => {
                        while let Some(event) = stream.next().await {
                            if let Ok(state) = event.get().await {
                                if let Err(e) = sender.send(state) {
                                    error!("failed to send device percentage: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to stream to device percentage: {}", e);
                    }
                }
            });
        });
        receiver
    }
    pub async fn stream_battery_level(&self) -> mpsc::Receiver<BatteryLevel> {
        info!("service-action:: stream battery level");
        let proxy = self.proxy.clone();
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match proxy.stream_battery_level().await {
                    Ok(mut stream) => {
                        while let Some(event) = stream.next().await {
                            if let Ok(state) = event.get().await {
                                let state = BatteryLevel::from(state);
                                if let Err(e) = sender.send(state) {
                                    error!("failed to send battery level: {}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to stream to device percentage: {}", e);
                    }
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
    use crate::proxies::ProxyError;
    use anyhow::Result;
    use mockall::mock;
    use zbus::proxy::PropertyStream;

    // Mock the UpowerInterface trait
    mock! {
        pub UpowerInterface {}

        #[async_trait::async_trait]
        impl UPowerInterface for UpowerInterface {
            async fn get_battery_level(&self) -> Result<u32, ProxyError>;
            async fn get_warning_level(&self) -> Result<u32, ProxyError>;
            async fn get_percentage(&self) -> Result<f64, ProxyError>;
            async fn get_state(&self) -> Result<u32, ProxyError>;
            async fn get_power_source_type(&self) -> Result<u32, ProxyError>;
            async fn stream_device_state(&self) -> Result<PropertyStream<u32>, ProxyError>;
        }
    }

    #[tokio::test]
    async fn test_get_battery_level_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_battery_level().returning(|| Ok(3)); // 3 maps to BatteryLevel::Low

        let service = UpowerService::new().await.unwrap();
        let result = service.get_battery_level().await;
        assert_eq!(result.unwrap(), BatteryLevel::Low);
    }

    #[tokio::test]
    async fn test_get_battery_level_invalid_enum() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_battery_level().returning(|| Ok(42)); // 42 is not a valid BatteryLevel

        let service = UpowerService::new().await.unwrap();
        let result = service.get_battery_level().await;
        assert!(matches!(
            result.unwrap_err(),
            UpowerError::InvalidBatteryLevel(_)
        ));
    }

    #[tokio::test]
    async fn test_get_battery_level_error() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_battery_level()
            .returning(|| Err(ProxyError::DbusCallFailed("dbus error".into())));

        let service = UpowerService::new().await.unwrap();
        let result = service.get_battery_level().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "dbus error");
    }

    #[tokio::test]
    async fn test_get_warning_level_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_warning_level().returning(|| Ok(2)); // Assume 2 is a valid WarningLevel

        let service = UpowerService::new().await.unwrap();
        let result = service.get_warning_level().await;
        assert_eq!(result.unwrap(), WarningLevel::from(2));
    }

    #[tokio::test]
    async fn test_get_warning_level_error() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_warning_level()
            .returning(|| Err(ProxyError::DbusCallFailed("dbus error".into())));

        let service = UpowerService::new().await.unwrap();
        let result = service.get_warning_level().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_percentage_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_percentage().returning(|| Ok(77.7)); // Assume 77.7 is a valid percentage

        let service = UpowerService::new().await.unwrap();
        let result = service.get_percentage().await;
        assert_eq!(result.unwrap(), 77.7);
    }

    #[tokio::test]
    async fn test_get_state_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_state().returning(|| Ok(2)); // 2 maps to some BatteryState

        let service = UpowerService::new().await.unwrap();
        let result = service.get_state().await;
        assert_eq!(result.unwrap(), BatteryState::try_from(2).unwrap());
    }

    #[tokio::test]
    async fn test_get_state_invalid_enum() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_state().returning(|| Ok(99)); // Invalid BatteryState

        let service = UpowerService::new().await.unwrap();
        let result = service.get_state().await;
        assert!(matches!(
            result.unwrap_err(),
            UpowerError::InvalidBatteryState
        ));
    }

    #[tokio::test]
    async fn test_get_power_source_type_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_power_source_type().returning(|| Ok(2)); // 2 maps to Battery

        let service = UpowerService::new().await.unwrap();
        let result = service.get_power_source_type().await;
        assert_eq!(result.unwrap(), PowerSourceType::try_from(2).unwrap());
    }

    #[tokio::test]
    async fn test_get_power_source_type_invalid_enum() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_power_source_type().returning(|| Ok(99)); // Invalid PowerSourceType

        let service = UpowerService::new().await.unwrap();
        let result = service.get_power_source_type().await;
        assert!(matches!(
            result.unwrap_err(),
            UpowerError::InvalidPowerSourceType
        ));
    }
}
*/