mod components;
mod errors;
mod gui;
mod modules;
mod pages;
mod settings;
mod theme;
mod types;

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
    types::{AssetParams, ImgFilter},
};
use mctk_smithay::{
    layer_surface::LayerOptions, lock_window::SessionLockWindowParams, WindowOptions,
};
use mctk_smithay::{layer_window::LayerWindowParams, WindowMessage};

use modules::{
    battery::{
        component::{get_battery_icons_charging_map, get_battery_icons_map},
        handler::BatteryServiceHandle,
    },
    bluetooth::{component::get_bluetooth_icons_map, handler::BluetoothServiceHandle},
    clock::handler::ClockServiceHandle,
    wireless::{component::get_wireless_icons_map, handler::WirelessServiceHandle},
};
use settings::LockScreenSettings;
use std::thread::{self, JoinHandle};
use theme::LockScreenTheme;
use tokio::runtime::Builder;
use tracing::info;
use tracing_subscriber::EnvFilter;
use types::{BatteryStatus, BluetoothStatus, WirelessStatus};

use crate::gui::Message;

#[derive(Debug)]
pub enum AppMessage {
    Clock { current_time: String },
    Wireless { status: WirelessStatus },
    Bluetooth { status: BluetoothStatus },
    Battery { level: u8, status: BatteryStatus },
}

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

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
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

    if let Some(icon) = modules.home.icon.default {
        svgs.insert("home_icon".to_string(), icon);
    }

    if let Some(icon) = modules.back_space.icon.default {
        svgs.insert("backspace_icon".to_string(), icon);
    }

    if let Some(icon) = modules.background.icon.default {
        assets.insert(
            "background".to_string(),
            AssetParams {
                path: icon,
                filter: ImgFilter::GRAY,
                blur: None,
            },
        );
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

    let namespace = settings.app.id.clone();

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let (session_lock_tx, session_lock_rx) = calloop::channel::channel();
    let (app_channel, app_receiver) = calloop::channel::channel();
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
            Some(app_channel.clone()),
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
        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
}

fn init_services(settings: LockScreenSettings, app_channel: Sender<AppMessage>) -> JoinHandle<()> {
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let time_format = settings.modules.clock.format.clone();
        let clock_f = run_clock_handler(time_format, app_channel.clone());
        let wireless_f = run_wireless_handler(app_channel.clone());
        let bluetooth_f = run_bluetooth_handler(app_channel.clone());
        let battery_f = run_battery_handler(app_channel.clone());

        runtime
            .block_on(
                runtime.spawn(
                    async move { tokio::join!(clock_f, wireless_f, bluetooth_f, battery_f) },
                ),
            )
            .unwrap();
    })
}

async fn run_clock_handler(time_format: String, app_channel: Sender<AppMessage>) {
    let mut clock_service_handle = ClockServiceHandle::new(app_channel);
    clock_service_handle.run(time_format).await;
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
