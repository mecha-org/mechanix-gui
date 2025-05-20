//! Bluetooth Device Properties Utilities
//!
//! This module defines the [`BluetoothDevice`] struct, which encapsulates common
//! Bluetooth device properties and provides utility methods for interfacing with
//! device property maps (such as those retrieved from D-Bus or BlueZ).
//!
//! Main features:
//! - Strongly-typed representation of Bluetooth device attributes.
//! - Convenient conversion from property maps to [`BluetoothDevice`].
//! - Handles missing or invalid properties gracefully.
//!
//! Intended for use in Bluetooth device management and interface layers.

/// Represents the properties of a Bluetooth device.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BluetoothDevice {
    /// The device's human-readable name.
    pub name: String,

    /// The user-assigned alias for the device.
    pub alias: String,

    /// The unique Bluetooth MAC address of the device.
    pub address: String,

    /// The type of Bluetooth address (e.g., "public" or "random").
    pub address_type: String,

    /// Indicates if the device uses legacy pairing.
    pub legacy_pairing: bool,

    /// The icon name representing the device type (if available).
    pub icon: String,

    /// Whether the device is marked as trusted.
    pub trusted: bool,

    /// Whether the device is paired with the system.
    pub paired: bool,

    /// Whether the device is currently connected.
    pub connected: bool,

    /// Indicates if the device is powered on (if this property is available).
    pub powered: Option<bool>,
}

impl BluetoothDevice {
    /// Constructs a `BluetoothDevice` from a property map, typically obtained from D-Bus.
    ///
    /// Returns `None` if the device has no valid name.
    pub fn from_properties(
        device_properties: &std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    ) -> Option<Self> {
        // Attempt to extract the "Name" property as a String.
        let name = device_properties
            .get("Name")
            .and_then(|v| v.downcast_ref::<String>().ok()) // Try to downcast to String
            .unwrap_or_default(); // Default to empty string if not found

        // Ensure the device has a valid (non-empty) name.
        if name.is_empty() {
            return None; // Return None if name is empty
        }

        Some(BluetoothDevice {
            // Set the device name (already extracted above).
            name,
            // Extract the device address, defaulting to empty string if missing.
            address: device_properties
                .get("Address")
                .and_then(|v| v.downcast_ref::<String>().ok())
                .unwrap_or_default(),
            // Extract the address type, defaulting to empty string if missing.
            address_type: device_properties
                .get("AddressType")
                .and_then(|v| v.downcast_ref::<String>().ok())
                .unwrap_or_default(),
            // Extract legacy pairing flag, defaulting to false if missing.
            legacy_pairing: device_properties
                .get("LegacyPairing")
                .and_then(|v| v.downcast_ref::<bool>().ok())
                .unwrap_or_default(),
            // Extract the alias, defaulting to empty string if missing.
            alias: device_properties
                .get("Alias")
                .and_then(|v| v.downcast_ref::<String>().ok())
                .unwrap_or_default(),
            // Extract the icon, defaulting to empty string if missing.
            icon: device_properties
                .get("Icon")
                .and_then(|v| v.downcast_ref::<String>().ok())
                .unwrap_or_default(),
            // Extract the trusted flag, defaulting to false if missing.
            trusted: device_properties
                .get("Trusted")
                .and_then(|v| v.downcast_ref::<bool>().ok())
                .unwrap_or_default(),
            // Extract the paired flag, defaulting to false if missing.
            paired: device_properties
                .get("Paired")
                .and_then(|v| v.downcast_ref::<bool>().ok())
                .unwrap_or_default(),
            // Extract the connected flag, defaulting to false if missing.
            connected: device_properties
                .get("Connected")
                .and_then(|v| v.downcast_ref::<bool>().ok())
                .unwrap_or_default(),
            // Extract the powered flag, wrapping in Some, defaulting to false if missing.
            powered: Some(
                device_properties
                    .get("Powered")
                    .and_then(|v| v.downcast_ref::<bool>().ok())
                    .unwrap_or_default(),
            ),
        })
    }
}
