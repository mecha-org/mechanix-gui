use crate::{init_services, AppMessage, AppParams, UiParams};
use mctk_core::reexports::smithay_client_toolkit::shell::wlr_layer::Layer;
use std::sync::{Arc, RwLock};

use crate::gui::OnScreenDisplay;
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
pub fn launch_homescreen(
    lock_window_tx: Arc<RwLock<Option<Sender<WindowMessage>>>>,
    ui_params: UiParams,
) -> anyhow::Result<()> {
    let UiParams {
        fonts,
        assets,
        svgs,
        settings,
        theme,
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
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
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
    let (mut app, mut event_loop, window_tx) =
        LayerWindow::open_blocking::<OnScreenDisplay, AppParams>(
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
            },
        );

    let handle = event_loop.handle();

    let window_tx_2 = window_tx.clone();
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
                AppMessage::IpAddress { address } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::IpAddress { address }),
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
                AppMessage::RunningApps { count } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::RunningApps { count }),
                    });
                }
                AppMessage::RunOnTop => {
                    layer_shell_opts.layer = Layer::Top;
                    let _ = layer_tx
                        .clone()
                        .send(LayerWindowMessage::ReconfigureLayerOpts {
                            opts: layer_shell_opts.clone(),
                        });
                }
                AppMessage::RunOnBottom => {
                    layer_shell_opts.layer = Layer::Bottom;
                    let _ = layer_tx
                        .clone()
                        .send(LayerWindowMessage::ReconfigureLayerOpts {
                            opts: layer_shell_opts.clone(),
                        });
                }
                AppMessage::Clock { date, time } => {
                    //println!("AppMessage::Clock {:?}", current_time);
                    println!("sending clock message to homescreen");
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Clock {
                            date: date.clone(),
                            time: time.clone()
                        }),
                    });
                    println!("sent clock message to homescreen");
                    if let Ok(lock_window_tx) = lock_window_tx.read() {
                        println!(
                            "got lock screen window guard {:?}",
                            lock_window_tx.as_ref().is_some()
                        );
                        if let Some(lock_window_tx) = lock_window_tx.as_ref() {
                            println!("sending clock message to lockscreen");
                            let _ = lock_window_tx.send(WindowMessage::Send {
                                message: msg!(Message::Clock { date, time }),
                            });
                            println!("sent clock message to lockscreen");
                        }
                    }
                }
                AppMessage::Wireless { status } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Wireless { status }),
                    });
                }
                AppMessage::Bluetooth { status } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Bluetooth { status }),
                    });
                }
                AppMessage::Battery { level, status } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Battery { level, status }),
                    });
                }
            },
            calloop::channel::Event::Closed => {}
        };
    });

    init_services(settings.clone(), app_channel_tx);

    loop {
        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
}
