mod errors;
mod gui;
mod modules;
mod settings;

use mctk_core::ImgFilter;
use modules::power::handler::PowerServiceHandle;
use modules::power::service::PowerService;
use settings::PowerOptionsSettings;
use std::time::Duration;
use std::{collections::HashMap, env};
use tokio::sync::mpsc;
use tokio::{runtime::Builder, sync::mpsc::Receiver};

use gui::PowerOptions;
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

use std::thread::{self, JoinHandle};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::gui::Message;

#[derive(Debug)]
pub enum AppMessage {
    PowerOff,
    Reboot,
    Logout,
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
        Err(_) => PowerOptionsSettings::default(),
    };

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

    if let Some(icon) = modules.shutdown.icon {
        svgs.insert("shutdown_icon".to_string(), icon);
    }

    if let Some(icon) = modules.restart.icon {
        svgs.insert("restart_icon".to_string(), icon);
    }

    if let Some(icon) = modules.logout.icon {
        svgs.insert("logout_icon".to_string(), icon);
    }

    if let Some(icon) = modules.background.icon {
        assets.insert(
            "background".to_string(),
            AssetParams {
                path: icon,
                filter: ImgFilter::GRAY,
                blur: None,
            },
        );
    }

    let app_id = settings
        .app
        .id
        .clone()
        .unwrap_or(String::from("mechanix.shell.power-options"));
    let namespace = app_id.clone();

    let layer_shell_opts = LayerOptions {
        anchor: wlr_layer::Anchor::BOTTOM | wlr_layer::Anchor::LEFT | wlr_layer::Anchor::RIGHT,
        layer: wlr_layer::Layer::Overlay,
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
    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<PowerOptions, AppMessage>(
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
    let (power_msg_tx, power_msg_rx) = mpsc::channel(128);
    let _ = handle.insert_source(app_receiver, move |event, _, _| {
        let _ = match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => {
                let power_msg_tx_cloned = power_msg_tx.clone();
                futures::executor::block_on(async move {
                    let _ = power_msg_tx_cloned.clone().send(msg).await;
                });
            }
            calloop::channel::Event::Closed => {}
        };
    });

    init_services(app_channel2, power_msg_rx);

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
    power_msg_rx: Receiver<AppMessage>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let power_f = run_power_handler(app_channel.clone(), power_msg_rx);

        runtime
            .block_on(runtime.spawn(async move { tokio::join!(power_f) }))
            .unwrap();
    })
}
async fn run_power_handler(app_channel: Sender<AppMessage>, power_msg_rx: Receiver<AppMessage>) {
    let mut power_service_handle = PowerServiceHandle::new(app_channel);
    power_service_handle.run(power_msg_rx).await;
}
