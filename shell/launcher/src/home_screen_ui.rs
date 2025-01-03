use crate::{
    init_services_home,
    modules::{
        applications::model::DesktopEntriesModel, battery::model::BatteryModel,
        clock::model::ClockModel, power_options::service::PowerOptionsService,
        running_apps::app_manager::AppManagerMessage,
    },
    AppMessage, AppParams, BatteryMessage, BluetoothMessage, BrightnessMessage,
    InitServicesParamsHome, RunningAppsMessage, SoundMessage, UiParams,
};
use mctk_core::context::Model;
use mctk_core::{
    context::{self},
    reexports::smithay_client_toolkit::shell::wlr_layer::Layer,
};
use networkmanager::{IpAddressModel, WirelessModel};
use std::sync::{Arc, RwLock};
use tokio::sync::{mpsc, oneshot};
use tracing::error;

use crate::gui::Launcher;
use mctk_core::{
    msg,
    reexports::smithay_client_toolkit::{
        reexports::calloop::{
            self,
            channel::{Event, Sender},
        },
        shell::wlr_layer,
    },
};
use mctk_smithay::layer_shell::layer_window::{LayerWindowMessage, LayerWindowParams};
use mctk_smithay::WindowOptions;
use mctk_smithay::{layer_shell::layer_surface::LayerOptions, WindowMessage};
use mctk_smithay::{layer_shell::layer_window::LayerWindow, WindowInfo};

use crate::gui::Message;
pub fn launch_homescreen(ui_params: UiParams) -> anyhow::Result<()> {
    let UiParams {
        fonts,
        assets,
        svgs,
        settings,
        theme,
        installed_apps,
        pinned_apps,
    } = ui_params;

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.home-screen"));
    let namespace = app_id.clone();

    let mut layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::TOP | wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT,
        layer: wlr_layer::Layer::Bottom,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::OnDemand,
        namespace: Some(namespace.clone()),
        zone: 36 as i32,
    };

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    let (layer_tx, layer_rx) = calloop::channel::channel();
    //subscribe to events channel
    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<Launcher, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            layer_shell_opts: layer_shell_opts.clone(),
            svgs,
            layer_tx: Some(layer_tx.clone()),
            layer_rx: Some(layer_rx),
        },
        AppParams {
            app_channel: Some(app_channel_tx.clone()),
            installed_apps: Some(installed_apps.clone()),
            pinned_apps: Some(pinned_apps.clone()),
        },
    );

    let handle = event_loop.handle();

    let window_tx_2 = window_tx.clone();
    let window_tx_3 = window_tx.clone();
    let context_handler = context::get_static_context_handler();
    context_handler.register_on_change(Box::new(move || {
        window_tx_3
            .send(WindowMessage::Send { message: msg!(0) })
            .unwrap();
    }));
    // context_handler.register_context(&WirelessModel::get().connected_network);
    IpAddressModel::get().register_context_handler(context_handler);
    WirelessModel::get().register_context_handler(context_handler);
    ClockModel::get().register_context_handler(context_handler);
    BatteryModel::get().register_context_handler(context_handler);
    DesktopEntriesModel::get().register_context_handler(context_handler);

    // let (wireless_msg_tx, wireless_msg_rx) = mpsc::channel(128);
    let (bluetooth_msg_tx, bluetooth_msg_rx) = mpsc::channel(128);
    let (brightness_msg_tx, brightness_msg_rx) = mpsc::channel(128);
    let (sound_msg_tx, sound_msg_rx) = mpsc::channel(128);
    let (app_manager_msg_tx, app_manager_msg_rx) = mpsc::channel(128);
    let app_channel_tx_2 = app_channel_tx.clone();
    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::CPUUsage { usage } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::CPUUsage { usage }),
                    });
                }
                AppMessage::Uptime { uptime } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Uptime { uptime }),
                    });
                }
                AppMessage::MachineName { name } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::MachineName { name }),
                    });
                }
                AppMessage::Net { online } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Net { online }),
                    });
                }
                AppMessage::Memory { total, used } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Memory { total, used }),
                    });
                }
                AppMessage::RunningApps { message } => match message {
                    RunningAppsMessage::Status { count } => {
                        let _ = window_tx_2.clone().send(WindowMessage::Send {
                            message: msg!(Message::RunningApps { count }),
                        });
                    }
                    RunningAppsMessage::Toggle { value } => {
                        let _ = window_tx_2.clone().send(WindowMessage::Send {
                            message: msg!(Message::RunningAppsToggle { show: value }),
                        });

                        if value {
                            let _ = app_channel_tx_2
                                .clone()
                                .send(AppMessage::ChangeLayer(Layer::Top));
                        }
                    }
                },
                AppMessage::ChangeLayer(layer) => {
                    layer_shell_opts.layer = layer;
                    let _ = layer_tx
                        .clone()
                        .send(LayerWindowMessage::ReconfigureLayerOpts {
                            opts: layer_shell_opts.clone(),
                        });
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::ChangeLayer(layer)),
                    });
                }
                AppMessage::Bluetooth { message } => match message {
                    BluetoothMessage::Status { status } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Bluetooth { status }),
                        });
                    }
                    BluetoothMessage::Toggle { .. } => {
                        let bluetooth_msg_tx_cloned = bluetooth_msg_tx.clone();
                        futures::executor::block_on(async move {
                            let res = bluetooth_msg_tx_cloned.clone().send(message).await;
                        });
                    }
                },
                AppMessage::Brightness { message } => match message {
                    BrightnessMessage::Value { value } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Brightness { value: value }),
                        });
                    }
                    BrightnessMessage::Change { .. } => {
                        let brightness_msg_tx_cloned = brightness_msg_tx.clone();
                        futures::executor::block_on(async move {
                            let res = brightness_msg_tx_cloned.clone().send(message).await;
                        });
                    }
                },
                AppMessage::Sound { message } => match message {
                    SoundMessage::Value { value } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Sound { value }),
                        });
                    }
                    SoundMessage::Change { .. } => {
                        let sound_msg_tx_cloned = sound_msg_tx.clone();
                        futures::executor::block_on(async move {
                            let res = sound_msg_tx_cloned.clone().send(message).await;
                        });
                    }
                },
                AppMessage::AppsUpdated {
                    apps,
                    app_id,
                    active_apps_count,
                } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::AppsUpdated {
                            apps,
                            app_id,
                            active_apps_count
                        }),
                    });
                }

                AppMessage::ShutDown => {
                    println!("AppMessage::ShutDown");
                    let _ = PowerOptionsService::shutdown();
                }
                AppMessage::Restart => {
                    println!("AppMessage::Restart");
                    let _ = PowerOptionsService::restart();
                }
                AppMessage::Unlock => {}
                AppMessage::AppOpen { app_id } => {
                    let app_manager_msg_tx2 = app_manager_msg_tx.clone();
                    futures::executor::block_on(async move {
                        let (tx, rx) = oneshot::channel();
                        let _ = app_manager_msg_tx2
                            .send(AppManagerMessage::LaunchApp {
                                app_id,
                                reply_to: tx,
                            })
                            .await;
                        let res = rx.await.expect("no reply from service");

                        match res {
                            Ok(r) => {
                                println!("AppMessage::AppOpen response {:?}", r);
                            }
                            Err(e) => {
                                println!("AppMessage::AppOpen error {}", e);
                            }
                        }
                    })
                }
                AppMessage::AppClose { app_id } => {
                    let app_manager_msg_tx2 = app_manager_msg_tx.clone();
                    futures::executor::block_on(async move {
                        let (tx, rx) = oneshot::channel();
                        let _ = app_manager_msg_tx2
                            .send(AppManagerMessage::CloseApp {
                                app_id,
                                reply_to: tx,
                            })
                            .await;
                        let res = rx.await.expect("no reply from service");

                        match res {
                            Ok(r) => {
                                println!("AppMessage::AppClose response {:?}", r);
                            }
                            Err(e) => {
                                println!("AppMessage::AppClose error {}", e);
                            }
                        }
                    })
                }
            },
            calloop::channel::Event::Closed => {}
        };
    });

    init_services_home(InitServicesParamsHome {
        settings,
        app_channel: app_channel_tx,
        bluetooth_msg_rx,
        brightness_msg_rx,
        sound_msg_rx,
        app_manager_msg_rx,
    });

    loop {
        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
}
