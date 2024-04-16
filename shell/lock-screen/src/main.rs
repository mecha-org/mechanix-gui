mod components;
mod errors;
mod gui;
mod pages;
mod settings;
mod theme;

use std::collections::HashMap;
use std::time::Duration;

use gui::LockScreen;
use mctk_core::{
    msg,
    reexports::{
        cosmic_text,
        smithay_client_toolkit::{
            reexports::calloop::{
                self,
                channel::Sender,
                timer::{TimeoutAction, Timer},
            },
            shell::wlr_layer,
        },
    },
};
use mctk_smithay::{
    layer_surface::LayerOptions, lock_window::SessionLockWindowParams, WindowOptions,
};
use mctk_smithay::{layer_window::LayerWindowParams, WindowMessage};

use settings::LockScreenSettings;
use theme::LockScreenTheme;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::gui::Message;

#[derive(Debug)]
pub enum AppMessage {}

// Layer Surface App
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(e) => {
            println!("error while reading settings {:?}", e);

            LockScreenSettings::default()
        }
    };

    // let custom_theme = match theme::read_theme_yml() {
    //     Ok(theme) => theme,
    //     Err(_) => LockScreenTheme::default(),
    // };

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

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

    if let Some(icon) = modules.back_space.icon.default {
        svgs.insert("backspace_icon".to_string(), icon);
    }

    if let Some(icon) = modules.background.icon.default {
        assets.insert("background".to_string(), icon);
    }

    let namespace = settings.app.id.clone();

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    // let layer_shell_opts = LayerOptions {
    //     anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
    //     layer: wlr_layer::Layer::Overlay,
    //     keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
    //     namespace: Some(namespace.clone()),
    //     zone: 0 as i32,
    // };
    let (session_lock_tx, session_lock_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) =
        mctk_smithay::lock_window::SessionLockWindow::open_blocking::<LockScreen, AppMessage>(
            SessionLockWindowParams {
                // title: "LockScreen".to_string(),
                // namespace,
                session_lock_tx,
                session_lock_rx,
                window_opts,
                fonts,
                assets,
                // layer_shell_opts,
                svgs,
            },
            None,
        );

    let handle = event_loop.handle();

    //subscribe to events channel
    let (channel_tx, channel_rx) = calloop::channel::channel();
    let window_tx_2 = window_tx.clone();
    let _ = handle.insert_source(channel_rx, move |event, _, app| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => {
                let _ = window_tx_2
                    .clone()
                    .send(WindowMessage::Send { message: msg!(msg) });
            }
            calloop::channel::Event::Closed => {}
        };
    });

    init_services(channel_tx).await;

    loop {
        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
}

async fn init_services(sender: Sender<Message>) {}
