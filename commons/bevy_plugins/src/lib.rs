pub mod bluetooth;
pub mod network_manager;
pub mod pulse_audio;
pub mod upower;
mod universal_search;

pub use crate::bluetooth::BluetoothPlugin;
pub use crate::network_manager::NetworkManagerPlugin;
pub use crate::pulse_audio::PulseAudioPlugin;
pub use freedesktop_upower_client::interfaces::device::BatteryState as UPowerBatteryState;
pub use freedesktop_upower_client::interfaces::device::BatteryLevel as UPowerBatteryLevel;
pub use crate::universal_search::UniversalSearchPlugin;
