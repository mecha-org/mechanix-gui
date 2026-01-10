use bluez::interfaces::device::BluetoothDevice;
use networkmanager::interfaces::wireless::WirelessNetworkInfo;
use pulseaudio::service::DeviceInfo;
use upower::interfaces::device::{BatteryLevel, BatteryState};

pub enum ShellStateMessage {
    WirelessStatusChanged {
        enabled: bool,
    },
    WirelessStrength {
        strength: u8,
    },
    ConnectedNetwork {
        network: Option<WirelessNetworkInfo>,
    },
    ListWirelessNetworks {
        list: Vec<WirelessNetworkInfo>,
    },
    NetworkEnabled {
        enabled: bool,
    },
    BluetoothEnabled {
        enabled: bool,
    },
    BluetoothDevices {
        count: u8,
    },
    AvailableBluetoothDevices {
        list: Vec<BluetoothDevice>,
    },
    BluetoothAddedEvent {
        device: BluetoothDevice,
    },
    BatteryLevelChanged {
        level: BatteryLevel,
    },
    BatteryStateChanged {
        state: BatteryState,
    },
    BatteryPercentageChanged {
        value: u8,
    },
    TimeUpdated,
    OutputSoundDevice {
        device_info: DeviceInfo,
    },
    Brightness {
        value: f32,
    },
    AddWirelessNetwork {
        network: WirelessNetworkInfo,
    },
    RemoveWirelessNetwork {
        ssid: String,
    },
}

pub enum NmMessage {
    ToggleWireless { enabled: bool },
    ConnectKnownNetwork { name: String },
}

#[derive(Debug)]
pub enum BtMessage {
    ToggleBluetooth { enabled: bool },
    ConnectDevice { address: String },
}

#[derive(Debug)]
pub enum VolumeMessage {
    VolumeChanged { name: String, value: f32 },
    MuteSink { name: String },
    UnmuteSink { name: String },
}

#[derive(Debug)]
pub enum BrightnessMessage {
    BrightnessChanged { value: f32 },
}
