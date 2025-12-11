use std::time::Duration;

use bluez::service::BluetoothService;
use chrono::Local;
use futures::{FutureExt, SinkExt, StreamExt, channel::mpsc, select};
use futures_timer::Delay;
use gpui::*;
use networkmanager::{
    interfaces::wireless::{NMState, WirelessNetworkInfo},
    service::NetworkManagerService,
};
use upower::{
    interfaces::device::{BatteryLevel, BatteryState},
    service::UPowerService,
};

#[derive(Debug, Default, Clone)]
pub struct WirelessDetails {
    pub enabled: bool,
    pub connected_network: Option<WirelessNetworkInfo>,
}

#[derive(Debug, Default, Clone)]
pub struct BluetoothDetails {
    pub enabled: bool,
    pub connected_devices: u8,
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
}

pub struct ShellStateManager {}

impl ShellStateManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(cx: &mut App) {
        let (mut message_tx, mut message_rx) = mpsc::channel::<ShellStateMessage>(10);
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
                        ShellStateMessage::BluetoothEnabled { enabled } => {
                            ShellState::global_mut(cx).bluetooth_details.enabled = enabled;
                        }
                        ShellStateMessage::BluetoothDevices { count } => {
                            ShellState::global_mut(cx)
                                .bluetooth_details
                                .connected_devices = count;
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
                        ShellStateMessage::ConnectedNetwork { network } => {
                            ShellState::global_mut(cx)
                                .wireless_details
                                .connected_network = network;
                        }
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

                let mut bluetooth_status_stream =
                    bluetooth_manager.stream_bluetooth_enabled_status().await;
                let mut bluetooth_device_stream =
                    bluetooth_manager.stream_bluetooth_device_status().await;

                loop {
                    select!{

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
                                                match network_manager.list_networks().await {
        Ok(result) => {
            let connected_network = result.iter().find(|n| n.is_active && !n.ssid.is_empty()).cloned();
            let _ = message_tx
                .send(ShellStateMessage::ConnectedNetwork {
                    network: connected_network,
                })
                .await;
        }
        Err(e) => {
            eprintln!("Failed to get active network: {}", e);
            return;
        }
    };
                                            }
                                            _ => {}
                                        }
                                    }
                        }

                        // bluetooth events
                        bluetooth_enabled = bluetooth_status_stream.next() => {
                            if let Some(enabled) = bluetooth_enabled {
                                let _ = message_tx.send(ShellStateMessage::BluetoothEnabled { enabled }).await;
                            }
                        },

                        bluetooth_device_event = bluetooth_device_stream.next() => {
                            if let Some(_event) = bluetooth_device_event {
                                let count = match bluetooth_manager.get_connected_devices().await {
        Ok(r) => r.len(),
        Err(e) => {
            eprintln!("Failed to get connected devices: {}", e);
            return;
        }
    };

    let _ = message_tx.send(ShellStateMessage::BluetoothDevices { count: count as u8 }).await;
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

pub enum ShellStateMessage {
    WirelessStatusChanged {
        enabled: bool,
    },
    WirelessStrength {
        strength: u8,
    },
    ConnectedNetwork {
        network: Option<WirelessNetworkInfo>,
    },
    BluetoothEnabled {
        enabled: bool,
    },
    BluetoothDevices {
        count: u8,
    },
    BatteryLevelChanged {
        level: BatteryLevel,
    },
    BatteryStateChanged {
        state: BatteryState,
    },
    BatteryPercentageChanged {
        value: u8,
    },
    TimeUpdated,
}

pub fn get_current_datetime() -> String {
    let now = Local::now();
    format!("{}", now.format("%H:%M"))
}

pub fn init(cx: &mut App) {
    let shell_state = ShellState::new();
    cx.set_global(shell_state);
    ShellStateManager::run(cx);
}

pub mod prelude {
    pub use crate::{ShellState, init};
}
