pub mod bluetooth_sync;
pub mod network_sync;
pub mod battery_sync;
pub mod brightness_sync;
pub mod sound_sync;

pub use bluetooth_sync::{
    handle_bluetooth_toggle, stream_bluetooth_device_status, sync_bluetooth_connected_status,
    sync_bluetooth_status,
};
pub use network_sync::{
    handle_wireless_toggle, stream_network_device_events, sync_connected_network, sync_network_status,
    sync_network_strength,
};
pub use battery_sync::{sync_battery_level, sync_battery_state, sync_battery_percentage};
pub use sound_sync::audio_event_handler;
pub use brightness_sync::brightness_event_handler;