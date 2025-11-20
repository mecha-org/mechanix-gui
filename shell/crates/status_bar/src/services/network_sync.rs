use crate::events::AppEvents;
use futures::{SinkExt, channel::mpsc};
use networkmanager::service::NetworkManagerService;

pub async fn sync_network_status(mut tx: mpsc::Sender<AppEvents>) {
    let network_manager = match NetworkManagerService::new().await {
        Ok(nm) => nm,
        Err(e) => {
            eprintln!("Failed to create NetworkManagerService: {}", e);
            return;
        }
    };
    let wireless_status_receiver = network_manager.stream_wireless_enabled_status().await;
    while let Ok(is_wireless_enabled) = wireless_status_receiver.recv() {
        let _ = tx
            .send(AppEvents::WirelessStatusChanged {
                enabled: is_wireless_enabled,
            })
            .await;
    }
}

pub async fn sync_network_strength(mut tx: mpsc::Sender<AppEvents>) {
    let network_manager = match NetworkManagerService::new().await {
        Ok(nm) => nm,
        Err(e) => {
            eprintln!("Failed to create NetworkManagerService: {}", e);
            return;
        }
    };

    let strength_receiver = network_manager.stream_active_network_strength().await;

    while let Ok(strength) = strength_receiver.recv() {
        let _ = tx.send(AppEvents::WirelessStrength { strength }).await;
    }
}
