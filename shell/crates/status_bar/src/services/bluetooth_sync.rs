use crate::events::AppEvents;
use bluez::service::BluetoothService;
use futures::{SinkExt, channel::mpsc};

pub async fn sync_bluetooth_status(mut tx: mpsc::Sender<AppEvents>) {
    let bluetooth_manager = match BluetoothService::new().await {
        Ok(bluetooth_manager) => bluetooth_manager,
        Err(e) => {
            eprintln!("Failed to create BluetoothService: {}", e);
            return;
        }
    };

    let status_receiver = bluetooth_manager.stream_bluetooth_enabled_status().await;

    while let Ok(enabled) = status_receiver.recv() {
        let _ = tx.send(AppEvents::BluetoothEnabled { enabled }).await;
    }
}

pub async fn sync_bluetooth_connected_status(mut tx: mpsc::Sender<AppEvents>) {
    let bluetooth_manager = match BluetoothService::new().await {
        Ok(bluetooth_manager) => bluetooth_manager,
        Err(e) => {
            eprintln!("Failed to create BluetoothService: {}", e);
            return;
        }
    };

    let connected_devices_count = match bluetooth_manager.get_connected_devices().await {
        Ok(r) => r.len(),
        Err(e) => {
            eprintln!("Failed to get connected devices: {}", e);
            return;
        }
    };
    let connected = connected_devices_count > 0;

    let _ = tx
        .send(AppEvents::BluetoothConnectionStatus { connected })
        .await;
}
