use networkmanager::interfaces::wireless::WirelessNetworkInfo;
use pulseaudio::service::DeviceInfo;
use upower::interfaces::device::BatteryState;

#[derive(Debug)]
pub enum AppEvents {
    BatteryStateChanged {
        state: BatteryState,
    },
    BatteryLevelChanged {
        level: u8,
    },
    BatteryPercentageChanged {
        value: u8,
    },
    WirelessStatusChanged {
        enabled: bool,
    },
    WirelessStrength {
        strength: u8,
    },
    ConnectedNetwork {
        network: Option<WirelessNetworkInfo>,
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