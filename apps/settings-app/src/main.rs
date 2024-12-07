mod components;
mod constants;
mod errors;
mod gui;
mod screens;
mod settings;
mod shared;
mod utils;

use crate::gui::SettingsApp;
use crate::screens::network::wireless_model::WirelessModel;
use crate::screens::sound::sound_model::SoundModel;
use futures::StreamExt;
use gui::{Message, NetworkMessage};
use mctk_core::{
    context::{self, Model},
    msg,
    reexports::{
        cosmic_text,
        smithay_client_toolkit::{
            reexports::calloop::{
                self,
                channel::{Event, Sender},
            },
            shell::wlr_layer,
        },
    },
    AssetParams,
};
use mctk_smithay::{
    xdg_shell::xdg_window::{XdgWindow, XdgWindowParams},
    WindowInfo, WindowMessage, WindowOptions,
};
use mechanix_status_bar_components::types::WirelessStatus;
use mechanix_system_dbus_client::wireless::{
    KnownNetworkListResponse, KnownNetworkResponse, WirelessInfoResponse,
};
use screens::battery::battery_model::BatteryModel;
// use screens::wireless::handler::{WirelessDetailsItem, WirelessServiceHandle};
use settings::{AppSettings, MainSettings};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
};
use tokio::runtime::Builder;
use tokio::select;
use tokio::sync::mpsc::{self, Receiver};
use tracing_subscriber::EnvFilter;
use zbus::message;

#[derive(Debug, Clone)]
pub struct UiParams {
    fonts: cosmic_text::fontdb::Database,
    assets: HashMap<String, AssetParams>,
    svgs: HashMap<String, String>,
    settings: MainSettings,
    // theme: AppTheme,
}

#[derive(Debug)]
pub enum WirelessMessage {
    Status { status: Option<bool> },
    Toggle { value: Option<bool> },
    ConnectedNetworkName { name: String },
    // ConnectedNetworkDetails {
    //     details: Option<WirelessDetailsItem>,
    // },
    // AvailableNetworksList {
    //     list: Vec<WirelessDetailsItem>,
    // },
    // KnownNetworksList {
    //     // manage networks
    //     list: Vec<KnownNetworkResponse>,
    // },
    getStatus,
}

#[derive(Debug)]
pub enum AppMessage {
    Wireless { message: WirelessMessage },
    NotFound,
}

#[derive(Default, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
    settings: Arc<RwLock<MainSettings>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let settings = match crate::settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(e) => {
            println!("error while reading settings {:?}", e);
            MainSettings::default()
        }
    };

    // let theme = match crate::theme::read_theme_yml() {
    //     Ok(theme) => theme,
    //     Err(_) => LauncherTheme::default(),
    // };

    let mut fonts: cosmic_text::fontdb::Database = cosmic_text::fontdb::Database::new();
    for path in settings.fonts.paths.clone() {
        if let Ok(content) = fs::read(Path::new(&path)) {
            fonts.load_font_data(content);
        }
    }

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    // wireless icons //
    assets.insert(
        "wireless_on".to_string(),
        AssetParams::new(modules.wireless.icon.on),
    );
    assets.insert(
        "wireless_off".to_string(),
        AssetParams::new(modules.wireless.icon.off),
    );
    assets.insert(
        "wireless_low".to_string(),
        AssetParams::new(modules.wireless.icon.low),
    );
    assets.insert(
        "wireless_weak".to_string(),
        AssetParams::new(modules.wireless.icon.weak),
    );
    assets.insert(
        "wireless_good".to_string(),
        AssetParams::new(modules.wireless.icon.good),
    );
    assets.insert(
        "wireless_strong".to_string(),
        AssetParams::new(modules.wireless.icon.strong),
    );
    assets.insert(
        "wireless_error".to_string(),
        AssetParams::new(modules.wireless.icon.error),
    );
    assets.insert(
        "wireless_not_found".to_string(),
        AssetParams::new(modules.wireless.icon.not_found),
    );

    assets.insert(
        "wireless_settings".to_string(),
        AssetParams::new(modules.wireless.icon.wireless_settings),
    );
    // secured wireless icons //
    assets.insert(
        "secured_wireless_on".to_string(),
        AssetParams::new(modules.wireless.secured_icon.on),
    );
    assets.insert(
        "secured_wireless_off".to_string(),
        AssetParams::new(modules.wireless.secured_icon.off),
    );
    assets.insert(
        "secured_wireless_low".to_string(),
        AssetParams::new(modules.wireless.secured_icon.low),
    );
    assets.insert(
        "secured_wireless_weak".to_string(),
        AssetParams::new(modules.wireless.secured_icon.weak),
    );
    assets.insert(
        "secured_wireless_strong".to_string(),
        AssetParams::new(modules.wireless.secured_icon.strong),
    );
    assets.insert(
        "secured_wireless_error".to_string(),
        AssetParams::new(modules.wireless.secured_icon.error),
    );

    // ------------------------------//
    assets.insert(
        "bluetooth_icon".to_string(),
        AssetParams::new(modules.bluetooth.icon),
    );
    assets.insert(
        "display_icon".to_string(),
        AssetParams::new(modules.display.icon),
    );
    assets.insert(
        "appearance_icon".to_string(),
        AssetParams::new(modules.appearance.icon),
    );
    assets.insert(
        "battery_icon".to_string(),
        AssetParams::new(modules.battery.icon),
    );
    assets.insert(
        "sound_icon".to_string(),
        AssetParams::new(modules.sound.icon),
    );
    assets.insert("lock_icon".to_string(), AssetParams::new(modules.lock.icon));
    assets.insert(
        "date_time_icon".to_string(),
        AssetParams::new(modules.date_time.icon),
    );
    assets.insert(
        "language_icon".to_string(),
        AssetParams::new(modules.language.icon),
    );
    assets.insert(
        "update_icon".to_string(),
        AssetParams::new(modules.update.icon),
    );
    assets.insert(
        "about_icon".to_string(),
        AssetParams::new(modules.about.icon.default),
    );
    assets.insert(
        "device_icon".to_string(),
        AssetParams::new(modules.about.icon.device),
    );
    assets.insert(
        "right_arrow_icon".to_string(),
        AssetParams::new(modules.see_options.right_arrow_icon),
    );
    assets.insert(
        "connected_icon".to_string(),
        AssetParams::new(modules.see_options.connected_icon),
    );
    assets.insert(
        "info_icon".to_string(),
        AssetParams::new(modules.see_options.info_icon),
    );
    assets.insert(
        "back_icon".to_string(),
        AssetParams::new(modules.footer.back_icon),
    );
    assets.insert(
        "add_icon".to_string(),
        AssetParams::new(modules.footer.add_icon),
    );
    assets.insert(
        "tick_icon".to_string(),
        AssetParams::new(modules.footer.tick_icon),
    );
    assets.insert(
        "delete_icon".to_string(),
        AssetParams::new(modules.footer.delete_icon),
    );
    // let background = modules.background.icon.default;
    // if background.len() > 0 {
    //     assets.insert("background".to_string(), AssetParams::new(background));
    // }

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix-settings"));
    let namespace = app_id.clone();

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    //subscribe to events channel
    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();
    let settings = Arc::new(RwLock::new(settings));

    let (mut app, mut event_loop, window_tx) = XdgWindow::open_blocking::<SettingsApp, AppParams>(
        XdgWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            svgs,
            ..Default::default()
        },
        AppParams {
            app_channel: Some(app_channel_tx.clone()),
            settings: settings.clone(),
        },
    );
    let app_channel = app_channel_tx.clone();

    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();
    let window_tx_channel = window_tx.clone();
    let context_handler = context::get_static_context_handler();
    context_handler.register_on_change(Box::new(move || {
        window_tx_channel
            .send(WindowMessage::Send { message: msg!(0) })
            .unwrap();
    }));
    SoundModel::get().register_context_handler(context_handler);
    WirelessModel::get().register_context_handler(context_handler);
    BatteryModel::get().register_context_handler(context_handler);

    let (wireless_msg_tx, wireless_msg_rx) = mpsc::channel(128);
    // let (bluetooth_msg_tx, bluetooth_msg_rx) = mpsc::channel(128);

    let _ = handle.insert_source(app_channel_rx, move |event, _, _| {
        let _ = match event {
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Wireless { message } => match message {
                    WirelessMessage::Status { status } => {
                        if let Some(value) = status {
                            let _ = window_tx_2.send(WindowMessage::Send {
                                message: msg!(NetworkMessage::WirelessStatus { status: value }),
                            });
                        } else {
                            println!("No Wireless Value found");
                        }
                    }
                    WirelessMessage::getStatus => {
                        let wireless_msg_tx_cloned = wireless_msg_tx.clone();
                        futures::executor::block_on(async move {
                            //let (tx, rx) = oneshot::channel();
                            let res = wireless_msg_tx_cloned.clone().send(message).await;
                            //let res = rx.await.expect("no reply from service");
                        });
                    }
                    WirelessMessage::Toggle { .. } => {
                        let wireless_msg_tx_cloned = wireless_msg_tx.clone();
                        futures::executor::block_on(async move {
                            //let (tx, rx) = oneshot::channel();
                            let res = wireless_msg_tx_cloned.clone().send(message).await;
                            //let res = rx.await.expect("no reply from service");
                        });
                    }
                    WirelessMessage::ConnectedNetworkName { name } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(NetworkMessage::ConnectedNetworkName { name: name }),
                        });
                    } // WirelessMessage::ConnectedNetworkDetails { details } => {
                      //     let _ = window_tx_2.send(WindowMessage::Send {
                      //         message: msg!(NetworkMessage::ConnectedNetworkDetails {
                      //             details: details
                      //         }),
                      //     });
                      // }
                      // WirelessMessage::AvailableNetworksList { list } => {
                      //     let _ = window_tx_2.send(WindowMessage::Send {
                      //         message: msg!(NetworkMessage::AvailableNetworksList { list: list }),
                      //     });
                      // }
                      // WirelessMessage::KnownNetworksList { list } => {
                      //     let _ = window_tx_2.send(WindowMessage::Send {
                      //         message: msg!(NetworkMessage::KnownNetworksList { list: list }),
                      //     });
                      // }
                },
                AppMessage::NotFound => todo!(),
                _ => (),
            },
            calloop::channel::Event::Closed => {}
        };
    });
    // // NOTE: not working with API for now
    // init_services(settings.clone(), app_channel, wireless_msg_rx);

    loop {
        event_loop.dispatch(None, &mut app).unwrap();

        if app.is_exited {
            break;
        }
    }

    Ok(())
}

// fn init_services(
//     settings: Arc<RwLock<MainSettings>>,
//     app_channel: Sender<AppMessage>,
//     wireless_msg_rx: Receiver<WirelessMessage>,
// ) -> JoinHandle<()> {
//     thread::spawn(move || {
//         let runtime = Builder::new_multi_thread()
//             .worker_threads(1)
//             .enable_all()
//             .build()
//             .unwrap();

//         let wireless_f = run_wireless_handler(app_channel.clone(), wireless_msg_rx);

//         runtime
//             .block_on(runtime.spawn(async move { tokio::join!(wireless_f) }))
//             .unwrap();
//     })
// }

// async fn run_wireless_handler(
//     app_channel: Sender<AppMessage>,
//     wireless_msg_rx: Receiver<WirelessMessage>,
// ) {
//     let mut wireless_service_handle = WirelessServiceHandle::new(app_channel);
//     wireless_service_handle.run(wireless_msg_rx).await;
// }
