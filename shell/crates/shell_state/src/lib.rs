use std::time::Duration;
use bluez::{interfaces::device::BluetoothDevice, service::{BluetoothEvent, BluetoothService, InterfaceEvent}};
use pulseaudio::service::{DeviceInfo, PulseAudioService};
use futures::{FutureExt, SinkExt, StreamExt, channel::mpsc, select};
use futures_timer::Delay;
use gpui::*;
use networkmanager::{
    interfaces::wireless::{EventType, NMState, WirelessNetworkInfo},
    service::NetworkManagerService,
};
use system_dbus::display_client;
use upower::{
    interfaces::device::{BatteryLevel, BatteryState},
    service::UPowerService,
};

pub mod messages;
pub mod helper;
pub use messages::*;
pub use helper::*;


#[derive(Debug, Default, Clone)]
pub struct WirelessDetails {
    pub enabled: bool,
    pub connected_network: Option<WirelessNetworkInfo>,
    pub networks: Option<Vec<WirelessNetworkInfo>>,
}

#[derive(Debug, Default, Clone)]
pub struct BluetoothDetails {
    pub enabled: bool,
    pub connected_devices: u8,
    pub available_devices: Option<Vec<BluetoothDevice>>,
}

#[derive(Debug, Default, Clone)]
pub struct ShellState {
    pub wireless_details: WirelessDetails,
    pub bluetooth_details: BluetoothDetails,
    pub current_time_date: String,
    pub battery_state: BatteryState,
    pub battery_level: BatteryLevel,
    pub battery_percent: u8,
    pub status_bar_entity: Option<EntityId>,
    pub default_sound_device: DeviceInfo,
    pub sound_devices: Vec<DeviceInfo>,
    pub brightness_value: f32,
    pub volume: f32,
    pub nm_tx: Option<mpsc::Sender<NmMessage>>,
    pub bt_tx: Option<mpsc::Sender<BtMessage>>,
    pub volume_tx: Option<mpsc::Sender<VolumeMessage>>,
    pub brightness_tx: Option<mpsc::Sender<BrightnessMessage>>,
}


impl Global for ShellState {}

impl ShellState {
    pub fn global(cx: &App) -> &ShellState {
        cx.global::<ShellState>()
    }

    pub fn global_mut(cx: &mut App) -> &mut ShellState {
        cx.global_mut::<ShellState>()
    }
}

impl ShellState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub async fn toggle_wireless(&self, enabled: bool) {
        let _ = self.nm_tx
            .clone()
            .unwrap()
            .send(NmMessage::ToggleWireless { enabled: enabled }).await;
    }

    pub async fn toggle_bluetooth(&self, enabled: bool) {
        let _ = self.bt_tx.clone().unwrap().send(BtMessage::ToggleBluetooth { enabled: enabled }).await;
    }

    pub async fn get_output_sound_devices(&self) {
        let _ = self.volume_tx.clone().unwrap().send(VolumeMessage::RequestOutputSounds).await;
    }
}

pub struct ShellStateManager {}

impl ShellStateManager {
    pub fn new() -> Self {
        Self {}
    }


    pub fn run(cx: &mut App) {
        let (mut message_tx, mut message_rx) = mpsc::channel::<ShellStateMessage>(210);

        let (nm_tx, mut nm_rx) = mpsc::channel::<NmMessage>(210);
        let (bt_tx, mut bt_rx) = mpsc::channel::<BtMessage>(210);
        let (volume_tx, mut volume_rx) = mpsc::channel::<VolumeMessage>(210);
        let (brightness_tx, mut brightness_rx) = mpsc::channel::<BrightnessMessage>(210);

        // ShellState::global_mut(cx).message_tx = Some(message_tx.clone());
        ShellState::global_mut(cx).nm_tx = Some(nm_tx.clone());
        ShellState::global_mut(cx).bt_tx = Some(bt_tx.clone());
        ShellState::global_mut(cx).volume_tx = Some(volume_tx.clone());
        ShellState::global_mut(cx).brightness_tx = Some(brightness_tx.clone());

        cx.spawn(async move |app| {
            while let Some(message) = message_rx.next().await {
                _ = app.update(|cx| {
                    match message {
                        ShellStateMessage::WirelessStatusChanged { enabled } => {
                            ShellState::global_mut(cx).wireless_details.enabled = enabled;
                            if !enabled {
                                ShellState::global_mut(cx)
                                    .wireless_details
                                    .connected_network = None;
                            }
                        }
                        ShellStateMessage::WirelessStrength { strength } => {
                            if let Some(mut connected_network) = ShellState::global(cx)
                                .wireless_details
                                .connected_network
                                .clone()
                            {
                                connected_network.signal_strength = strength;
                                ShellState::global_mut(cx)
                                    .wireless_details
                                    .connected_network = Some(connected_network);
                            }
                        }
                        ShellStateMessage::ConnectedNetwork { network } => {
                            ShellState::global_mut(cx)
                                .wireless_details
                                .connected_network = network;
                        }
                        ShellStateMessage::ListWirelessNetworks { list } => {
                            ShellState::global_mut(cx)
                                .wireless_details
                                .networks = Some(list);
                        }
                        ShellStateMessage::BluetoothEnabled { enabled } => {
                            ShellState::global_mut(cx).bluetooth_details.enabled = enabled;
                        }
                        ShellStateMessage::BluetoothDevices { count } => {
                            ShellState::global_mut(cx)
                                .bluetooth_details
                                .connected_devices = count;
                        }
                        ShellStateMessage::AvailableBluetoothDevices { list } => {
                            ShellState::global_mut(cx)
                                .bluetooth_details
                                .available_devices = Some(list);
                        }
                        ShellStateMessage::AddWirelessNetwork { network } => {
                            let current_networks = ShellState::global_mut(cx)
                                .wireless_details
                                .networks
                                .get_or_insert_with(Vec::new);

                            // Check if network with same SSID already exists
                            let exists = current_networks
                                .iter()
                                .any(|n| n.ssid == network.ssid);

                            if !exists {
                                current_networks.push(network);
                            } else {
                                // Update existing network (in case signal strength changed)
                                if let Some(existing) = current_networks
                                    .iter_mut()
                                    .find(|n| n.ssid == network.ssid)
                                {
                                    *existing = network;
                                }
                            }
                        }
                        ShellStateMessage::RemoveWirelessNetwork { ssid } => {
                            if let Some(networks) = &mut ShellState::global_mut(cx)
                                .wireless_details
                                .networks
                            {
                                networks.retain(|n| n.ssid != ssid);
                            }
                        }
                        ShellStateMessage::BluetoothAddedEvent { device } => {
                            let current_devices = ShellState::global_mut(cx)
                                .bluetooth_details
                                .available_devices.clone();
                            if let Some(current_devices) = current_devices {
                                let mut available_devices = current_devices.clone();
                                available_devices.push(device);
                                ShellState::global_mut(cx)
                                    .bluetooth_details
                                    .available_devices = Some(available_devices);
                            }
                        }
                        ShellStateMessage::BatteryStateChanged { state } => {
                            ShellState::global_mut(cx).battery_state = state;
                        }
                        ShellStateMessage::BatteryLevelChanged { level } => {
                            ShellState::global_mut(cx).battery_level = level;
                        }
                        ShellStateMessage::BatteryPercentageChanged { value } => {
                            ShellState::global_mut(cx).battery_percent = value;
                        }
                        ShellStateMessage::TimeUpdated => {
                            let current_time_date = get_current_datetime();
                            ShellState::global_mut(cx).current_time_date = current_time_date;
                            if let Some(entity) = ShellState::global(cx).status_bar_entity {
                                cx.notify(entity);
                            }
                        }
                        ShellStateMessage::OutputSoundDevice { device_info } => {
                            ShellState::global_mut(cx).default_sound_device = device_info.clone();
                            ShellState::global_mut(cx).volume = device_info.clone().volume as f32;
                        }
                        ShellStateMessage::OutputSounds { list } => {
                            ShellState::global_mut(cx).sound_devices = list;
                        }
                        ShellStateMessage::Brightness { value } => {
                            ShellState::global_mut(cx).brightness_value = value;
                        }
                        _ => {}
                    };
                });
            }
        })
            .detach();

        let executor = cx.background_executor();
        executor
            .spawn(async move {
                let event_after = Duration::from_secs(10);
                let mut time_event = Delay::new(event_after).fuse();
                let _ = message_tx.send(ShellStateMessage::TimeUpdated).await;

                let network_manager = NetworkManagerService::new().await.unwrap();

                let bluetooth_manager = BluetoothService::new().await.unwrap();
                let battery_manager = UPowerService::new().await.unwrap();

                let mut battery_status_stream = battery_manager.stream_device_state().await;
                let mut battery_level_stream = battery_manager.stream_battery_level().await;
                let mut battery_percentage_stream =
                    battery_manager.stream_device_percentage().await;

                let mut enable_state_stream =
                    network_manager.stream_wireless_enabled_status().await;
                let mut device_state_stream = network_manager.stream_device_events().await;
                let mut strength_stream = network_manager.stream_active_network_strength().await;
                let mut access_point_stream = network_manager.stream_access_point_events().await;

                let mut bluetooth_status_stream =
                    bluetooth_manager.stream_bluetooth_enabled_status().await;
                let mut bluetooth_device_stream =
                    bluetooth_manager.stream_bluetooth_device_status().await;

                let _ = get_available_bluetooth_devices(message_tx.clone(), &bluetooth_manager).await;

                let pulse_manager = PulseAudioService::new().unwrap();
                let _ = get_sound_device_info(&mut message_tx, &pulse_manager).await;

                let brightness_value = match display_client::get_brightness().await {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("Error getting brightness: {}", e);
                        0
                    }
                };

                let brightness_percent = if brightness_value > 0 {
                    u8_to_percent(brightness_value, MAX_DEVICE_BRIGHTNESS)
                } else {
                    MAX_DEVICE_BRIGHTNESS as f32
                };
                let _ = message_tx.send(ShellStateMessage::Brightness { value: brightness_percent }).await;


                loop {
                    select! {

                        // battery events
                        battery_level = battery_level_stream.next() => {
                            if let Some(level) = battery_level {
                                let _ = message_tx.send(ShellStateMessage::BatteryLevelChanged { level: level }).await;
                            }
                        },

                        battery_state = battery_status_stream.next() => {
                            if let Some(state) = battery_state {
                                let _ = message_tx.send(ShellStateMessage::BatteryStateChanged { state }).await;
                            }
                        },

                        battery_percentage = battery_percentage_stream.next() => {
                            if let Some(percentage) = battery_percentage {
                                let value = percentage as u8;
                                let _ = message_tx.send(ShellStateMessage::BatteryPercentageChanged { value }).await;
                            }
                        }

                        // network events
                        strength = strength_stream.next() => {
                            if let Some(strength) = strength {
                                let _ = message_tx.send(ShellStateMessage::WirelessStrength { strength }).await;
                            }
                        },

                        enable_state = enable_state_stream.next() => {
                            if let Some(is_enabled) = enable_state {
                                let _ = message_tx.send(ShellStateMessage::WirelessStatusChanged { enabled: is_enabled }).await;
                            }
                        },

                        device_state = device_state_stream.next() => {
                            if let Some(nm_state) = device_state {
                            match nm_state {
                                    NMState::ConnectedGlobal | NMState::ConnectedLocal=> {
                                        let _ = sync_connected_network(message_tx.clone(), &network_manager.clone()).await;
                                    }
                                     _ => {}
                                }
                            }
                        }

                        ap_event = access_point_stream.next() => {
                            match ap_event {
                                Some(Ok(event)) => {
                                    match event.event_type {
                                        EventType::Added => {
                                            if let Some(new_network) = event.wireless_network_info {
                                                let _ = message_tx.send(
                                                    ShellStateMessage::AddWirelessNetwork { 
                                                        network: new_network 
                                                    }
                                                ).await;
                                            }
                                        }
                                        EventType::Removed => {
                                            if let Some(removed_network) = event.wireless_network_info {
                                                println!("network is removed : {:?}", removed_network.clone()  );
                                                let _ = message_tx.send(
                                                    ShellStateMessage::RemoveWirelessNetwork { 
                                                        ssid: removed_network.ssid 
                                                    }
                                                ).await;
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                  
                        event = nm_rx.next() => {
                            if let Some(event) = event {
                                match event {
                                   NmMessage::ToggleWireless { enabled } => {
                                        let _ = network_manager.toggle_wireless(enabled).await;
                                   }
                                   NmMessage::ConnectKnownNetwork { name } => {
                                       let _ = network_manager.connect_to_saved_network(&name.clone()).await;
                                   }
                                }
                            }
                        }


                        // // bluetooth events
                        bluetooth_enabled = bluetooth_status_stream.next() => {
                            if let Some(enabled) = bluetooth_enabled {
                                let _ = message_tx.send(ShellStateMessage::BluetoothEnabled { enabled }).await;
                            }
                        },

                        bluetooth_device_event = bluetooth_device_stream.next() => {
                            if let Some(event) = bluetooth_device_event {
                                match event {
                                    BluetoothEvent { event: InterfaceEvent::DeviceAdded, device: Some(device) } => {
                                        let _ = message_tx.send(ShellStateMessage::BluetoothAddedEvent { device: device }).await;
                                    }
                                    BluetoothEvent { event: InterfaceEvent::DeviceRemoved, device: Some(_) } => {
                                        let _ = get_available_bluetooth_devices(message_tx.clone(), &bluetooth_manager).await;
                                    }
                                    _ => {
                                        let _ = sync_bluetooth_connected_status(message_tx.clone(), &bluetooth_manager).await;
                                    }
                                }
                            }
                        }

                        event = bt_rx.next() => {
                            if let Some(event) = event {
                                match event {
                                    BtMessage::ToggleBluetooth { enabled } => {
                                        let _ = bluetooth_manager.toggle_bluetooth(enabled).await;
                                    }
                                    BtMessage::ConnectDevice { address } => {
                                                let _ = match bluetooth_manager.connect(&address).await {
                                                    Ok(_) => {
                                                       println!("Connected to device: {:?}", address);
                                                    }
                                                    Err(e) => {
                                                        eprintln!("Failed to connect to device: {}", e);
                                                    }
                                                };
                                            }
                                }
                            }
                        }


                        // volume events
                        volume_event = volume_rx.next() => {
                            match volume_event {
                                Some(VolumeMessage::VolumeChanged { name, value }) => {
                                    match pulse_manager.handle.set_sink_volume_by_name(&name, &value).await {
                                        Ok(_) => {
                                            get_sound_device_info(&mut message_tx, &pulse_manager).await;
                                        },
                                        Err(e) => {
                                            eprintln!("Failed to set volume: {}", e);
                                        }
                                    }
                                }
                                Some(VolumeMessage::MuteSink { name }) => {
                                    match pulse_manager.handle.set_sink_mute_by_name(&name).await {
                                        Ok(_) => {
                                            get_sound_device_info(&mut message_tx, &pulse_manager).await;
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to set mute: {}", e);
                                        }
                                    }
                                }
                                Some(VolumeMessage::UnmuteSink { name }) => {
                                    match pulse_manager.handle.set_sink_unmute_by_name(&name).await {
                                        Ok(_) => {
                                        get_sound_device_info(&mut message_tx, &pulse_manager).await; 
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to unset mute: {}", e);
                                        }
                                    }
                                }
                                Some(VolumeMessage::RequestOutputSounds) => {
                                    match pulse_manager.handle.get_sinks().await {
                                        Ok(list) => {
                                               let _ = message_tx.send(ShellStateMessage::OutputSounds { list }).await;
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to unset mute: {}", e);
                                        }
                                    }
                                }
                                 Some(VolumeMessage::SetDefaultOutputSoundDevice { name }) => {
                                    match pulse_manager.handle.set_default_sink_by_name(&name).await {
                                        Ok(_) => {
                                            let _= get_sound_device_info(&mut message_tx, &pulse_manager).await;
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to unset mute: {}", e);
                                        }
                                    }
                                }

                                None => break,
                            }
                        
                        }

                        // brightness events
                        brightness_event = brightness_rx.next() => {
                        match brightness_event {
                            Some(BrightnessMessage::BrightnessChanged { value }) => {

                                let value = if value <= DEFAULT_MIN_BRIGHTNESS { DEFAULT_MIN_BRIGHTNESS } else { value };
                                let value_u8 = percent_to_u8(value.clone(), MAX_DEVICE_BRIGHTNESS);
                                match display_client::set_brightness(value_u8).await {
                                    Ok(_) => (),
                                    Err(e) => {
                                        eprintln!("Failed to set brightness: {}", e);
                                    }
                                }
                            }
                            None => break,
                            }
                        }
                        
                        _ = time_event => {
                            let _ = message_tx.send(ShellStateMessage::TimeUpdated).await;
                            time_event = Delay::new(event_after).fuse();
                        }

                    }
                }
            })
            .detach();
    }
}

pub fn init(cx: &mut App) {
    let shell_state = ShellState::new();
    cx.set_global(shell_state);
    ShellStateManager::run(cx);
}

pub mod prelude {
    pub use crate::{ShellState, init};
}
