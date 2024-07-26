mod errors;
mod gui;
mod settings;
mod theme;

use std::collections::HashMap;
use std::time::Duration;

use gui::Notification;
use mctk_core::{
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
    types::{AssetParams, ImgFilter},
};
use mctk_smithay::layer_shell::layer_window::LayerWindowParams;
use mctk_smithay::WindowOptions;
use mctk_smithay::{layer_shell::layer_surface::LayerOptions, WindowMessage};
use mctk_smithay::{layer_shell::layer_window::LayerWindow, WindowInfo};
use serde::{Deserialize, Serialize};

use crate::gui::Message;
use clap::Parser;
use settings::NotificationSettings;
use theme::NotificationTheme;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct NotificationArgs {
    #[arg(long)]
    app_name: Option<String>,
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    icon: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AppParams {}

#[derive(Debug)]
enum AppMessage {}

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

            NotificationSettings::default()
        }
    };

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => NotificationTheme::default(),
    };

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let assets: HashMap<String, AssetParams> = HashMap::new();
    let svgs: HashMap<String, String> = HashMap::new();

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.notification"));
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

    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<Notification, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            layer_shell_opts,
            svgs,
            ..Default::default()
        },
        AppParams {},
    );

    let handle = event_loop.handle();

    //subscribe to events channel
    let (channel_tx, channel_rx) = calloop::channel::channel();
    let window_tx_2 = window_tx.clone();
    let _ = handle.insert_source(channel_rx, move |event: Event<AppMessage>, _, app| {
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

    loop {
        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
}
