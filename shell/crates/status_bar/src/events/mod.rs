use upower::interfaces::device::BatteryState;

#[derive(Debug)]
pub enum AppEvents {
    WirelessStatusChanged { enabled: bool },
    WirelessStrength { strength: u8 },
    BluetoothEnabled { enabled: bool },
    BluetoothConnectionStatus { connected: bool },
    BatteryStateChanged { state: BatteryState },
    BatteryPercentageChanged { value: u8 },
    TimeUpdated,
}
