use commons::prelude::*;
use futures::{SinkExt, StreamExt, channel::mpsc, select};
use gpui::*;
use networkmanager::service::NetworkManagerService;
use bluez::service::BluetoothService;
use status_bar::{prelude::*, services::*};
use upower::service::UPowerService;
use std::time::Duration;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(540.0), px(36.0)), cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                ..Default::default()
            },
            |_window, cx| {
                let (mut app_channel_tx, mut app_channel_rx) = mpsc::channel::<AppEvents>(120);
                let executor = cx.background_executor();

                executor
                    .spawn(async move {
                        let network_manager = NetworkManagerService::new().await.unwrap();
                        let bluetooth_manager = BluetoothService::new().await.unwrap();
                        let battery_manager = UPowerService::new().await.unwrap();

                         let mut battery_status_stream = battery_manager.stream_device_state().await;
                        let mut battery_percentage_stream = battery_manager.stream_device_percentage().await;

                        let mut enable_state_stream = network_manager.stream_wireless_enabled_status().await;
                        let mut strength_stream = network_manager.stream_active_network_strength().await;

                        let mut bluetooth_status_stream = bluetooth_manager.stream_bluetooth_enabled_status().await;
                        let mut bluetooth_device_stream = bluetooth_manager.stream_bluetooth_device_status().await;
                     
                  
                        loop{
                            select!{
                                
                                // battery events
                                battery_state = battery_status_stream.next() => {
                                    if let Some(state) = battery_state {
                                        let _ = app_channel_tx.send(AppEvents::BatteryStateChanged { state }).await;
                                    }
                                },

                                battery_percentage = battery_percentage_stream.next() => {
                                    if let Some(percentage) = battery_percentage {
                                        let value = percentage as u8;
                                        let _ = app_channel_tx.send(AppEvents::BatteryPercentageChanged { value }).await;
                                    }
                                }

                                // network events
                                enable_state = enable_state_stream.next() => {
                                    if let Some(is_enabled) = enable_state {
                                        println!("from nm stream : {:?}", is_enabled);
                                        let _ = app_channel_tx.send(AppEvents::WirelessStatusChanged { enabled: is_enabled }).await;
                                    }
                                },

                                strength_stream = strength_stream.next() => {
                                    if let Some(strength) = strength_stream {
                                        println!("strength: {:?}", strength);
                                        let _ = app_channel_tx.send(AppEvents::WirelessStrength { strength }).await;
                                    }
                                }
                                

                                // bluetooth events
                                bluetooth_enabled = bluetooth_status_stream.next() => {
                                    if let Some(enabled) = bluetooth_enabled {
                                        let _ = app_channel_tx.send(AppEvents::BluetoothEnabled { enabled }).await;
                                    }
                                },

                                bluetooth_device_event = bluetooth_device_stream.next() => {
                                    if let Some(_event) = bluetooth_device_event {
                                        let _ = sync_bluetooth_connected_status(app_channel_tx.clone(), &bluetooth_manager).await;
                                    }
                                }

                            }
                        }
                  
                  
                    })
                    .detach();

                cx.new(|cx| {
                    cx.spawn(async move |app, cx| {
                        while let Some(event) = app_channel_rx.next().await {
                            match event {
                                AppEvents::WirelessStatusChanged { enabled } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.wireless_enabled = enabled;
                                        cx.notify();
                                    });
                                }
                                AppEvents::WirelessStrength { strength } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.wireless_strength = strength;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BluetoothEnabled { enabled } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.bluetooth_enabled = enabled;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BluetoothConnectionStatus { connected } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.bluetooth_connected = connected;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BatteryStateChanged { state } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.battery_state = state;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BatteryPercentageChanged { value } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.battery_percent = value;
                                        cx.notify();
                                    });
                                }
                                AppEvents::TimeUpdated => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.update_time(cx);
                                    });
                                }
                            }
                        }
                    })
                    .detach();

                    cx.spawn(async move |app, cx| {
                        loop {
                            cx.background_executor().timer(Duration::from_secs(1)).await;
                            let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                this.update_time(cx);
                            });
                        }
                    })
                    .detach();

                    StatusBar::new()
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
