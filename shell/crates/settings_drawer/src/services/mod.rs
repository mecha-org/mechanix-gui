pub mod bluetooth_sync;
pub mod network_sync;

pub use bluetooth_sync::{
    handle_bluetooth_toggle, sync_bluetooth_connected_status, sync_bluetooth_status,
};
pub use network_sync::{
    handle_wireless_toggle, sync_connected_network, sync_network_status, sync_network_strength,
};
