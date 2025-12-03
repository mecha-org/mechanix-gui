use networkmanager::interfaces::wireless::{AccessPointEvent, WirelessNetworkInfo};
use pulseaudio::service::DeviceInfo;
use upower::interfaces::device::BatteryState;

#[derive(Debug)]
pub enum AppEvents {
    BatteryStateChanged {
        state: BatteryState,
    },
    BatteryPercentageChanged {
        value: u8,
    },
    WirelessStatusChanged {
        enabled: bool,
    },
    ConnectedNetwork {
        network: Option<WirelessNetworkInfo>,
    },
    ListWirelessNetworks {
        list: Vec<WirelessNetworkInfo>,
    },
    AccessPointEvent {
        event: AccessPointEvent,
    }, 
    BluetoothEnabled {
        enabled: bool,
    },
    BluetoothDevices {
        count: u8,
    },
    OutputSoundDevice {
        device_info: DeviceInfo,
    },
    Brightness {
        value: f32,
    },
}

#[derive(Debug)]
pub enum NmEvents {
    WirelessToggle { enabled: bool },
    ConnectKnownNetwork { name: String },
}

#[derive(Debug)]
pub enum BtEvents {
    BluetoothToggle { enabled: bool },
}

#[derive(Debug)]
pub enum VolumeEvents {
    VolumeChanged { name: String, value: f32 },
    MuteSink { name: String },
    UnmuteSink { name: String },
}

#[derive(Debug)]
pub enum BrightnessEvents {
    BrightnessChanged { value: f32 },
}