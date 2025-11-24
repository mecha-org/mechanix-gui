pub mod network_sync;
pub mod bluetooth_sync;
pub mod battery_sync;

pub use network_sync::{sync_network_status, sync_network_strength};
pub use bluetooth_sync::{sync_bluetooth_status, sync_bluetooth_connected_status};
pub use battery_sync::{sync_battery_level, sync_battery_state};