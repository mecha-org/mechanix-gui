//! High-level TimeDate Service Abstraction
//!
//! This module provides a high-level interface for managing system time and date settings
//! through a D-Bus interface. It wraps the low-level D-Bus calls in a more ergonomic API.
//!
//! # Examples
//!
//! ```ignore
//! use timedate_service::{TimeDateService, MockTimeDateInterface};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), TimeDateError> {
//!     let interface = MockTimeDateInterface::new();
//!     let service = TimeDateService::new(interface);
//!
//!     // Get current timezone
//!     let timezone = service.get_timezone().await?;
//!     println!("Current timezone: {}", timezone);
//!
//!     // Enable NTP synchronization
//!     service.set_ntp(true, false).await?;
//!
//!     Ok(())
//! }
//! ```

use crate::errors::TimeDateError;
use crate::interfaces::TimeDateInterface;

/// A high-level service for managing system time and date settings.
///
/// This struct provides methods to interact with system time settings including:
/// - Timezone management
/// - NTP synchronization
/// - System time configuration
/// - RTC (Real-Time Clock) settings
#[derive(Debug)]
pub struct TimeDateService<T: TimeDateInterface> {
    td: T,
}

impl<T: TimeDateInterface> TimeDateService<T> {
    /// Creates a new TimeDate service instance.
    ///
    /// # Arguments
    ///
    /// * `td` - An implementation of the `TimeDateInterface` trait
    ///
    pub fn new(td: T) -> Self {
        Self { td }
    }

    /// Returns a list of available time zones.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<String>)` - A list of available timezone names (e.g., "Europe/Berlin")
    /// * `Err(TimeDateError)` - If the operation fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let zones = service.list_time_zones().await?;
    /// for zone in zones {
    ///     println!("Available timezone: {}", zone);
    /// }
    /// ```
    pub async fn list_time_zones(&self) -> Result<Vec<String>, TimeDateError> {
        self.td.list_time_zones().await.map_err(TimeDateError::from)
    }

    /// Sets the system timezone.
    ///
    /// # Arguments
    ///
    /// * `timezone` - The timezone name (e.g., "Europe/Berlin"). Must be a valid timezone
    ///                from `/usr/share/zoneinfo/zone.tab`
    /// * `interactive` - If true, may prompt for authentication
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Set timezone to UTC non-interactively
    /// service.set_timezone("UTC", false).await?;
    /// ```
    pub async fn set_timezone(
        &self,
        timezone: &str,
        interactive: bool,
    ) -> Result<(), TimeDateError> {
        self.td
            .set_timezone(timezone, interactive)
            .await
            .map_err(TimeDateError::from)
    }

    /// Gets the current system timezone.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The current timezone name (e.g., "UTC")
    /// * `Err(TimeDateError)` - If the operation fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let current_tz = service.get_timezone().await?;
    /// println!("Current timezone: {}", current_tz);
    /// ```
    pub async fn get_timezone(&self) -> Result<String, TimeDateError> {
        self.td.get_timezone().await.map_err(TimeDateError::from)
    }

    /// Configures the RTC (Real-Time Clock) time keeping.
    ///
    /// # Arguments
    ///
    /// * `local_rtc` - If true, RTC is maintained in local time rather than UTC
    /// * `fix_system` - If true, system time will be synchronized from RTC
    /// * `interactive` - If true, may prompt for authentication
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Configure RTC to use UTC and sync system time
    /// service.set_local_rtc(false, true, false).await?;
    /// ```
    pub async fn set_local_rtc(
        &self,
        local_rtc: bool,
        fix_system: bool,
        interactive: bool,
    ) -> Result<(), TimeDateError> {
        self.td
            .set_local_rtc(local_rtc, fix_system, interactive)
            .await
            .map_err(TimeDateError::from)
    }

    /// Controls system clock synchronization with NTP.
    ///
    /// Enables or disables the systemd-timesyncd service for network time synchronization.
    ///
    /// # Arguments
    ///
    /// * `use_ntp` - If true, enables NTP synchronization
    /// * `interactive` - If true, may prompt for authentication
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Enable NTP synchronization
    /// service.set_ntp(true, false).await?;
    /// ```
    pub async fn set_ntp(
        &self,
        use_ntp: bool,
        interactive: bool,
    ) -> Result<(), TimeDateError> {
        self.td.set_ntp(use_ntp, interactive).await.map_err(TimeDateError::from)
    }

    /// Sets the system time.
    ///
    /// # Arguments
    ///
    /// * `usec_utc` - Microseconds since the UNIX epoch (1 Jan 1970 UTC)
    /// * `relative` - If true, `usec_utc` is added to current time
    /// * `interactive` - If true, may prompt for authentication
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Set absolute time to Jan 1, 2025 00:00:00 UTC
    /// let time_2025 = 1735669800;
    /// service.set_time(time_2025, false, false).await?;
    ///
    /// // Adjust time relatively by adding 1 second
    /// service.set_time(1_000_000, true, false).await?;
    /// ```
    pub async fn set_time(
        &self,
        usec_utc: i64,
        relative: bool,
        interactive: bool,
    ) -> Result<(), TimeDateError> {
        self.td
            .set_time(usec_utc, relative, interactive)
            .await
            .map_err(TimeDateError::from)
    }

    /// Gets the current system time in microseconds since the UNIX epoch.
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` - Current time in microseconds since the UNIX epoch
    /// * `Err(TimeDateError)` - If the operation fails
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let current_time = service.get_time_usec().await?;
    /// println!("Current time (µs since epoch): {}", current_time);
    /// ```
    pub async fn get_time_usec(&self) -> Result<u64, TimeDateError> {
        self.td.get_time_usec().await.map_err(TimeDateError::from)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    use crate::proxies::ProxyError;

    mock! {
        TimeDateImpl {}
        #[async_trait]
        impl TimeDateInterface for TimeDateImpl {
            async fn list_time_zones(&self) -> anyhow::Result<Vec<String>, ProxyError>;
            async fn set_local_rtc(&self, local_rtc: bool, fix_system: bool, interactive: bool) -> anyhow::Result<(), ProxyError>;
            async fn set_ntp(&self, use_ntp: bool, interactive: bool) -> anyhow::Result<(), ProxyError>;
            async fn set_time(&self, usec_utc: i64, relative: bool, interactive: bool) -> anyhow::Result<(), ProxyError>;
            async fn set_timezone(&self, timezone: &str, interactive: bool) -> anyhow::Result<(), ProxyError>;
            async fn get_timezone(&self) -> anyhow::Result<String, ProxyError>;
            async fn get_time_usec(&self) -> anyhow::Result<u64, ProxyError>;
        }
    }

    #[tokio::test]
    async fn test_get_timezone() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_get_timezone()
            .times(1)
            .returning(|| Ok("UTC".to_string()));

        let service = TimeDateService::new(mock);
        assert_eq!(service.get_timezone().await.unwrap(), "UTC");
    }
    #[tokio::test]
    async fn test_set_timezone() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_set_timezone()
            .times(1)
            .with(eq("Europe/Berlin"), eq(false))
            .returning(|_, _| Ok(()));

        let service = TimeDateService::new(mock);
        service.set_timezone("Europe/Berlin", false).await.unwrap();
    }
    #[tokio::test]
    async fn test_list_time_zones() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_list_time_zones()
            .times(1)
            .returning(|| Ok(vec!["UTC".to_string(), "Europe/Berlin".to_string()]));

        let service = TimeDateService::new(mock);
        let zones = service.list_time_zones().await.unwrap();
        assert!(zones.contains(&"UTC".to_string()));
        assert!(zones.contains(&"Europe/Berlin".to_string()));
    }
    #[tokio::test]
    async fn test_set_ntp() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_set_ntp()
            .times(1)
            .with(eq(true), eq(false))
            .returning(|_, _| Ok(()));

        let service = TimeDateService::new(mock);
        service.set_ntp(true, false).await.unwrap();
    }
    #[tokio::test]
    async fn test_set_local_rtc() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_set_local_rtc()
            .times(1)
            .with(eq(false), eq(true), eq(false))
            .returning(|_, _, _| Ok(()));

        let service = TimeDateService::new(mock);
        service.set_local_rtc(false, true, false).await.unwrap();
    }
    #[tokio::test]
    async fn test_set_time() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_set_time()
            .times(1)
            .with(eq(1735669800), eq(false), eq(false))
            .returning(|_, _, _| Ok(()));

        let service = TimeDateService::new(mock);
        service.set_time(1735669800, false, false).await.unwrap();
    }
    #[tokio::test]
    async fn test_get_time_usec() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_get_time_usec()
            .times(1)
            .returning(|| Ok(1735669800000000));

        let service = TimeDateService::new(mock);
        assert_eq!(service.get_time_usec().await.unwrap(), 1735669800000000);
    }
    #[tokio::test]
    async fn test_set_time_relative() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_set_time()
            .times(1)
            .with(eq(1_000_000), eq(true), eq(false))
            .returning(|_, _, _| Ok(()));

        let service = TimeDateService::new(mock);
        service.set_time(1_000_000, true, false).await.unwrap();
    }
    #[tokio::test]
    async fn test_set_time_error() {
        let mut mock = MockTimeDateImpl::new();
        mock.expect_set_time()
            .times(1)
            .with(eq(1735669800), eq(false), eq(false))
            .returning(|_, _, _| Err(ProxyError::DbusCallFailed("".to_string())));

        let service = TimeDateService::new(mock);
        let result = service.set_time(1735669800, false, false).await;
        assert!(result.is_err());
    }
}