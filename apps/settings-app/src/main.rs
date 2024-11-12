mod components;
mod constants;
mod errors;
mod gui;
mod screens;
mod settings;
mod shared;
mod utils;

use crate::gui::SettingsApp;
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
    AssetParams,
};
use mctk_smithay::{
    layer_shell::{
        layer_surface::LayerOptions,
        layer_window::{LayerWindow, LayerWindowParams},
    },
    WindowInfo, WindowOptions,
};
use settings::{AppSettings, MainSettings};
use std::{collections::HashMap, fs, path::Path};
use tokio::runtime::Builder;
use tokio::select;
use tokio::sync::mpsc::{self, Receiver};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
pub struct UiParams {
    fonts: cosmic_text::fontdb::Database,
    assets: HashMap<String, AssetParams>,
    svgs: HashMap<String, String>,
    settings: MainSettings,
    // theme: AppTheme,
}

#[derive(Debug)]
enum AppMessage {
    ShowNetworkScreen,
    // Network { isConnected: bool, value: String }, // {isConnected: true, value: Mecha}
}

#[derive(Default, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let settings = match crate::settings::read_settings_yml() {
        Ok(settings) => {
            println!("SUCCESS read_settings_yml {:?}", settings.clone());
            settings
        }
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
        println!("font path is {:?}", path);
        if let Ok(content) = fs::read(Path::new(&path)) {
            fonts.load_font_data(content);
        }
    }

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    println!(
        "----> CHECK right_arrow_icon {:?} ",
        modules.see_options.right_arrow_icon.clone()
    );

    assets.insert(
        "wifi_icon".to_string(),
        AssetParams::new(modules.wireless.icon),
    );
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
        AssetParams::new(modules.about.icon),
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
        "back_icon".to_string(),
        AssetParams::new(modules.footer.back_icon),
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
    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<SettingsApp, AppParams>(
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

    // let (wireless_msg_tx, wireless_msg_rx) = mpsc::channel(128);
    // let (bluetooth_msg_tx, bluetooth_msg_rx) = mpsc::channel(128);
    // let (brightness_msg_tx, brightness_msg_rx) = mpsc::channel(128);
    // let (sound_msg_tx, sound_msg_rx) = mpsc::channel(128);
    // let (app_manager_msg_tx, app_manager_msg_rx) = mpsc::channel(128);

    // TODO: handle api
    let _ = handle.insert_source(app_channel_rx, move |event, _, _| {
        println!("11------------------> CHANGE TO NETWORK SCREEN !!");

        let _ = match event {
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::ShowNetworkScreen => {
                    println!("12------------------> CHANGE TO NETWORK SCREEN !!");
                }
            },
            calloop::channel::Event::Closed => {}
        };
    });

    loop {
        event_loop.dispatch(None, &mut app).unwrap();
    }
    // //End

    Ok(())
}
