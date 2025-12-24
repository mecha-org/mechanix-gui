use bluez::interfaces::device::BluetoothDevice;
use networkmanager::interfaces::wireless::{AccessPointEvent, WirelessNetworkInfo};
use pulseaudio::service::DeviceInfo;

#[derive(Debug)]
pub enum AppEvents {
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
    BluetoothDevicesCount {
        count: u8,
    },
    BluetoothAddedEvent {
        device: BluetoothDevice,
    },
    AvailableBluetoothDevices {
        list: Vec<BluetoothDevice>,
    },
    OutputSoundDevice {
        device_info: DeviceInfo,
    },
    Brightness {
        value: f32,
    },
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
