use crate::messages::ShellStateMessage;
use bluez::service::BluetoothService;
use chrono::Local;
use futures::{SinkExt, channel::mpsc};
use networkmanager::service::NetworkManagerService;
use pulseaudio::service::PulseAudioService;

pub const MAX_DEVICE_BRIGHTNESS: u32 = 254;
pub const DEFAULT_MIN_BRIGHTNESS: f32 = 10.;

pub fn get_current_datetime() -> String {
    let now = Local::now();
    format!("{}", now.format("%H:%M"))
}

pub async fn sync_connected_network(
    mut message_tx: mpsc::Sender<ShellStateMessage>,
    network_manager_service: &NetworkManagerService,
) {
    match network_manager_service.list_networks().await {
        Ok(result) => {
            let mut sorted_list = result.clone();
            sorted_list.sort_by_key(|n| (!n.is_active, !n.is_known));
            sorted_list.retain(|n| !n.ssid.is_empty());

            let _ = message_tx
                .send(ShellStateMessage::ListWirelessNetworks { list: sorted_list })
                .await;

            let connected_network = result.iter().find(|n| n.is_active).cloned();
            let is_connected = result.iter().any(|n| n.is_active && !n.ssid.is_empty()); // IMP

            let _ = message_tx
                .send(ShellStateMessage::ConnectedNetwork {
                    network: if is_connected {
                        connected_network.clone()
                    } else {
                        None
                    },
                })
                .await;
        }
        Err(e) => {
            eprintln!("Failed to get active network: {}", e);
            return;
        }
    };
}

pub async fn get_available_bluetooth_devices(
    mut message_tx: mpsc::Sender<ShellStateMessage>,
    bluetooth_manager_service: &BluetoothService,
) {
    let discovery_durations = std::time::Duration::from_secs(5);

    match bluetooth_manager_service
        .get_available_devices(discovery_durations)
        .await
    {
        Ok(devices) => {
            let _ = message_tx
                .send(ShellStateMessage::AvailableBluetoothDevices { list: devices })
                .await;
        }
        Err(e) => {
            eprintln!("Failed to get available devices: {}", e);
            return;
        }
    };
}

pub async fn sync_bluetooth_connected_status(
    mut message_tx: mpsc::Sender<ShellStateMessage>,
    bluetooth_manager_service: &BluetoothService,
) {
    let count = match bluetooth_manager_service.get_connected_devices().await {
        Ok(r) => r.len(),
        Err(e) => {
            eprintln!("Failed to get connected devices: {}", e);
            return;
        }
    };

    let _ = message_tx
        .send(ShellStateMessage::BluetoothDevices { count: count as u8 })
        .await;
}

pub async fn get_sound_device_info(
    tx: &mut mpsc::Sender<ShellStateMessage>,
    pulse_service: &PulseAudioService,
) {
    if let Ok(device_info) = pulse_service.handle.get_default_sink().await {
        let _ = tx
            .send(ShellStateMessage::OutputSoundDevice { device_info })
            .await;
    }
}

pub fn u8_to_percent(value: u8, max_u32: u32) -> f32 {
    value as f32 / max_u32 as f32 * 100.0
}
pub fn percent_to_u8(percent: f32, max_u32: u32) -> u8 {
    ((percent / 100.0) * max_u32 as f32).round() as u8
}
