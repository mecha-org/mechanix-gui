use crate::events::{AppEvents, BtEvents};
use bluez::service::BluetoothService;
use futures::{SinkExt, StreamExt, channel::mpsc, select};

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

    let count = match bluetooth_manager.get_connected_devices().await {
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

pub async fn handle_bluetooth_toggle(mut bt_rx: mpsc::Receiver<BtEvents>) {
    let bluetooth_manager = match BluetoothService::new().await {
        Ok(bluetooth_manager) => bluetooth_manager,
        Err(e) => {
            eprintln!("Failed to create BluetoothService: {}", e);
            return;
        }
    };

    loop {
        select! {
            event = bt_rx.next() => {
                if let Some(event) = event  {
                    match event {
                            BtEvents::BluetoothToggle { enabled } => {
                            let _ = bluetooth_manager.toggle_bluetooth(enabled).await;
                        }
                    }
                }
            }
        }
    }
}
