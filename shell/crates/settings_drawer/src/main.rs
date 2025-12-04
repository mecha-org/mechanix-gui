use gpui::*;
use commons::prelude::*;
use futures::{SinkExt, StreamExt, channel::mpsc, select};
use networkmanager::interfaces::wireless::EventType;
use pulseaudio::service::PulseAudioService;
use system_dbus::display_client;
use upower::service::UPowerService;
use networkmanager::{interfaces::wireless::NMState, service::NetworkManagerService};
use bluez::service::BluetoothService;
use settings_drawer::prelude::*;
use settings_drawer::services::*;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(540.0), px(620.0)), cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                ..Default::default()
            },
            |_window, cx| {
                let (mut app_channel_tx, mut app_channel_rx) = mpsc::channel::<AppEvents>(120);
                let (nm_tx, mut nm_rx) = mpsc::channel::<NmEvents>(128);
                let (bt_tx, mut bt_rx) = mpsc::channel::<BtEvents>(128);
                let (volume_tx, mut volume_rx) = mpsc::channel::<VolumeEvents>(128);
                let (brightness_tx, mut brightness_rx) = mpsc::channel::<BrightnessEvents>(128);
                let executor = cx.background_executor();

                executor
                    .spawn(async move{

                        let battery_manager = UPowerService::new().await.unwrap();
                        let network_manager = NetworkManagerService::new().await.unwrap();
                        let bluetooth_manager = BluetoothService::new().await.unwrap();
                        
                        let mut battery_status_stream = battery_manager.stream_device_state().await;
                        let mut battery_percentage_stream = battery_manager.stream_device_percentage().await;

                        let mut enable_state_stream = network_manager.stream_wireless_enabled_status().await;
                        let mut device_state_stream = network_manager.stream_device_events().await;
                        let mut active_aceess_point_stream = network_manager.stream_access_point_events().await;

                        let mut bluetooth_status_stream = bluetooth_manager.stream_bluetooth_enabled_status().await;
                        let mut bluetooth_device_stream = bluetooth_manager.stream_bluetooth_device_status().await;
                        
                        let pulse_service = PulseAudioService::new().unwrap();
                        let _update_volume_info = update_device_info(&mut app_channel_tx, &pulse_service).await;

                        let brightness_value = match display_client::get_brightness().await {
                            Ok(value) => value,
                            Err(e) => {
                                eprintln!("Error getting brightness: {}", e);
                                0
                            }
                        };
                        let brightness_percent = if brightness_value > 0 {u8_to_percent(brightness_value, MAX_DEVICE_BRIGHTNESS) } else {0.0};
                        let _ = app_channel_tx.send(AppEvents::Brightness { value: brightness_percent }).await;


                        let list_networks = network_manager.list_networks().await.unwrap();
                        let _ = app_channel_tx.send(AppEvents::ListWirelessNetworks { list: list_networks } ).await;

                        loop {
                            select! {
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

                                event = nm_rx.next() => {
                                    if let Some(event) = event  {
                                        match event {
                                            NmEvents::WirelessToggle{enabled}=>{let _=network_manager.toggle_wireless(enabled).await;}
                                            NmEvents::ConnectKnownNetwork { name } => {
                                                let _ = network_manager.connect_to_saved_network(&name.clone()).await;
                                            },
                                        }
                                    }
                                }
                             
                                enable_state = enable_state_stream.next() => {
                                    if let Some(is_enabled) = enable_state {
                                        let _ = app_channel_tx.send(AppEvents::WirelessStatusChanged { enabled: is_enabled }).await;
                                    }
                                },

                                device_state = device_state_stream.next() => {
                                    if let Some(nm_state) = device_state {
                                    match nm_state {
                                            NMState::ConnectedGlobal | NMState::ConnectedLocal=> {
                                                let _ = sync_connected_network(app_channel_tx.clone(), &network_manager.clone()).await;
                                            }
                                            _ => {}
                                        }
                                    }
                                }

                                active_ap_event = active_aceess_point_stream.next() => {
                                   if let Some(event) = active_ap_event {
                                    // println!("event: {:?}", event);
                                       match event {
                                            Ok(ap_event) => {
                                                let _ = app_channel_tx.send(AppEvents::AccessPointEvent { event: ap_event }).await;
                                            }
                                            Err(e) => eprintln!("Error getting wifi state: {e}"),
                                        }
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

                                event = bt_rx.next() => {
                                    if let Some(event) = event  {
                                        match event {
                                                BtEvents::BluetoothToggle { enabled } => {
                                                let _ = bluetooth_manager.toggle_bluetooth(enabled).await;
                                            }
                                        }
                                    }
                                }

                                // volume events
                                volume_event = volume_rx.next() => {
                                    match volume_event {
                                        Some(VolumeEvents::VolumeChanged { name, value }) => {
                                            match pulse_service.handle.set_sink_volume_by_name(&name, &value).await {
                                                Ok(_) => {
                                                    update_device_info(&mut app_channel_tx, &pulse_service).await;

                                                }
                                                Err(e) => {
                                                    eprintln!("Failed to set volume: {}", e);
                                                }
                                            }
                                        }
                                        Some(VolumeEvents::MuteSink { name }) => {
                                            match pulse_service.handle.set_sink_mute_by_name(&name).await {
                                                Ok(_) => {
                                                    update_device_info(&mut app_channel_tx, &pulse_service).await;
                                                }
                                                Err(e) => {
                                                    eprintln!("Failed to set mute: {}", e);
                                                }
                                            }
                                        }
                                        Some(VolumeEvents::UnmuteSink { name }) => {
                                            match pulse_service.handle.set_sink_unmute_by_name(&name).await {
                                                Ok(_) => {
                                                update_device_info(&mut app_channel_tx, &pulse_service).await; 
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
                                    Some(BrightnessEvents::BrightnessChanged { value }) => {
                                        let value = if value < DEFAULT_MIN_BRIGHTNESS { DEFAULT_MIN_BRIGHTNESS } else { value };
                                        let value = percent_to_u8(value, MAX_DEVICE_BRIGHTNESS);
                                        match display_client::set_brightness(value).await {
                                            Ok(_) => {
                                                if let Ok(value) = display_client::get_brightness().await {
                                                    let value = u8_to_percent(value, MAX_DEVICE_BRIGHTNESS);
                                                    let _ = app_channel_tx.send(AppEvents::Brightness { value }).await;
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("Error setting brightness: {:?}", e);
                                            }
                                        }
                                    }
                                    None => break,
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
                                AppEvents::BatteryStateChanged { state } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.battery_state = state;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BatteryPercentageChanged { value } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.battery_percent = value;
                                        cx.notify();
                                    });
                                }
                                AppEvents::WirelessStatusChanged { enabled } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.wireless_details.enabled = enabled;
                                        cx.notify();
                                    });
                                } 
                                AppEvents::ListWirelessNetworks { list } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        // sort the list where is_active and is_known are first
                                        let mut sorted_list = list;
                                        sorted_list.sort_by_key(|n|(
                                            !n.is_active,
                                            !n.is_known
                                        ));
                                          sorted_list.retain(|n| !n.ssid.is_empty());
                                          this.wireless_details.networks = Some(sorted_list);
                                        cx.notify();
                                    });
                                }
                                AppEvents::AccessPointEvent { event } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        let event_network_info = event.wireless_network_info;

                                        match event.event_type {
                                            EventType::Added => {
                                                // // update list - add access point
                                                    if let Some(event_network_info) = event_network_info {
                                                        match &this.wireless_details.networks {
                                                            Some(networks) => {
                                                                let mut networks = networks.clone();
                                                                networks.push(event_network_info);
                                                                
                                                                this.wireless_details.networks = Some(networks);
                                                            }
                                                            None => {
                                                                this.wireless_details.networks = Some(vec![event_network_info]);
                                                            }
                                                        }
                                                    }
                                                },
                                            EventType::Removed => {
                                                //  // // update list - remove access point
                                                //     if let Some(event_network_info) = event_network_info {
                                                //         match &this.wireless_details.networks {
                                                //             Some(networks) => {
                                                //                 let mut networks = networks.clone();
                                                //                 networks.retain(|n| n.ssid != event_network_info.ssid);

                                                //                 this.wireless_details.networks = Some(networks);
                                                //             }
                                                //             None => {
                                                //                 this.wireless_details.networks = Some(vec![event_network_info]);
                                                //             }
                                                //         }
                                                //     }
                                            }
                                        }
                                        cx.notify();
                                    });
                                }
                                AppEvents::ConnectedNetwork { network } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.wireless_details.connected_network = network;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BluetoothEnabled { enabled } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.bluetooth_details.enabled = enabled;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BluetoothDevices { count } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.bluetooth_details.devices = count;
                                        cx.notify();
                                    });
                                }
                                AppEvents::OutputSoundDevice { device_info } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        let device_info = Some(device_info).clone().unwrap();
                                        this.volume_mute = device_info.mute;
                                        this.volume_slider_value = if this.volume_mute { 0.0 } else { device_info.volume as f32 };
                                        this.volume_device_name = device_info.name.unwrap_or_else(|| "default".to_string());
                                        this.volume_slider_state.update(cx, |state, _cx| {
                                            state.value = this
                                                .volume_slider_value
                                                .clamp(state.min, state.max);
                                        });
                                        cx.notify();
                                    });
                                }
                                AppEvents::Brightness { value } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.brightness_slider_value = value;
                                        this.brightness_slider_state.update(cx, |state, _cx| {
                                            state.value = value.clamp(state.min, state.max);
                                        });
                                        cx.notify();
                                    });
                                }
                            }
                        }
                    })
                    .detach();

                    SettingsDrawer::new(
                        cx,
                        nm_tx.clone(),
                        bt_tx.clone(),
                        volume_tx.clone(),
                        brightness_tx.clone(),
                    )
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
