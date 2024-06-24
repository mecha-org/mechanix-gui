mod agent;
mod components;
mod dbus;
mod errors;
mod gui;
mod pages;
mod settings;
mod types;

use agent::register_polkit_agent;
use gui::PolkitAgent;
use keyring::Entry;
use mctk_core::{
    reexports::{
        cosmic_text,
        smithay_client_toolkit::{
            reexports::calloop::{self, channel::Event},
            shell::wlr_layer,
        },
    },
    types::AssetParams,
};
use mctk_smithay::layer_shell::layer_window::LayerWindowParams;
use mctk_smithay::WindowOptions;
use mctk_smithay::{layer_shell::layer_surface::LayerOptions, WindowMessage};
use mctk_smithay::{layer_shell::layer_window::LayerWindow, WindowInfo};
use mechanix_system_dbus_client::security::Security;
use settings::PolkitAgentSettings;
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio::{select, time};
use tracing::info;
use tracing_subscriber::EnvFilter;
use types::{Params, PolkitError, PolkitEvent};

#[derive(Debug)]
enum AppMessage {
    Authenticate { password: String },
    Cancel,
}

#[tokio::main]
async fn main() {
    let (event_tx, mut event_rx) = mpsc::channel(128);
    loop {
        select! {
            _ = register_polkit_agent(event_tx.clone()) => {
                println!("Agent exited");
            }
            Some(event) = event_rx.recv() => {
                println!("polkit event {:?}", event);
                match event {
                    PolkitEvent::CreateDialog(params) => {
                            // launch ui
                            let ui_t = std::thread::spawn(move || {
                                 let _ = launch_auth_ui(params);
                            });
                            ui_t.join().unwrap();

                    }
                    PolkitEvent::CancelDialog { cookie } => {
                        println!("CancelDialog called {:?}", cookie);
                    }
                }
            }
        }
    }
}

fn launch_auth_ui(params: Params) -> anyhow::Result<()> {
    println!("inside launch ui");

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(e) => {
            println!("error while reading settings {:?}", e);

            PolkitAgentSettings::default()
        }
    };

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let icons = settings.icons.clone();
    if let Some(icon) = icons.lock {
        svgs.insert("lock_icon".to_string(), icon);
    }

    if let Some(icon) = icons.submit {
        svgs.insert("submit_icon".to_string(), icon);
    }

    if let Some(icon) = icons.close {
        svgs.insert("close_icon".to_string(), icon);
    }

    if let Some(icon) = icons.next {
        svgs.insert("next_icon".to_string(), icon);
    }

    if let Some(icon) = icons.back_space {
        svgs.insert("backspace_icon".to_string(), icon);
    }

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.polkit-agent"));
    let namespace = app_id.clone();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::TOP,
        layer: wlr_layer::Layer::Top,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<PolkitAgent, AppMessage>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            layer_shell_opts,
            svgs,
            ..Default::default()
        },
        Some(app_channel_tx),
    );

    let handle = event_loop.handle();

    //subscribe to events channel
    let window_tx_2 = window_tx.clone();
    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Cancel => {
                    let Some(sender) = params.response_tx.lock().unwrap().take() else {
                        println!("response sender not found");
                        return;
                    };
                    let _ = sender.send(Err(PolkitError::Cancelled));
                    let _ = window_tx_2.send(WindowMessage::WindowEvent {
                        event: mctk_smithay::WindowEvent::CloseRequested,
                    });
                }
                AppMessage::Authenticate { password } => {
                    let Some(sender) = params.response_tx.lock().unwrap().take() else {
                        println!("response sender not found");
                        return;
                    };
                    let cookie = params.cookie.clone();
                    let Some(identity) = params.identities.get(0) else {
                        println!("identity not found");
                        return;
                    };

                    let Some(username) = users::get_current_username() else {
                        println!("username not found");
                        return;
                    };

                    let Ok(username) = username.into_string() else {
                        println!("error converting username to string");
                        return;
                    };

                    let Ok(entry) = Entry::new("mechanix-shell", &username) else {
                        println!("error creating entry");
                        return;
                    };

                    let Ok(secret) = entry.get_password() else {
                        println!("error while getting entry password");
                        return;
                    };

                    if secret.is_empty() {
                        println!("secret cannot be empty");
                        return;
                    }

                    let auth_res =
                        Security::authenticate_polkit(password, secret, cookie, identity);
                    println!("auth res is {:?}", auth_res);
                    let _ = sender.send(Ok(()));
                    println!("Closing window");
                    let _ = window_tx_2.send(WindowMessage::WindowEvent {
                        event: mctk_smithay::WindowEvent::CloseRequested,
                    });
                    println!("Window closed");
                }
            },
            calloop::channel::Event::Closed => {
                println!("calloop::event::closed");
            }
        };
    });

    loop {
        // println!("event_loop_dispatch");
        let _ = event_loop.dispatch(Duration::from_millis(16), &mut app);

        // println!("app is_exited {:?}", app.is_exited);
        if app.is_exited {
            break;
        }
    }
    //End

    println!("UI loop ended");
    Ok(())
}
