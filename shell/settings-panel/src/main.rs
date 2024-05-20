mod errors;
mod gui;
mod modules;
mod settings;
mod theme;
mod types;
mod widgets;

use echo_client::EchoClient;
use modules::battery::component::get_battery_icons_charging_map;
use std::time::Duration;
use std::{collections::HashMap, env};
use tokio::{
    runtime::Builder,
    sync::{mpsc::Receiver, oneshot},
};

use gui::SettingsPanel;
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
    types::AssetParams,
};
use mctk_smithay::layer_shell::layer_window::LayerWindow;
use mctk_smithay::layer_shell::layer_window::LayerWindowParams;
use mctk_smithay::{layer_shell::layer_surface::LayerOptions, WindowMessage};
use mctk_smithay::{WindowInfo, WindowOptions};

use modules::{
    battery::component::get_battery_icons_map,
    bluetooth::component::get_bluetooth_icons_map,
    brightness::component::get_brightness_icons_map,
    cpu::{component::get_cpu_icons_map, handler::CpuServiceHandle},
    memory::{component::get_memory_icons_map, handler::MemoryServiceHandle},
    rotation::component::get_rotation_icons_map,
    running_apps::component::get_running_apps_icons_map,
    settings::component::get_settings_icons_map,
    sound::component::get_sound_icons_map,
    wireless::component::get_wireless_icons_map,
};
use modules::{
    battery::handler::BatteryServiceHandle, running_apps::handler::RunningAppsServiceHandle,
};
use modules::{
    bluetooth::handler::BluetoothServiceHandle, brightness::handler::BrightnessServiceHandle,
};
use modules::{sound::handler::SoundServiceHandle, wireless::handler::WirelessServiceHandle};
use settings::SettingsPanelSettings;
use std::thread::{self, JoinHandle};
use theme::SettingsPanelTheme;
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::EnvFilter;
use types::{BatteryStatus, BluetoothStatus, WirelessStatus};

use crate::gui::Message;

#[derive(Debug)]
pub enum WirelessMessage {
    Status { status: WirelessStatus },
    Toggle { value: Option<bool> },
}

#[derive(Debug)]
pub enum BluetoothMessage {
    Status { status: BluetoothStatus },
    Toggle { value: Option<bool> },
}

#[derive(Debug)]
pub enum BatteryMessage {
    Status { level: u8, status: BatteryStatus },
}

#[derive(Debug)]
pub enum CpuMessage {
    Usage { usage: f32 },
}

#[derive(Debug)]
pub enum MemoryMessage {
    Usage { used: u64, total: u64 },
}

#[derive(Debug)]
pub enum RunningAppsMessage {
    Count { count: i32 },
}

#[derive(Debug)]
pub enum RotationMessage {}

#[derive(Debug)]
pub enum SoundMessage {
    Value { value: u8 },
    Change { value: u8 },
}

#[derive(Debug)]
pub enum BrightnessMessage {
    Value { value: u8 },
    Change { value: u8 },
}

#[derive(Debug)]
pub enum AppMessage {
    Wireless { message: WirelessMessage },
    Bluetooth { message: BluetoothMessage },
    Battery { message: BatteryMessage },
    Cpu { message: CpuMessage },
    Memory { message: MemoryMessage },
    Sound { message: SoundMessage },
    Brightness { message: BrightnessMessage },
    RunningApps { message: RunningAppsMessage },
    Show,
    Hide,
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
        Err(_) => SettingsPanelSettings::default(),
    };

    // let custom_theme = match theme::read_theme_yml() {
    //     Ok(theme) => theme,
    //     Err(_) => SettingsPanelTheme::default(),
    // };

    let width = settings.window.size.0 as u32;
    let height = settings.window.size.1 as u32;
    let window_opts = WindowOptions {
        height,
        width,
        scale_factor: 1.0,
    };

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    let battery_assets = get_battery_icons_map(modules.battery.icon);
    let battery_charging_assets = get_battery_icons_charging_map(modules.battery.charging_icon);
    let bluetooth_assets = get_bluetooth_icons_map(modules.bluetooth.icon);
    let wireless_assets = get_wireless_icons_map(modules.wireless.icon);
    let rotation_assets = get_rotation_icons_map(modules.rotation.icon);
    let settings_assets = get_settings_icons_map(modules.settings.icon);
    let running_apps_assets = get_running_apps_icons_map(modules.running_apps.icon);
    let cpu_assets = get_cpu_icons_map(modules.cpu.icon);
    let memory_assets = get_memory_icons_map(modules.memory.icon);
    let sound_assets = get_sound_icons_map(modules.sound.icon);
    let brightness_assets = get_brightness_icons_map(modules.brightness.icon);

    svgs.extend(battery_assets);
    svgs.extend(battery_charging_assets);
    svgs.extend(wireless_assets);
    svgs.extend(bluetooth_assets);
    svgs.extend(rotation_assets);
    svgs.extend(settings_assets);
    svgs.extend(running_apps_assets);
    svgs.extend(cpu_assets);
    svgs.extend(memory_assets);
    svgs.extend(sound_assets);
    svgs.extend(brightness_assets);

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.settings-panel"));
    let namespace = app_id.clone();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::BOTTOM | wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT,
        layer: wlr_layer::Layer::Top,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: 0 as i32,
    };

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let window_info = WindowInfo {
        id: app_id,
        title: settings.title.clone(),
        namespace,
    };

    let (app_channel, app_receiver) = calloop::channel::channel();
    let app_channel2 = app_channel.clone();
    let (mut app, mut event_loop, window_tx) =
        LayerWindow::open_blocking::<SettingsPanel, AppMessage>(
            LayerWindowParams {
                window_info,
                window_opts: window_opts,
                fonts,
                assets,
                svgs,
                layer_shell_opts,
                ..Default::default()
            },
            Some(app_channel),
        );

    let handle = event_loop.handle();

    let window_tx_2 = window_tx.clone();
    // create mpsc channel for interacting with the input_method handler
    let (wireless_msg_tx, wireless_msg_rx) = mpsc::channel(128);
    let (bluetooth_msg_tx, bluetooth_msg_rx) = mpsc::channel(128);
    let (rotation_msg_tx, rotation_msg_rx) = mpsc::channel(128);
    let (brightness_msg_tx, brightness_msg_rx) = mpsc::channel(128);
    let (sound_msg_tx, sound_msg_rx) = mpsc::channel(128);

    let _ = handle.insert_source(app_receiver, move |event, _, _| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Wireless { message } => match message {
                    WirelessMessage::Status { status } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Wireless { status }),
                        });
                    }
                    WirelessMessage::Toggle { .. } => {
                        let wireless_msg_tx_cloned = wireless_msg_tx.clone();
                        futures::executor::block_on(async move {
                            //let (tx, rx) = oneshot::channel();
                            let res = wireless_msg_tx_cloned.clone().send(message).await;
                            //let res = rx.await.expect("no reply from service");
                        });
                    }
                },
                AppMessage::Bluetooth { message } => match message {
                    BluetoothMessage::Status { status } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Bluetooth { status }),
                        });
                    }
                    BluetoothMessage::Toggle { .. } => {
                        let bluetooth_msg_tx_cloned = bluetooth_msg_tx.clone();
                        futures::executor::block_on(async move {
                            //let (tx, rx) = oneshot::channel();
                            let res = bluetooth_msg_tx_cloned.clone().send(message).await;
                            //let res = rx.await.expect("no reply from service");
                        });
                    }
                },
                AppMessage::Battery { message } => match message {
                    BatteryMessage::Status { level, status } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Battery { level, status }),
                        });
                    }
                },
                AppMessage::Cpu { message } => match message {
                    CpuMessage::Usage { usage } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Cpu {
                                usage: usage as i32
                            }),
                        });
                    }
                },
                AppMessage::Memory { message } => match message {
                    MemoryMessage::Usage { used, total } => {
                        let usage = used as f32 / total as f32 * 100.;
                        //println!("MemoryMessage::Usage {} {} {} ", used, total, usage);
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Memory { usage: used }),
                        });
                    }
                },
                AppMessage::Brightness { message } => match message {
                    BrightnessMessage::Value { value } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Brightness {
                                value: value as i32
                            }),
                        });
                    }
                    BrightnessMessage::Change { .. } => {
                        let brightness_msg_tx_cloned = brightness_msg_tx.clone();
                        futures::executor::block_on(async move {
                            //let (tx, rx) = oneshot::channel();
                            let res = brightness_msg_tx_cloned.clone().send(message).await;
                            //let res = rx.await.expect("no reply from service");
                        });
                    }
                },
                AppMessage::Sound { message } => match message {
                    SoundMessage::Value { value } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::Sound {
                                value: value as i32
                            }),
                        });
                    }
                    SoundMessage::Change { .. } => {
                        let sound_msg_tx_cloned = sound_msg_tx.clone();
                        futures::executor::block_on(async move {
                            //let (tx, rx) = oneshot::channel();
                            let res = sound_msg_tx_cloned.clone().send(message).await;
                            //let res = rx.await.expect("no reply from service");
                        });
                    }
                },
                AppMessage::RunningApps { message } => match message {
                    RunningAppsMessage::Count { count } => {
                        let _ = window_tx_2.send(WindowMessage::Send {
                            message: msg!(Message::RunningApps { count }),
                        });
                    }
                },
                AppMessage::Show => {
                    println!("AppMessage::Show");
                    let _ = window_tx_2
                        .clone()
                        .send(WindowMessage::Resize { width, height });
                }
                AppMessage::Hide => {
                    println!("AppMessage::Hide");
                    let _ = window_tx_2.clone().send(WindowMessage::Resize {
                        width: 1,
                        height: 1,
                    });
                }
                _ => (),
            },
            calloop::channel::Event::Closed => {}
        };
    });

    init_services(
        app_channel2,
        wireless_msg_rx,
        bluetooth_msg_rx,
        rotation_msg_rx,
        brightness_msg_rx,
        sound_msg_rx,
    );

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    //End

    Ok(())
}

fn init_services(
    app_channel: Sender<AppMessage>,
    wireless_msg_rx: Receiver<WirelessMessage>,
    bluetooth_msg_rx: Receiver<BluetoothMessage>,
    rotation_msg_rx: Receiver<RotationMessage>,
    brightness_msg_rx: Receiver<BrightnessMessage>,
    sound_msg_rx: Receiver<SoundMessage>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let wireless_f = run_wireless_handler(app_channel.clone(), wireless_msg_rx);
        let bluetooth_f = run_bluetooth_handler(app_channel.clone(), bluetooth_msg_rx);
        let battery_f = run_battery_handler(app_channel.clone());
        let running_apps_f = run_running_apps_handler(app_channel.clone());
        let cpu_f = run_cpu_handler(app_channel.clone());
        let memory_f = run_memory_handler(app_channel.clone());
        let brightness_f = run_brightness_handler(app_channel.clone(), brightness_msg_rx);
        let sound_f = run_sound_handler(app_channel.clone(), sound_msg_rx);

        runtime
            .block_on(runtime.spawn(async move {
                tokio::join!(
                    wireless_f,
                    bluetooth_f,
                    battery_f,
                    running_apps_f,
                    cpu_f,
                    memory_f,
                    brightness_f,
                    sound_f,
                )
            }))
            .unwrap();
    })
}

async fn run_wireless_handler(
    app_channel: Sender<AppMessage>,
    wireless_msg_rx: Receiver<WirelessMessage>,
) {
    let mut wireless_service_handle = WirelessServiceHandle::new(app_channel);
    wireless_service_handle.run(wireless_msg_rx).await;
}

async fn run_bluetooth_handler(
    app_channel: Sender<AppMessage>,
    bluetooth_msg_rx: Receiver<BluetoothMessage>,
) {
    let mut bluetooth_service_handle = BluetoothServiceHandle::new(app_channel);
    bluetooth_service_handle.run(bluetooth_msg_rx).await;
}

async fn run_battery_handler(app_channel: Sender<AppMessage>) {
    let mut battery_service_handle = BatteryServiceHandle::new(app_channel);
    battery_service_handle.run().await;
}

async fn run_running_apps_handler(app_channel: Sender<AppMessage>) {
    let mut running_apps_service_handle = RunningAppsServiceHandle::new(app_channel);
    running_apps_service_handle.run().await;
}

async fn run_cpu_handler(app_channel: Sender<AppMessage>) {
    let mut cpu_service_handle = CpuServiceHandle::new(app_channel);
    cpu_service_handle.run().await;
}

async fn run_memory_handler(app_channel: Sender<AppMessage>) {
    let mut memory_service_handle = MemoryServiceHandle::new(app_channel);
    memory_service_handle.run().await;
}
async fn run_brightness_handler(
    app_channel: Sender<AppMessage>,
    brightness_msg_rx: Receiver<BrightnessMessage>,
) {
    let mut brightness_service_handle = BrightnessServiceHandle::new(app_channel);
    brightness_service_handle.run(brightness_msg_rx).await;
}
async fn run_sound_handler(app_channel: Sender<AppMessage>, sound_msg_rx: Receiver<SoundMessage>) {
    let mut sound_service_handle = SoundServiceHandle::new(app_channel);
    sound_service_handle.run(sound_msg_rx).await;
}
