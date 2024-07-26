mod components;
mod errors;
mod gui;
mod handlers;
mod pages;
mod settings;
mod theme;
mod users;
mod constants;

use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use greetd_ipc::Response;
use gui::Greeter;
use handlers::login::handler::{LoginHandler, LoginHandlerMessage};
use mctk_core::types::{AssetParams, ImgFilter};
use mctk_core::{msg, reexports::cosmic_text};
use mctk_smithay::layer_shell::layer_window::{LayerWindow, LayerWindowParams};
use mctk_smithay::{layer_shell::layer_surface::LayerOptions, WindowMessage};
use mctk_smithay::{WindowInfo, WindowOptions};
use mechanix_status_bar_components::modules::battery::component::{
    get_battery_icons_charging_map, get_battery_icons_map,
};
use mechanix_status_bar_components::modules::battery::handler::BatteryServiceHandle;
use mechanix_status_bar_components::modules::bluetooth::component::get_bluetooth_icons_map;
use mechanix_status_bar_components::modules::bluetooth::handler::BluetoothServiceHandle;
use mechanix_status_bar_components::modules::clock::handler::ClockServiceHandle;
use mechanix_status_bar_components::modules::wireless::component::get_wireless_icons_map;
use mechanix_status_bar_components::modules::wireless::handler::WirelessServiceHandle;
use mechanix_status_bar_components::StatusBarMessage;
use smithay_client_toolkit::reexports::calloop::{self, channel::Sender};

use mechanix_status_bar_components::types::{BatteryStatus, BluetoothStatus, WirelessStatus};
use settings::GreeterSettings;
use smithay_client_toolkit::shell::wlr_layer;
use theme::GreeterTheme;
use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};
use tracing::info;
use tracing_subscriber::EnvFilter;
use users::UsersSettings;

use crate::gui::Message;

#[derive(Debug)]
pub enum Prompt {
    Captcha { message: String },
    Password { message: String },
}

#[derive(Debug, Clone)]
pub enum AuthSubmit {
    Username(String),
    Password(String),
    Captcha(String),
    Cancel,
}

#[derive(Debug)]
pub enum LoginHandlerEvents {
    ShowErr(String),
    ClearErr,
    HandleGreetdResponse(Response),
    Prompts(Prompt),
    AuthError,
}

#[derive(Debug)]
pub enum AppMessage {
    LoginEvents(LoginHandlerEvents),
    AuthSubmit(AuthSubmit),
}

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
        Err(e) => {
            println!("error while reading settings {:?}", e);
            GreeterSettings::default()
        }
    };

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => GreeterTheme::default(),
    };

    let users_settings = match users::read_users_yml() {
        Ok(users) => users,
        Err(_) => UsersSettings::default(),
    };

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    if let icon = modules.lock.icon.default {
        svgs.insert("lock_icon".to_string(), icon);
    }

    if let icon = modules.unlock.icon.default {
        svgs.insert("unlock_icon".to_string(), icon);
    }

    if let icon = modules.back.icon.default {
        svgs.insert("back_icon".to_string(), icon);
    }

    if let icon = modules.next.icon.default {
        svgs.insert("next_icon".to_string(), icon);
    }

    if let icon = modules.back_space.icon.default {
        svgs.insert("backspace_icon".to_string(), icon);
    }
    if let icon = modules.home.icon.default {
        svgs.insert("home_icon".to_string(), icon);
    }

    if let icon = modules.power.icon.default {
        svgs.insert("power_icon".to_string(), icon);
    }

    if let icon = modules.shutdown.icon.default {
        svgs.insert("shutdown_icon".to_string(), icon);
    }

    if let icon = modules.restart.icon.default {
        svgs.insert("restart_icon".to_string(), icon);
    }

    if let icon = modules.sleep.icon.default {
        svgs.insert("sleep_icon".to_string(), icon);
    }

    if let icon = modules.close.icon.default {
        svgs.insert("close_icon".to_string(), icon);
    }

    if let icon = modules.submit.icon.default {
        svgs.insert("submit_icon".to_string(), icon);
    }
    if let icon = modules.show.icon.default {
        svgs.insert("show_icon".to_string(), icon);
    }
    if let icon = modules.hide.icon.default {
        svgs.insert("hide_icon".to_string(), icon);
    }
    if let icon = modules.background.icon.default {
        assets.insert(
            "background".to_string(),
            AssetParams {
                path: icon,
                filter: ImgFilter::GRAY,
                blur: None,
            },
        );
    }

    for user in users_settings.users {
        if let Some(icon) = user.avatar {
            svgs.insert(user.username, icon);
        }
    }

    let modules = settings.modules.clone();

    let battery_assets = get_battery_icons_map(modules.battery.icon);
    let battery_charging_assets = get_battery_icons_charging_map(modules.battery.charging_icon);
    let bluetooth_assets = get_bluetooth_icons_map(modules.bluetooth.icon);
    let wireless_assets = get_wireless_icons_map(modules.wireless.icon);

    svgs.extend(battery_assets);
    svgs.extend(battery_charging_assets);
    svgs.extend(wireless_assets);
    svgs.extend(bluetooth_assets);

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.greeter"));
    let namespace = app_id.clone();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
        layer: wlr_layer::Layer::Bottom,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };
    let (app_channel, app_receiver) = calloop::channel::channel();
    let app_channel2 = app_channel.clone();

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<Greeter, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            layer_shell_opts,
            svgs,
            ..Default::default()
        },
        AppParams {
            app_channel: Some(app_channel),
        },
    );

    let handle = event_loop.handle();

    let window_tx_2 = window_tx.clone();
    let window_tx_3 = window_tx.clone();
    //subscribe to events channel
    let (greeter_msg_tx, greeter_msg_rx) = mpsc::channel(128);
    let _ = handle.insert_source(app_receiver, move |event, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => {
                println!("main::event {:?}", msg);
                match msg {
                    AppMessage::AuthSubmit(submit_type) => {
                        let greeter_msg_tx = greeter_msg_tx.clone();
                        match submit_type {
                            AuthSubmit::Username(username) => {
                                futures::executor::block_on(async move {
                                    let (tx, rx) = oneshot::channel();
                                    let _ = greeter_msg_tx
                                        .clone()
                                        .send(LoginHandlerMessage::Login {
                                            username,
                                            reply_to: tx,
                                        })
                                        .await;
                                    let res = rx.await.expect("no reply from service");
                                });
                            }
                            AuthSubmit::Password(password) => {
                                futures::executor::block_on(async move {
                                    let (tx, rx) = oneshot::channel();
                                    let _ = greeter_msg_tx
                                        .clone()
                                        .send(LoginHandlerMessage::PasswordInput {
                                            password: password,
                                            reply_to: tx,
                                        })
                                        .await;
                                    let res = rx.await.expect("no reply from service");
                                });
                            }
                            AuthSubmit::Captcha(captcha) => {
                                futures::executor::block_on(async move {
                                    let (tx, rx) = oneshot::channel();
                                    let _ = greeter_msg_tx
                                        .clone()
                                        .send(LoginHandlerMessage::CaptchaInput {
                                            captcha,
                                            reply_to: tx,
                                        })
                                        .await;
                                    let res = rx.await.expect("no reply from service");
                                });
                            }
                            AuthSubmit::Cancel => {
                                futures::executor::block_on(async move {
                                    let (tx, rx) = oneshot::channel();
                                    let _ = greeter_msg_tx
                                        .clone()
                                        .send(LoginHandlerMessage::CancelSession { reply_to: tx })
                                        .await;
                                    let res = rx.await.expect("no reply from service");
                                });
                            }
                        };
                    }
                    AppMessage::LoginEvents(login_event) => {
                        println!("enc {:?}", msg!(Box::new(&login_event)));
                        let _ = window_tx_2.clone().send(WindowMessage::Send {
                            message: msg!(LoginHandlerEvents::from(login_event)),
                        });
                    }
                }

                // AppMessage::Test => {
                //     let _ = window_tx_2.clone().send(WindowMessage::Send {
                //         message: msg!(Message::AppsUpdated { apps }),
                //     });
                // }
            }
            calloop::channel::Event::Closed => {}
        };
    });

    let (status_bar_channel, status_bar_receiver) = calloop::channel::channel();
    let _ = handle.insert_source(status_bar_receiver, move |event, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => {
                println!("main::event {:?}", msg);
                match msg {
                    StatusBarMessage::Clock { current_time } => {
                        //println!("StatusBarMessage::Clock {:?}", current_time);
                        let _ = window_tx_3.clone().send(WindowMessage::Send {
                            message: msg!(Message::Clock { current_time }),
                        });
                    }
                    StatusBarMessage::Wireless { status } => {
                        let _ = window_tx_3.clone().send(WindowMessage::Send {
                            message: msg!(Message::Wireless { status }),
                        });
                    }
                    StatusBarMessage::Bluetooth { status } => {
                        let _ = window_tx_3.clone().send(WindowMessage::Send {
                            message: msg!(Message::Bluetooth { status }),
                        });
                    }
                    StatusBarMessage::Battery { level, status } => {
                        let _ = window_tx_3.clone().send(WindowMessage::Send {
                            message: msg!(Message::Battery { level, status }),
                        });
                    }
                    StatusBarMessage::Window { title, activated } => {}
                }
            }
            calloop::channel::Event::Closed => {}
        };
    });
    init_services(greeter_msg_rx, settings, app_channel2, status_bar_channel);

    loop {
        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
}

fn init_services(
    greeter_msg_rx: mpsc::Receiver<LoginHandlerMessage>,
    settings: GreeterSettings,
    app_channel: Sender<AppMessage>,
    status_bar_channel: Sender<StatusBarMessage>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let login_f = run_login_handler(greeter_msg_rx, app_channel.clone());
        let time_format = settings.modules.clock.format.clone();
        let clock_f = run_clock_handler(time_format, status_bar_channel.clone());
        let wireless_f = run_wireless_handler(status_bar_channel.clone());
        let bluetooth_f = run_bluetooth_handler(status_bar_channel.clone());
        let battery_f = run_battery_handler(status_bar_channel.clone());

        runtime
            .block_on(runtime.spawn(async move {
                tokio::join!(login_f, clock_f, wireless_f, bluetooth_f, battery_f)
            }))
            .unwrap();
    })
}

async fn run_login_handler(
    msg_rx: mpsc::Receiver<LoginHandlerMessage>,
    app_channel_tx: calloop::channel::Sender<AppMessage>,
) {
    // create the login instance
    let login_handler = LoginHandler::new().await;

    // start the login handler
    let _ = login_handler.unwrap().run(msg_rx, app_channel_tx).await;
}

async fn run_clock_handler(time_format: String, status_bar_channel: Sender<StatusBarMessage>) {
    let mut clock_service_handle = ClockServiceHandle::new(status_bar_channel);
    clock_service_handle.run(time_format).await;
}

async fn run_wireless_handler(status_bar_channel: Sender<StatusBarMessage>) {
    let mut wireless_service_handle = WirelessServiceHandle::new(status_bar_channel);
    wireless_service_handle.run().await;
}

async fn run_bluetooth_handler(status_bar_channel: Sender<StatusBarMessage>) {
    let mut bluetooth_service_handle = BluetoothServiceHandle::new(status_bar_channel);
    bluetooth_service_handle.run().await;
}

async fn run_battery_handler(status_bar_channel: Sender<StatusBarMessage>) {
    let mut battery_service_handle = BatteryServiceHandle::new(status_bar_channel);
    battery_service_handle.run().await;
}
