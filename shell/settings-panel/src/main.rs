mod errors;
mod event_handler;
mod gui;
mod modules;
mod settings;
mod theme;
mod widgets;

use std::collections::HashMap;
use std::time::Duration;

use event_handler::zbus::ZbusServiceHandle;
use gui::SettingsPanel;
use mctk_core::{
    msg,
    reexports::smithay_client_toolkit::{
        reexports::calloop::{
            self,
            channel::Sender,
            timer::{TimeoutAction, Timer},
        },
        shell::wlr_layer,
    },
};
use mctk_smithay::{layer_surface::LayerOptions, WindowOptions};
use mctk_smithay::{layer_window::LayerWindowParams, WindowMessage};

use modules::{
    battery::component::get_battery_icons_map, bluetooth::component::get_bluetooth_icons_map,
    brightness::component::get_brightness_icons_map, cpu::component::get_cpu_icons_map,
    memory::component::get_memory_icons_map, rotation::component::get_rotation_icons_map,
    running_apps::component::get_running_apps_icons_map,
    settings::component::get_settings_icons_map, sound::component::get_sound_icons_map,
    wireless::component::get_wireless_icons_map,
};
use settings::SettingsPanelSettings;
use theme::SettingsPanelTheme;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::gui::Message;

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
        Err(_) => SettingsPanelSettings::default(),
    };

    // let custom_theme = match theme::read_theme_yml() {
    //     Ok(theme) => theme,
    //     Err(_) => SettingsPanelTheme::default(),
    // };

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let fonts: HashMap<String, String> = settings.fonts.clone();

    let mut assets: HashMap<String, String> = HashMap::new();
    let mut svgs: HashMap<String, String> = HashMap::new();

    let modules = settings.modules.clone();

    let battery_assets = get_battery_icons_map(modules.battery.icon);
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
    svgs.extend(wireless_assets);
    svgs.extend(bluetooth_assets);
    svgs.extend(rotation_assets);
    svgs.extend(settings_assets);
    svgs.extend(running_apps_assets);
    svgs.extend(cpu_assets);
    svgs.extend(memory_assets);
    svgs.extend(sound_assets);
    svgs.extend(brightness_assets);

    let namespace = settings.app.id.clone().unwrap_or_default();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::TOP | wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT,
        layer: wlr_layer::Layer::Overlay,
        keyboard_interactivity: wlr_layer::KeyboardInteractivity::Exclusive,
        namespace: Some(namespace.clone()),
        zone: window_opts.height as i32,
    };

    let (mut app, mut event_loop, window_tx) =
        mctk_smithay::layer_window::LayerWindow::open_blocking::<SettingsPanel>(
            LayerWindowParams {
                title: settings.title.clone(),
                namespace,
                window_opts,
                fonts,
                assets,
                layer_shell_opts,
                svgs,
            },
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
        event_loop
            .dispatch(Duration::from_millis(16), &mut app)
            .unwrap();
    }
    //End

    Ok(())
}

async fn init_services(sender: Sender<Message>) {
    let mut zbus_service_handle = ZbusServiceHandle::new();
    let sender_clone_1 = sender.clone();
    let _ = tokio::spawn(async move {
        info!(task = "init_services", "Starting zbus service");
        zbus_service_handle.run(sender_clone_1).await;
    });
}
