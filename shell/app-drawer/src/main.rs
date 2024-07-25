mod components;
mod errors;
mod gui;
mod pages;
mod settings;
mod theme;
mod constants;

use std::collections::HashMap;
use std::time::Duration;

use gui::AppDrawer;
use mctk_core::{
    msg,
    reexports::{
        cosmic_text,
        smithay_client_toolkit::{
            reexports::calloop::{self, channel::Sender},
            shell::wlr_layer,
        },
    },
    types::AssetParams,
};
use mctk_smithay::layer_shell::layer_window::LayerWindowParams;
use mctk_smithay::WindowOptions;
use mctk_smithay::{layer_shell::layer_surface::LayerOptions, WindowMessage};
use mctk_smithay::{layer_shell::layer_window::LayerWindow, WindowInfo};

use desktop_entries::DesktopEntries;
use settings::AppDrawerSettings;
use theme::AppDrawerTheme;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::gui::Message;

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
            AppDrawerSettings::default()
        }
    };

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => AppDrawerTheme::default(),
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

    if let icon = modules.home.icon.default {
        svgs.insert("home_icon".to_string(), icon);
    }

    if let icon = modules.search.icon.default {
        svgs.insert("search_icon".to_string(), icon);
    }

    if let icon = modules.back.icon.default {
        svgs.insert("back_icon".to_string(), icon);
    }

    if let icon = modules.clear.icon.default {
        svgs.insert("clear_icon".to_string(), icon);
    }

    if let icon = modules.not_found.icon.default {
        assets.insert("not_found_icon".to_string(), AssetParams::new(icon));
    }

    if let icon = modules.not_found.icon.small {
        assets.insert("not_found_small_icon".to_string(), AssetParams::new(icon));
    }

    if let Ok(desktop_entries) = DesktopEntries::new() {
        for entry in desktop_entries.entries {
            if let Some(icon_path) = entry.icon_path {
                match icon_path.extension().and_then(|ext| ext.to_str()) {
                    Some("png") => {
                        assets.insert(
                            entry.name,
                            AssetParams::new(icon_path.to_str().unwrap().to_string()),
                        );
                    }
                    Some("svg") => {
                        svgs.insert(entry.name, icon_path.to_str().unwrap().to_string());
                    }
                    _ => (),
                }
            }
        }
    };

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.app-drawer"));
    let namespace = app_id.clone();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT | wlr_layer::Anchor::BOTTOM,
        layer: wlr_layer::Layer::Bottom,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<AppDrawer, AppParams>(
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

    init_services(channel_tx);

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    //End

    Ok(())
}

fn init_services(sender: Sender<Message>) {}
