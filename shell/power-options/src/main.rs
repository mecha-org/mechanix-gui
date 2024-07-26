mod errors;
mod gui;
mod service;
mod settings;
mod constants;

use mctk_core::ImgFilter;
use settings::PowerOptionsSettings;
use std::collections::HashMap;
use std::time::Duration;

use gui::PowerOptions;
use mctk_core::{
    reexports::{cosmic_text, smithay_client_toolkit::shell::wlr_layer},
    types::AssetParams,
};
use mctk_smithay::layer_shell::layer_surface::LayerOptions;
use mctk_smithay::layer_shell::layer_window::LayerWindow;
use mctk_smithay::layer_shell::layer_window::LayerWindowParams;
use mctk_smithay::{WindowInfo, WindowOptions};

use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
pub struct AppParams {}

#[derive(Debug)]
pub enum AppMessage {}

// Layer Surface App
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
            PowerOptionsSettings::default()
        }
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

    if let icon = modules.shutdown.icon {
        svgs.insert("shutdown_icon".to_string(), icon);
    }

    if let icon = modules.restart.icon {
        svgs.insert("restart_icon".to_string(), icon);
    }

    if let icon = modules.logout.icon {
        svgs.insert("logout_icon".to_string(), icon);
    }

    if let icon = modules.background.icon {
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

    let (mut app, mut event_loop, window_tx) = LayerWindow::open_blocking::<PowerOptions, AppParams>(
        LayerWindowParams {
            window_info,
            window_opts: window_opts,
            fonts,
            assets,
            svgs,
            layer_shell_opts,
            ..Default::default()
        },
        AppParams {},
    );

    loop {
        event_loop.dispatch(None, &mut app).unwrap();
    }
    //End

    Ok(())
}
