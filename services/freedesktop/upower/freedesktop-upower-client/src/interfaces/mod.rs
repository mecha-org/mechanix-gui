//! Asynchronous, type-safe interface for querying battery and power source information via UPower.
//!
//! ## Example
//! ``` ignore
//! use upower_rs::UpowerInterface;
//! #[async_trait]
//! impl UpowerInterface for MyDevice { /* ... */ }
//! ```
//!
//! ## Thread Safety
//! All trait implementations must be `Send + Sync` for safe concurrent use.
//!
//! ## Modules
//! - `device`: Strongly-typed enums and helpers for UPower device properties

use anyhow::Result;
use async_trait::async_trait;
use zbus::proxy::PropertyStream;
use crate::proxies::ProxyError;

pub mod device;

/// Asynchronous interface for interacting with UPower devices.
///
/// This trait defines the contract for querying battery and device state
/// information asynchronously. All methods return a `Result` type,
/// representing either the requested value or an error.
///
/// The trait is marked with `#[async_trait]` to enable async methods in traits
/// on stable Rust
///
/// # Thread Safety
/// - Implementors must be `Send + Sync` to allow safe concurrent use across threads.
///
/// # Usage
/// Implement this trait for any type that needs to provide UPower device information
/// asynchronously, such as a D-Bus proxy or a mock for testing.
///
/// # Example
/// ```ignore
/// #[async_trait]
/// impl UpowerInterface for MyDevice {
///     async fn get_battery_level(&self) -> Result<u32, ProxyError> { ... }
///     // ... other methods ...
/// }
/// ```
#[async_trait]
pub trait UPowerInterface: Send + Sync {
    /// Asynchronously get the current battery level as a percentage (0-100).
    async fn get_battery_level(&self) -> Result<u32, ProxyError>;

    /// Asynchronously get the battery warning level (e.g., low, critical).
    async fn get_warning_level(&self) -> Result<u32, ProxyError>;

    /// Asynchronously get the precise battery percentage (0.0-100.0).
    async fn get_percentage(&self) -> Result<f64, ProxyError>;

    /// Asynchronously get the battery power state (e.g., charging, discharging).
    async fn get_state(&self) -> Result<u32, ProxyError>;

    /// Asynchronously get the type of power source (e.g., battery, UPS).
    async fn get_power_source_type(&self) -> Result<u32, ProxyError>;

    /// Asynchronously get a device state change event as a string.
    async fn stream_device_state(&self) -> Result<PropertyStream<u32>, ProxyError>;
    async fn stream_device_percentage(&self) -> Result<PropertyStream<f64>, ProxyError>;
    async fn stream_battery_level(&self) -> Result<PropertyStream<u32>, ProxyError>;
}
