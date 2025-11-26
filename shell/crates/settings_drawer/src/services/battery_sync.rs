use crate::events::AppEvents;
use futures::{SinkExt, channel::mpsc};
use upower::service::UPowerService;

pub async fn sync_battery_level(mut tx: mpsc::Sender<AppEvents>) {
    let upower_manager = match UPowerService::new().await {
        Ok(upower_manager) => upower_manager,
        Err(e) => {
            eprintln!("Failed to create UpowerService: {}", e);
            return;
        }
    };

    let status_receiver = upower_manager.stream_device_percentage().await;

    while let Ok(percentage) = status_receiver.recv() {
          let level = percentage as u8;
        let _ = tx.send(AppEvents::BatteryLevelChanged { level }).await;
    }
}

pub async fn sync_battery_state(mut tx: mpsc::Sender<AppEvents>) {
    let upower_manager = match UPowerService::new().await {
        Ok(upower_manager) => upower_manager,
        Err(e) => {
            eprintln!("Failed to create UpowerService: {}", e);
            return;
        }
    };

    let status_receiver = upower_manager.stream_device_state().await;
    while let Ok(state) = status_receiver.recv() {
        let _ = tx.send(AppEvents::BatteryStateChanged { state }).await;
    }

}


pub async fn sync_battery_percentage(mut tx: mpsc::Sender<AppEvents>) {
    let upower_manager = match UPowerService::new().await {
        Ok(upower_manager) => upower_manager,
        Err(e) => {
            eprintln!("Failed to create UpowerService: {}", e);
            return;
        }
    };

    let status_receiver = upower_manager.stream_device_percentage().await;
    while let Ok(percentage) = status_receiver.recv() {
        let value = percentage as u8;
        let _ = tx.send(AppEvents::BatteryPercentageChanged { value }).await;
    }

}