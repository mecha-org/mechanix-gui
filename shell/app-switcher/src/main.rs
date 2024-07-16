mod components;
mod errors;
mod gui;
mod services;
mod settings;
mod theme;
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use gui::{AppMessage, AppSwitcher};
use mctk_core::AssetParams;
use mctk_core::{msg, reexports::cosmic_text};
use mctk_smithay::xdg_shell::xdg_window::{XdgWindow, XdgWindowParams};
use mctk_smithay::{WindowInfo, WindowMessage, WindowOptions};
use services::app_manager::{AppManagerMessage, AppManagerService};
use smithay_client_toolkit::reexports::calloop::{self, channel::Sender};

use settings::AppSwitcherSettings;
use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::gui::Message;

#[derive(Debug, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
}

// Layer Surface App
fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => AppSwitcherSettings::default(),
    };

    let window = settings.window.clone();

    let window_opts = WindowOptions {
        height: window.size.1 as u32,
        width: window.size.0 as u32,
        scale_factor: 1.0,
    };

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    if let Some(icon) = modules.back.icon {
        svgs.insert("back_icon".to_string(), icon);
    }

    if let Some(icon) = modules.close_all.icon {
        svgs.insert("close_all_icon".to_string(), icon);
    }

    if let Some(icon) = modules.not_found.icon.small {
        assets.insert("not_found_small_icon".to_string(), AssetParams::new(icon));
    }

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.app-switcher"));

    let window_info = WindowInfo {
        id: app_id.clone(),
        title: settings.title.clone(),
        namespace: app_id,
    };

    let (app_channel, app_receiver) = calloop::channel::channel();
    let app_channel2 = app_channel.clone();
    let (mut app, mut event_loop, window_tx) = XdgWindow::open_blocking::<AppSwitcher, AppParams>(
        XdgWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            svgs,
            ..Default::default()
        },
        AppParams {
            app_channel: Some(app_channel),
        },
    );

    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();
    //subscribe to events channel
    let (app_manager_msg_tx, app_manager_msg_rx) = mpsc::channel(128);
    let _ = handle.insert_source(app_receiver, move |event, _, _| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::AppsUpdated { apps } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::AppsUpdated { apps }),
                    });
                }
                AppMessage::AppInstanceClicked(instance) => {
                    let app_manager_msg_tx2 = app_manager_msg_tx.clone();
                    futures::executor::block_on(async move {
                        let (tx, rx) = oneshot::channel();
                        println!("sending message to wayland to activate app instance");
                        let _ = app_manager_msg_tx2
                            .clone()
                            .send(AppManagerMessage::ActivateAppInstance {
                                instance,
                                reply_to: tx,
                            })
                            .await;
                        println!("message sent to wayland to activate app instance");
                        let res = rx.await.expect("no reply from service");

                        match res {
                            Ok(r) => {
                                println!("activate app instance res from wayland {:?}", r);
                            }
                            Err(e) => {
                                error!("activate app instance error from wayland {}", e);
                            }
                        }
                    });
                }
                AppMessage::AppInstanceCloseClicked(instance) => {
                    let app_manager_msg_tx2 = app_manager_msg_tx.clone();
                    futures::executor::block_on(async move {
                        let (tx, rx) = oneshot::channel();
                        println!("sending message to wayland to close app instance");
                        let _ = app_manager_msg_tx2
                            .send(AppManagerMessage::CloseAppInstance {
                                instance,
                                reply_to: tx,
                            })
                            .await;
                        println!("message sent to wayland to close app instance");
                        let res = rx.await.expect("no reply from service");

                        match res {
                            Ok(r) => {
                                println!("close app instance res from wayland {:?}", r);
                            }
                            Err(e) => {
                                error!("close app instance error from wayland {}", e);
                            }
                        }
                    })
                }
                AppMessage::CloseAllApps => {
                    let app_manager_msg_tx2 = app_manager_msg_tx.clone();
                    futures::executor::block_on(async move {
                        let (tx, rx) = oneshot::channel();
                        println!("sending message to wayland to close all apps instance");
                        let _ = app_manager_msg_tx2
                            .send(AppManagerMessage::CloseAllApps { reply_to: tx })
                            .await;
                        println!("message sent to wayland to close all apps instance");
                        let res = rx.await.expect("no reply from service");

                        match res {
                            Ok(r) => {
                                println!("close all apps instance res from wayland {:?}", r);
                            }
                            Err(e) => {
                                println!("close all apps instance error from wayland {}", e);
                            }
                        }
                    })
                }
                _ => (),
            },
            calloop::channel::Event::Closed => {}
        };
    });

    init_services(app_manager_msg_rx, app_channel2);

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    //End

    Ok(())
}

fn init_services(
    app_manager_msg_rx: mpsc::Receiver<AppManagerMessage>,
    app_channel: Sender<AppMessage>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let future1 = run_app_manager_handler(app_manager_msg_rx, app_channel);

        runtime
            .block_on(runtime.spawn(async move { tokio::join!(future1) }))
            .unwrap();
    })
}

async fn run_app_manager_handler(
    msg_rx: mpsc::Receiver<AppManagerMessage>,
    app_channel_tx: calloop::channel::Sender<AppMessage>,
) {
    // create the app manager instance
    let mut app_manager_handler = AppManagerService::new();

    // start the app manager handler
    let _ = app_manager_handler.run(msg_rx, app_channel_tx).await;
}
