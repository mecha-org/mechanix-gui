mod errors;
mod event_handler;
mod gui;
mod modules;
mod settings;
mod theme;

use std::collections::HashMap;
use std::time::Duration;

use event_handler::zbus::ZbusServiceHandle;
use gui::StatusBar;
use mctk_core::{
    msg,
    reexports::smithay_client_toolkit::{
        reexports::calloop::{self, channel::Sender},
        shell::wlr_layer,
    },
};
use mctk_smithay::{layer_surface::LayerOptions, WindowOptions};
use mctk_smithay::{layer_window::LayerWindowParams, WindowMessage};
use modules::{
    battery::{component::get_battery_icons_map, handler::BatteryServiceHandle},
    bluetooth::component::get_bluetooth_icons_map,
    clock::handler::ClockServiceHandle,
    wireless::{component::get_wireless_icons_map, handler::WirelessServiceHandle},
};
use modules::{bluetooth::handler::BluetoothServiceHandle, window::handler::WindowServiceHandle};

use settings::StatusBarSettings;
use theme::StatusBarTheme;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::gui::{BatteryLevel, BluetoothStatus, Message, WirelessStatus};

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
        Err(_) => StatusBarSettings::default(),
    };

    // let custom_theme = match theme::read_theme_yml() {
    //     Ok(theme) => theme,
    //     Err(_) => StatusBarTheme::default(),
    // };

    let window = settings.window.clone();

    let window_opts = WindowOptions {
        height: window.height.unwrap_or(48) as u32,
        width: window.width.unwrap_or(480) as u32,
        scale_factor: 1.0,
    };

    let fonts: HashMap<String, String> = settings.fonts.clone();

    let mut assets: HashMap<String, String> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    let battery_assets = get_battery_icons_map(modules.battery.icon);
    let bluetooth_assets = get_bluetooth_icons_map(modules.bluetooth.icon);
    let wireless_assets = get_wireless_icons_map(modules.wireless.icon);

    svgs.extend(battery_assets);
    svgs.extend(wireless_assets);
    svgs.extend(bluetooth_assets);

    let namespace = settings.app.id.clone().unwrap_or_default();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::TOP,
        layer: wlr_layer::Layer::Bottom,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: window_opts.height as i32,
    };

    let (mut app, mut event_loop, window_tx) =
        mctk_smithay::layer_window::LayerWindow::open_blocking::<StatusBar>(LayerWindowParams {
            title: settings.title.clone(),
            namespace,
            window_opts,
            fonts,
            assets,
            svgs,
            layer_shell_opts,
        });

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

    init_services(settings, channel_tx).await;

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    //End

    Ok(())
}

async fn init_services(settings: StatusBarSettings, sender: Sender<Message>) {
    let mut clock_service_handle = ClockServiceHandle::new();
    let sender_clone_1 = sender.clone();
    let _ = tokio::spawn(async move {
        info!(task = "init_services", "Starting clock");
        clock_service_handle
            .run(settings.modules.clock.format, sender_clone_1)
            .await;
    });

    let mut wireless_service_handle = WirelessServiceHandle::new();
    let sender_clone_2 = sender.clone();
    let _ = tokio::spawn(async move {
        info!(task = "init_services", "Starting wireless service");
        wireless_service_handle.run(sender_clone_2).await;
    });

    let mut bluetooth_service_handle = BluetoothServiceHandle::new();
    let sender_clone_3 = sender.clone();
    let _ = tokio::spawn(async move {
        info!(task = "init_services", "Starting bluetooth service");
        bluetooth_service_handle.run(sender_clone_3).await;
    });

    let mut battery_service_handle = BatteryServiceHandle::new();
    let sender_clone_4 = sender.clone();
    let _ = tokio::spawn(async move {
        info!(task = "init_services", "Starting battery service");
        battery_service_handle.run(sender_clone_4).await;
    });

    let mut window_manager_service_handle = WindowServiceHandle::new();
    let sender_clone_5 = sender.clone();
    let _ = tokio::spawn(async move {
        info!(task = "init_services", "Starting window manager service");
        window_manager_service_handle.run(sender_clone_5).await;
    });

    let mut zbus_service_handle = ZbusServiceHandle::new();
    let sender_clone_6 = sender;
    let _ = tokio::spawn(async move {
        info!(task = "init_services", "Starting zbus service");
        zbus_service_handle.run(sender_clone_6).await;
    });
}
