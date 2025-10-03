mod constants;
mod errors;
mod gui;
mod home_screen_ui;
mod lock_gui;
mod lock_screen_ui;
mod modules;
mod navigation_bar;
mod navigation_bar_ui;
mod pages;
mod settings;
mod shared;
mod theme;
mod types;
mod utils;

use desktop_entries::{DesktopEntries, DesktopEntry};
use futures::StreamExt;
use home_screen_ui::launch_homescreen;
use lock_screen_ui::launch_lockscreen;
use logind::get_current_session;
use mctk_core::reexports::smithay_client_toolkit::shell::wlr_layer::Layer;
use modules::applications::model::DesktopEntriesModel;
use modules::bluetooth::handler::BluetoothServiceHandle;
use modules::cpu::handler::CPUServiceHandle;
use modules::home::handler::HomeButtonHandler;
use modules::memory::handler::MemoryHandle;
use modules::name::handler::MachineNameHandle;
use modules::networking::handler::NetworkingHandle;
use modules::running_apps::app_manager::{AppManagerMessage, AppManagerService};
use modules::running_apps::running_app::AppDetails;
use modules::settings_panel::brightness::handler::BrightnessServiceHandle;
use modules::settings_panel::rotation::component::get_rotation_icons_map;
use modules::settings_panel::sound::handler::SoundServiceHandle;
use modules::uptime::handler::UptimeHandle;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::thread::{self, JoinHandle};
use tokio::runtime::Builder;
use tokio::select;
use tokio::sync::mpsc::{self, Receiver};
use types::BluetoothStatus;
use upower::BatteryStatus;
use utils::{
    get_battery_icons_charging_map, get_battery_icons_map, get_bluetooth_icons_map,
    get_wireless_icons_map,
};

use mctk_core::{
    reexports::{
        cosmic_text,
        smithay_client_toolkit::reexports::calloop::{self, channel::Sender},
    },
    types::AssetParams,
};
use settings::LauncherSettings;
use theme::LauncherTheme;
use tracing_subscriber::EnvFilter;

use crate::navigation_bar::launch_navigation_bar;

#[derive(Default, Debug, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
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
pub enum RunningAppsMessage {
    Status { count: i32 },
    Toggle { value: bool },
}

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
pub enum RotationMessage {}

#[derive(Debug)]
enum AppMessage {
    CPUUsage {
        usage: f32,
    },
    Uptime {
        uptime: String,
    },
    MachineName {
        name: String,
    },
    Net {
        online: bool,
    },
    Memory {
        total: u64,
        used: u64,
    },
    RunningApps {
        message: RunningAppsMessage,
    },
    ChangeLayer(Layer),
    Bluetooth {
        message: BluetoothMessage,
    },
    Sound {
        message: SoundMessage,
    },
    Brightness {
        message: BrightnessMessage,
    },
    AppsUpdated {
        apps: Vec<AppDetails>,
        app_id: String,
        active_apps_count: i32,
    },
    ShutDown,
    Restart,
    Unlock,
    AppOpen {
        app_id: String,
    },
    AppClose {
        app_id: String,
    },
    MinimizeAll,
}

#[derive(Debug, Clone)]
pub struct UiParams {
    fonts: cosmic_text::fontdb::Database,
    assets: HashMap<String, AssetParams>,
    svgs: HashMap<String, String>,
    settings: LauncherSettings,
    theme: LauncherTheme,
}

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .init();

    let settings = match crate::settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(e) => {
            println!("error while reading settings {:?}", e);

            LauncherSettings::default()
        }
    };

    let theme = match crate::theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => LauncherTheme::default(),
    };

    let mut fonts: cosmic_text::fontdb::Database = cosmic_text::fontdb::Database::new();
    for path in settings.fonts.paths.clone() {
        if let Ok(content) = fs::read(Path::new(&path)) {
            fonts.load_font_data(content);
        }
    }

    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();
    svgs.insert(
        "link".to_string(),
        "/usr/share/mechanix/shell/launcher/assets/icons/link.svg".to_string(),
    );

    let modules = settings.modules.clone();

    let battery_assets = get_battery_icons_map(modules.battery.icon);
    let battery_charging_assets = get_battery_icons_charging_map(modules.battery.charging_icon);
    let bluetooth_assets = get_bluetooth_icons_map(modules.bluetooth.icon);
    let wireless_assets = get_wireless_icons_map(modules.wireless.icon);
    let rotation_assets = get_rotation_icons_map(modules.rotation.icon);
    let power_icon = modules.power.icon.default;
    let lock_icon = modules.lock.icon.default;
    let settings_icon = modules.settings.icon.default;
    let search_icon = modules.search.icon.default;
    let launch_icon = modules.launch.icon.default;
    let delete_icon = modules.delete.icon.default;
    let close_icon = modules.close.icon.default;
    let terminal_icon = modules.terminal.icon;
    let shutdown_icon = modules.power_options.shutdown.icon;
    let restart_icon = modules.power_options.restart.icon;

    assets.extend(battery_assets);
    assets.extend(battery_charging_assets);
    assets.extend(wireless_assets);
    assets.extend(bluetooth_assets);
    assets.extend(rotation_assets);
    assets.insert("power_icon".to_string(), AssetParams::new(power_icon));
    assets.insert("lock_icon".to_string(), AssetParams::new(lock_icon));
    assets.insert("settings_icon".to_string(), AssetParams::new(settings_icon));
    assets.insert("search_icon".to_string(), AssetParams::new(search_icon));
    assets.insert("launch_icon".to_string(), AssetParams::new(launch_icon));
    assets.insert("delete_icon".to_string(), AssetParams::new(delete_icon));
    assets.insert("shutdown_icon".to_string(), AssetParams::new(shutdown_icon));
    assets.insert("restart_icon".to_string(), AssetParams::new(restart_icon));
    assets.insert("close_icon".to_string(), AssetParams::new(close_icon));
    assets.insert("terminal_icon".to_string(), AssetParams::new(terminal_icon));

    assets.insert(
        "app_list_bg".to_string(),
        AssetParams::new(
            "/usr/share/mechanix/shell/launcher/assets/icons/app_list_bg.png".to_string(),
        ),
    );

    assets.insert(
        "files".to_string(),
        AssetParams::new("/usr/share/mechanix/shell/launcher/assets/icons/files.png".to_string()),
    );
    assets.insert(
        "music".to_string(),
        AssetParams::new("/usr/share/mechanix/shell/launcher/assets/icons/music.png".to_string()),
    );
    assets.insert(
        "terminal".to_string(),
        AssetParams::new(
            "/usr/share/mechanix/shell/launcher/assets/icons/terminal.png".to_string(),
        ),
    );
    assets.insert(
        "chromium".to_string(),
        AssetParams::new(
            "/usr/share/mechanix/shell/launcher/assets/icons/chromium.png".to_string(),
        ),
    );
    assets.insert(
        "notes".to_string(),
        AssetParams::new("/usr/share/mechanix/shell/launcher/assets/icons/notes.png".to_string()),
    );
    assets.insert(
        "settings".to_string(),
        AssetParams::new(
            "/usr/share/mechanix/shell/launcher/assets/icons/settings.png".to_string(),
        ),
    );
    assets.insert(
        "memory_bg".to_string(),
        AssetParams::new(
            "/usr/share/mechanix/shell/launcher/assets/icons/memory_bg.png".to_string(),
        ),
    );
    assets.insert(
        "storage_bg".to_string(),
        AssetParams::new(
            "/usr/share/mechanix/shell/launcher/assets/icons/storage_bg.png".to_string(),
        ),
    );
    assets.insert(
        "firefox".to_string(),
        AssetParams::new("/usr/share/mechanix/shell/launcher/assets/icons/firefox.png".to_string()),
    );

    assets.insert(
        "hacker_news_icon".to_string(),
        AssetParams::new(
            "/usr/share/mechanix/shell/launcher/assets/icons/hacker_news_icon.png".to_string(),
        ),
    );

    let background = modules.background.icon.default;
    if background.len() > 0 {
        assets.insert("background".to_string(), AssetParams::new(background));
    }

    // for app in installed_apps.clone() {
    //     if let Some(icon_path) = app.icon_path.clone() {
    //         match icon_path.extension().and_then(|ext| ext.to_str()) {
    //             Some("png") => {
    //                 assets.insert(
    //                     app.name,
    //                     AssetParams::new(icon_path.to_str().unwrap().to_string()),
    //                 );
    //             }
    //             Some("svg") => {
    //                 svgs.insert(app.name, icon_path.to_str().unwrap().to_string());
    //             }
    //             _ => (),
    //         }
    //     }
    // }

    let ui_params = UiParams {
        fonts,
        assets,
        svgs,
        settings,
        theme,
    };

    let ui_params_1 = ui_params.clone();
    let _ = std::thread::spawn(move || {
        let _ = launch_homescreen(ui_params_1);
    });

    let ui_params_2 = ui_params.clone();

    let _ = std::thread::spawn(move || {
        let _ = launch_navigation_bar(ui_params_2);
    });

    let session = get_current_session().await.unwrap();
    let mut lock = session.receive_lock().await.unwrap();
    let mut unlock = session.receive_unlock().await.unwrap();
    let is_session_locked = session.locked_hint().await.unwrap();
    if is_session_locked {
        let ui_params_1 = ui_params.clone();
        let _ = std::thread::spawn(move || {
            let _ = launch_lockscreen(ui_params_1);
        });
    }
    loop {
        select! {
            _ =  lock.next() =>  {
                let session = get_current_session().await.unwrap();
                let is_session_locked = session.locked_hint().await.unwrap();
                println!("is_session_locked {:?}", is_session_locked);
                if !is_session_locked {
                    let _ = session.set_locked_hint(true).await;
                    let ui_params_1 = ui_params.clone();
                    let _ = std::thread::spawn(move || {
                        let _ = launch_lockscreen( ui_params_1);
                    });
                    let _ = session.set_locked_hint(false).await;
                }
            }, _ = unlock.next() => {
                println!("logind unlock");
                let session = get_current_session().await.unwrap();
                let _ = session.set_locked_hint(false).await;
            }
        }
    }
}

pub struct InitServicesParamsHome {
    pub settings: LauncherSettings,
    pub app_channel: Sender<AppMessage>,
    // pub wireless_msg_rx: Receiver<WirelessMessage>,
    pub bluetooth_msg_rx: Receiver<BluetoothMessage>,
    pub brightness_msg_rx: Receiver<BrightnessMessage>,
    pub sound_msg_rx: Receiver<SoundMessage>,
    pub app_manager_msg_rx: Receiver<AppManagerMessage>,
}
pub struct InitServicesParamsNav {
    pub app_channel: Sender<AppMessage>,
    pub app_manager_msg_rx: Receiver<AppManagerMessage>,
}

pub struct InitServicesParamsLock {
    pub settings: LauncherSettings,
    pub app_channel: Sender<AppMessage>,
    // pub wireless_msg_rx: Receiver<WirelessMessage>,
    pub bluetooth_msg_rx: Receiver<BluetoothMessage>,
}

fn init_services_home(init_params: InitServicesParamsHome) -> JoinHandle<()> {
    let InitServicesParamsHome {
        settings,
        app_channel,
        bluetooth_msg_rx,
        brightness_msg_rx,
        sound_msg_rx,
        app_manager_msg_rx,
    } = init_params;
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let bluetooth_f = run_bluetooth_handler(app_channel.clone(), bluetooth_msg_rx);
        // let rotation_f = run_rotation_handler(app_channel.clone(), rotation_msg_rx);
        let brightness_f = run_brightness_handler(app_channel.clone(), brightness_msg_rx);
        let sound_f = run_sound_handler(app_channel.clone(), sound_msg_rx);
        let cpu_f = run_cpu_handler(app_channel.clone());
        let memory_f = run_memory_handler(app_channel.clone());
        let uptime_f = run_uptime_handler(app_channel.clone());
        let machine_name_f = run_machine_name_handler(app_channel.clone());
        let net_f = run_net_handler(app_channel.clone());
        // let running_apps_f = run_running_apps_handler(app_channel.clone());
        let app_manager_f = run_app_manager_handler(app_manager_msg_rx, app_channel.clone());
        let home_button_f = run_home_button_handler(app_channel.clone());

        runtime
            .block_on(runtime.spawn(async move {
                tokio::join!(
                    // wireless_f,
                    bluetooth_f,
                    // rotation_f,
                    brightness_f,
                    sound_f,
                    cpu_f,
                    memory_f,
                    uptime_f,
                    machine_name_f,
                    net_f,
                    // running_apps_f,
                    app_manager_f,
                    home_button_f
                )
            }))
            .unwrap();
    })
}

fn init_services_lock(init_params: InitServicesParamsLock) -> JoinHandle<()> {
    let InitServicesParamsLock {
        settings,
        app_channel,
        // wireless_msg_rx,
        bluetooth_msg_rx,
    } = init_params;
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let bluetooth_f = run_bluetooth_handler(app_channel.clone(), bluetooth_msg_rx);

        runtime
            .block_on(runtime.spawn(async move {
                tokio::join!(
                    // wireless_f,
                    bluetooth_f,
                )
            }))
            .unwrap();
    })
}

fn init_services_nav(init_params: InitServicesParamsNav) -> JoinHandle<()> {
    let InitServicesParamsNav {
        app_channel,
        app_manager_msg_rx,
    } = init_params;
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let app_manager_f = run_app_manager_handler(app_manager_msg_rx, app_channel.clone());

        runtime
            .block_on(runtime.spawn(async move { tokio::join!(app_manager_f,) }))
            .unwrap();
    })
}

async fn run_bluetooth_handler(
    app_channel: Sender<AppMessage>,
    bluetooth_msg_rx: Receiver<BluetoothMessage>,
) {
    let mut bluetooth_service_handle = BluetoothServiceHandle::new(app_channel);
    bluetooth_service_handle.run(bluetooth_msg_rx).await;
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

async fn run_cpu_handler(app_channel: Sender<AppMessage>) {
    let mut cpu_handle = CPUServiceHandle::new(app_channel);
    cpu_handle.run().await;
}

async fn run_memory_handler(app_channel: Sender<AppMessage>) {
    let mut memory_handle = MemoryHandle::new(app_channel);
    memory_handle.run().await;
}

async fn run_uptime_handler(app_channel: Sender<AppMessage>) {
    let mut uptime_handle = UptimeHandle::new(app_channel);
    uptime_handle.run().await;
}

async fn run_machine_name_handler(app_channel: Sender<AppMessage>) {
    let mut machine_name_handle = MachineNameHandle::new(app_channel);
    machine_name_handle.run().await;
}

async fn run_net_handler(app_channel: Sender<AppMessage>) {
    let mut net_handle = NetworkingHandle::new(app_channel);
    net_handle.run().await;
}

// async fn run_running_apps_handler(app_channel: Sender<AppMessage>) {
//     let mut running_apps = RunningAppsHandle::new(app_channel);
//     running_apps.run().await;
// }

async fn run_home_button_handler(app_channel: Sender<AppMessage>) {
    let home_button_handle = HomeButtonHandler::new(app_channel);
    home_button_handle.run().await;
}

async fn run_app_manager_handler(
    msg_rx: mpsc::Receiver<AppManagerMessage>,
    app_channel_tx: calloop::channel::Sender<AppMessage>,
) {
    // create the app manager instance
    let mut app_manager_handler = AppManagerService::new();

    // start the app manager handler
    let _ = app_manager_handler.run(msg_rx, app_channel_tx).await;
}
