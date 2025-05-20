//! UPower helper service.
//!
//! Provides high-level, type-safe access to UPower device information
//! by wrapping an implementation of the `UpowerInterface` trait.

use crate::errors::UpowerError;
use crate::interfaces::device::{BatteryLevel, BatteryState, PowerSourceType, WarningLevel};
use crate::interfaces::UpowerInterface;
use anyhow::Result;

/// A service wrapper providing convenient methods for accessing UPower device data.
///
/// This struct is generic over any type implementing the [`UpowerInterface`] trait,
/// allowing for flexible backends (e.g., real D-Bus proxy or a mock for testing).
pub struct UpowerService<T: UpowerInterface> {
    /// The underlying UPower interface implementation.
    upower: T,
}

impl<T: UpowerInterface> UpowerService<T> {
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
    pub fn new(upower: T) -> Self {
        Self { upower }
    }

    /// Asynchronously retrieves the battery level as a strongly typed [`BatteryLevel`] enum.
    ///
    /// # Returns
    /// * `Ok(BatteryLevel)` on success.
    /// * `Err(UpowerError::InvalidBatteryLevel)` if the returned value is not recognized.
    /// * Propagates any error from the underlying interface.
    pub async fn get_battery_level(&self) -> Result<BatteryLevel, UpowerError> {
        match self.upower.get_battery_level().await {
            Ok(level) => {
                // Convert the raw value to the BatteryLevel enum, or return a descriptive error.
                let level = match BatteryLevel::try_from(level) {
                    Ok(level) => level,
                    Err(e) => return Err(UpowerError::InvalidBatteryLevel(e.into())),
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
        match self.upower.get_warning_level().await {
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
        self.upower.get_percentage().await.map_err(|e| e.into())
    }

    /// Asynchronously retrieves the current battery status as a [`BatteryState`] enum.
    ///
    /// # Returns
    /// * `Ok(BatteryState)` on success.
    /// * `Err(UpowerError::InvalidBatteryState)` if the returned value is not recognized.
    /// * Propagates any error from the underlying interface.
    pub async fn get_state(&self) -> Result<BatteryState, UpowerError> {
        match self.upower.get_state().await {
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
        match self.upower.get_power_source_type().await {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proxies::ProxyError;
    use anyhow::Result;
    use mockall::mock;

    // Mock the UpowerInterface trait
    mock! {
        pub UpowerInterface {}

        #[async_trait::async_trait]
        impl UpowerInterface for UpowerInterface {
            async fn get_battery_level(&self) -> Result<u32, ProxyError>;
            async fn get_warning_level(&self) -> Result<u32, ProxyError>;
            async fn get_percentage(&self) -> Result<f64, ProxyError>;
            async fn get_state(&self) -> Result<u32, ProxyError>;
            async fn get_power_source_type(&self) -> Result<u32, ProxyError>;
            async fn get_device_state_change_event(&self) -> Result<String, ProxyError>;
        }
    }

    #[tokio::test]
    async fn test_get_battery_level_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_battery_level().returning(|| Ok(3)); // 3 maps to BatteryLevel::Low

        let service = UpowerService::new(mock);
        let result = service.get_battery_level().await;
        assert_eq!(result.unwrap(), BatteryLevel::Low);
    }

    #[tokio::test]
    async fn test_get_battery_level_invalid_enum() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_battery_level().returning(|| Ok(42)); // 42 is not a valid BatteryLevel

        let service = UpowerService::new(mock);
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

        let service = UpowerService::new(mock);
        let result = service.get_battery_level().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "dbus error");
    }

    #[tokio::test]
    async fn test_get_warning_level_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_warning_level().returning(|| Ok(2)); // Assume 2 is a valid WarningLevel

        let service = UpowerService::new(mock);
        let result = service.get_warning_level().await;
        assert_eq!(result.unwrap(), WarningLevel::from(2));
    }

    #[tokio::test]
    async fn test_get_warning_level_error() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_warning_level()
            .returning(|| Err(ProxyError::DbusCallFailed("dbus error".into())));

        let service = UpowerService::new(mock);
        let result = service.get_warning_level().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_percentage_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_percentage().returning(|| Ok(77.7)); // Assume 77.7 is a valid percentage

        let service = UpowerService::new(mock);
        let result = service.get_percentage().await;
        assert_eq!(result.unwrap(), 77.7);
    }

    #[tokio::test]
    async fn test_get_state_success() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_state().returning(|| Ok(2)); // 2 maps to some BatteryState

        let service = UpowerService::new(mock);
        let result = service.get_state().await;
        assert_eq!(result.unwrap(), BatteryState::try_from(2).unwrap());
    }

    #[tokio::test]
    async fn test_get_state_invalid_enum() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_state().returning(|| Ok(99)); // Invalid BatteryState

        let service = UpowerService::new(mock);
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

        let service = UpowerService::new(mock);
        let result = service.get_power_source_type().await;
        assert_eq!(result.unwrap(), PowerSourceType::try_from(2).unwrap());
    }

    #[tokio::test]
    async fn test_get_power_source_type_invalid_enum() {
        let mut mock = MockUpowerInterface::new();
        mock.expect_get_power_source_type().returning(|| Ok(99)); // Invalid PowerSourceType

        let service = UpowerService::new(mock);
        let result = service.get_power_source_type().await;
        assert!(matches!(
            result.unwrap_err(),
            UpowerError::InvalidPowerSourceType
        ));
    }
}
