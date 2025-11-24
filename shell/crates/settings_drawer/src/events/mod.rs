use networkmanager::interfaces::wireless::WirelessNetworkInfo;
use pulseaudio::service::DeviceInfo;

#[derive(Debug)]
pub enum AppEvents {
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
}

#[derive(Debug)]
pub enum NmEvents {
    WirelessToggle { enabled: bool },
}

#[derive(Debug)]
pub enum BtEvents {
    BluetoothToggle { enabled: bool },
}
