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
        .send(AppEvents::BluetoothDevicesCount { count: count as u8 })
        .await;
}

pub async fn get_available_devices(
    mut tx: mpsc::Sender<AppEvents>,
    bluetooth_manager_service: &BluetoothService,
) {
    let discovery_durations = core::time::Duration::from_secs(5);

    match bluetooth_manager_service.get_available_devices(discovery_durations).await {
        Ok(devices) => {
            let _ = tx.send(AppEvents::AvailableBluetoothDevices { list: devices }).await;
        }
        Err(e) => {
            eprintln!("Failed to get available devices: {}", e);
            return;
        }
    };
}
