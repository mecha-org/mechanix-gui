use std::{fmt, io, str::FromStr};
pub use upower::BatteryStatus;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WirelessConnectedState {
    Low,
    Weak,
    Good,
    Strong,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WirelessStatus {
    #[default]
    On,
    Off,
    Connected(WirelessConnectedState),
    NotFound,
}

impl fmt::Display for WirelessStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WirelessStatus::On => write!(f, "WirelessOn"),
            WirelessStatus::Off => write!(f, "WirelessOff"),
            WirelessStatus::Connected(state) => write!(f, "WirelessConnected({:?})", state),
            WirelessStatus::NotFound => write!(f, "WirelessNotFound"),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BluetoothStatus {
    On,
    #[default]
    Off,
    Connected,
    NotFound,
}

impl fmt::Display for BluetoothStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BluetoothStatus::On => write!(f, "BluetoothOn"),
            BluetoothStatus::Off => write!(f, "BluetoothOff"),
            BluetoothStatus::Connected => write!(f, "BluetoothConnected"),
            BluetoothStatus::NotFound => write!(f, "BluetoothNotFound"),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BatteryLevel {
    #[default]
    Level0,
    Level10,
    Level20,
    Level30,
    Level40,
    Level50,
    Level60,
    Level70,
    Level80,
    Level90,
    Level100,
    ChargingLevel0,
    ChargingLevel10,
    ChargingLevel20,
    ChargingLevel30,
    ChargingLevel40,
    ChargingLevel50,
    ChargingLevel60,
    ChargingLevel70,
    ChargingLevel80,
    ChargingLevel90,
    ChargingLevel100,
    NotFound,
}

impl fmt::Display for BatteryLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ShutdownState {
    Pressed,
    Released,
    Clicked,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RestartState {
    Pressed,
    Released,
    Clicked,
}
