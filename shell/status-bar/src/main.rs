mod errors;
mod event_handler;
mod gui;
mod modules;
mod settings;
mod theme;
mod types;

use gui::StatusBar;
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
use modules::{
    battery::{
        component::{get_battery_icons_charging_map, get_battery_icons_map},
        handler::BatteryServiceHandle,
    },
    bluetooth::component::get_bluetooth_icons_map,
    clock::handler::ClockServiceHandle,
    wireless::{component::get_wireless_icons_map, handler::WirelessServiceHandle},
};
use modules::{bluetooth::handler::BluetoothServiceHandle, window::handler::WindowServiceHandle};
use std::time::Duration;
use std::{collections::HashMap, fmt};
use tracing_subscriber::EnvFilter;

use settings::StatusBarSettings;
use std::thread::{self, JoinHandle};
use tokio::runtime::Builder;
use types::{BatteryStatus, BluetoothStatus, WirelessStatus};

use crate::gui::Message;

#[derive(Debug)]
pub enum AppMessage {
    Clock { current_time: String },
    Window { title: String, activated: bool },
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: u8, status: BatteryStatus },
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
        Err(_) => StatusBarSettings::default(),
    };

    // let custom_theme = match theme::read_theme_yml() {
    //     Ok(theme) => theme,
    //     Err(_) => StatusBarTheme::default(),
    // };

    let window = settings.window.clone();

    let window_opts = WindowOptions {
        height: window.height.unwrap_or(37) as u32,
        width: window.width.unwrap_or(480) as u32,
        scale_factor: 1.0,
    };

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

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
        .unwrap_or(String::from("mechanix.shell.status-bar"));
    let namespace = app_id.clone();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::TOP,
        layer: wlr_layer::Layer::Bottom,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: window_opts.height as i32,
    };

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    let (app_channel, app_receiver) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<StatusBar, AppMessage>(
        LayerWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            svgs,
            layer_shell_opts,
            ..Default::default()
        },
        None,
    );

    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();

    let _ = handle.insert_source(app_receiver, move |event, _, _| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Clock { current_time } => {
                    //println!("AppMessage::Clock {:?}", current_time);
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Clock { current_time }),
                    });
                }
                AppMessage::Window { title, activated } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Window { title, activated }),
                    });
                }
                AppMessage::Wireless { status } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Wireless { status }),
                    });
                }
                AppMessage::Bluetooth { status } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Bluetooth { status }),
                    });
                }
                AppMessage::Battery { level, status } => {
                    let _ = window_tx_2.clone().send(WindowMessage::Send {
                        message: msg!(Message::Battery { level, status }),
                    });
                }

                _ => (),
            },
            calloop::channel::Event::Closed => {}
        };
    });

    init_services(settings.clone(), app_channel);

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    //End

    Ok(())
}

fn init_services(settings: StatusBarSettings, app_channel: Sender<AppMessage>) -> JoinHandle<()> {
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let time_format = settings.modules.clock.format.clone();
        let clock_f = run_clock_handler(time_format, app_channel.clone());
        let window_f = run_window_handler(app_channel.clone());
        let wireless_f = run_wireless_handler(app_channel.clone());
        let bluetooth_f = run_bluetooth_handler(app_channel.clone());
        let battery_f = run_battery_handler(app_channel.clone());

        runtime
            .block_on(runtime.spawn(async move {
                tokio::join!(clock_f, wireless_f, window_f, bluetooth_f, battery_f)
            }))
            .unwrap();
    })
}

async fn run_clock_handler(time_format: String, app_channel: Sender<AppMessage>) {
    let mut clock_service_handle = ClockServiceHandle::new(app_channel);
    clock_service_handle.run(time_format).await;
}

async fn run_window_handler(app_channel: Sender<AppMessage>) {
    let mut window_service_handle = WindowServiceHandle::new(app_channel);
    window_service_handle.run().await;
}

async fn run_wireless_handler(app_channel: Sender<AppMessage>) {
    let mut wireless_service_handle = WirelessServiceHandle::new(app_channel);
    wireless_service_handle.run().await;
}

async fn run_bluetooth_handler(app_channel: Sender<AppMessage>) {
    let mut bluetooth_service_handle = BluetoothServiceHandle::new(app_channel);
    bluetooth_service_handle.run().await;
}

async fn run_battery_handler(app_channel: Sender<AppMessage>) {
    let mut battery_service_handle = BatteryServiceHandle::new(app_channel);
    battery_service_handle.run().await;
}
