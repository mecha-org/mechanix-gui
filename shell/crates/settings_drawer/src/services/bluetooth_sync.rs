use crate::events::AppEvents;
use bluez::service::BluetoothService;
use futures::{SinkExt, channel::mpsc};

pub async fn sync_bluetooth_connected_status(
    mut tx: mpsc::Sender<AppEvents>,
    bluetooth_manager_service: &BluetoothService,
) {
    let count = match bluetooth_manager_service.get_connected_devices().await {
        Ok(r) => r.len(),
        Err(e) => {
            eprintln!("Failed to get connected devices: {}", e);
            return;
        }
    };

    let _ = tx
        .send(AppEvents::BluetoothDevices { count: count as u8 })
        .await;
}
