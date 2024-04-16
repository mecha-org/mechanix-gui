mod components;
mod errors;
mod gui;
mod handlers;
mod pages;
mod settings;
mod theme;
mod users;

use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use greetd_ipc::Response;
use gui::Greeter;
use handlers::login::handler::{LoginHandler, LoginHandlerMessage};
use mctk_core::{msg, reexports::cosmic_text};
use mctk_smithay::{
    layer_surface::LayerOptions, lock_window::SessionLockWindowParams, WindowOptions,
};
use mctk_smithay::{layer_window::LayerWindowParams, WindowMessage};
use smithay_client_toolkit::reexports::calloop::{self, channel::Sender};

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

    let mut assets: HashMap<String, String> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    if let Some(icon) = modules.lock.icon.default {
        svgs.insert("lock_icon".to_string(), icon);
    }

    if let Some(icon) = modules.unlock.icon.default {
        svgs.insert("unlock_icon".to_string(), icon);
    }

    if let Some(icon) = modules.back.icon.default {
        svgs.insert("back_icon".to_string(), icon);
    }

    if let Some(icon) = modules.next.icon.default {
        svgs.insert("next_icon".to_string(), icon);
    }

    if let Some(icon) = modules.back_space.icon.default {
        svgs.insert("backspace_icon".to_string(), icon);
    }
    if let Some(icon) = modules.home.icon.default {
        svgs.insert("home_icon".to_string(), icon);
    }

    if let Some(icon) = modules.power.icon.default {
        svgs.insert("power_icon".to_string(), icon);
    }

    if let Some(icon) = modules.shutdown.icon.default {
        svgs.insert("shutdown_icon".to_string(), icon);
    }

    if let Some(icon) = modules.restart.icon.default {
        svgs.insert("restart_icon".to_string(), icon);
    }

    if let Some(icon) = modules.sleep.icon.default {
        svgs.insert("sleep_icon".to_string(), icon);
    }

    if let Some(icon) = modules.close.icon.default {
        svgs.insert("close_icon".to_string(), icon);
    }

    if let Some(icon) = modules.submit.icon.default {
        svgs.insert("submit_icon".to_string(), icon);
    }
    if let Some(icon) = modules.show.icon.default {
        svgs.insert("show_icon".to_string(), icon);
    }
    if let Some(icon) = modules.hide.icon.default {
        svgs.insert("hide_icon".to_string(), icon);
    }
    if let Some(icon) = modules.background.icon.default {
        assets.insert("background".to_string(), icon);
    }

    for user in users_settings.users {
        if let Some(icon) = user.avatar {
            svgs.insert(user.username, icon);
        }
    }

    let namespace = "mctk.lock.screeen".to_string();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
        layer: wlr_layer::Layer::Bottom,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };
    let (app_channel, app_receiver) = calloop::channel::channel();
    let app_channel2 = app_channel.clone();
    let (mut app, mut event_loop, window_tx) =
        mctk_smithay::layer_window::LayerWindow::open_blocking::<Greeter, AppMessage>(
            LayerWindowParams {
                title: "Greeter".to_string(),
                namespace,
                window_opts,
                fonts,
                assets,
                layer_shell_opts,
                svgs,
            },
            Some(app_channel),
        );

    let handle = event_loop.handle();

    let window_tx_2 = window_tx.clone();
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

    init_services(greeter_msg_rx, app_channel2);

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    //End

    Ok(())
}

fn init_services(
    greeter_msg_rx: mpsc::Receiver<LoginHandlerMessage>,
    app_channel: Sender<AppMessage>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let future1 = run_login_handler(greeter_msg_rx, app_channel);

        runtime
            .block_on(runtime.spawn(async move { tokio::join!(future1) }))
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
