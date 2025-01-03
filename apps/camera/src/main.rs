mod components;
mod config;
mod constants;
mod contexts;

use components::footer::Footer;
use components::preview::Preview;
use components::settings::Settings;
use contexts::camera::Camera;
use contexts::state::State;
use mctk_core::context::Model;
use mctk_core::prelude::*;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::{self, channel::Event};
use mctk_core::renderables::Renderable;
use mctk_smithay::xdg_shell::xdg_window::{self, XdgWindow, XdgWindowParams};
use mctk_smithay::{WindowInfo, WindowMessage, WindowOptions};
use smithay_client_toolkit::reexports::calloop::channel::Sender;
use std::any::Any;
use std::collections::HashMap;

// App level channel
#[derive(Debug)]
pub enum AppMessage {
    Exit,
}

#[derive(Debug, Clone)]
pub struct AppParams {
    app_channel: Option<calloop::channel::Sender<AppMessage>>,
}

#[derive(Debug, Default)]
pub struct AppState {
    app_channel: Option<Sender<AppMessage>>,
}

#[component(State = "AppState")]
#[derive(Debug, Default)]
pub struct App {}

#[state_component_impl(AppState)]
impl Component for App {
    fn init(&mut self) {
        let app_state = AppState { app_channel: None };
        self.state = Some(app_state);
    }

    fn render(&mut self, _: RenderContext) -> Option<Vec<Renderable>> {
        None
    }

    fn view(&self) -> Option<Node> {
        let mut base = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size: size_pct!(100.0),
                direction: Direction::Column,
                axis_alignment: Alignment::Start,
                cross_alignment: Alignment::Center,
            ]
        );
        if State::get_settings_state() {
            base = base.push(node!(Settings::new()));
        }
        base = base.push(node!(Footer::new()));
        base = base.push(node!(Preview::new()));
        Some(base)
    }
}

use std::env;

#[derive(Debug)]
struct CameraConfig {
    fps: u32,
    device_index: usize,
    height: u32,
    width: u32,
    auto: bool,
}

impl CameraConfig {
    fn new() -> Self {
        // Default values for camera configuration
        CameraConfig {
            fps: 30,
            device_index: 0,
            height: 240,
            width: 320,
            auto: true,
        }
    }

    fn parse_args() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut config = CameraConfig::new();

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--fps" => {
                    if i + 1 < args.len() {
                        config.fps = args[i + 1].parse().unwrap_or(config.fps);
                        i += 1;
                    }
                }
                "--camera" => {
                    if i + 1 < args.len() {
                        config.device_index = args[i + 1].parse().unwrap_or(config.device_index);
                        i += 1;
                    }
                }
                "--height" => {
                    if i + 1 < args.len() {
                        config.height = args[i + 1].parse().unwrap_or(config.height);
                        i += 1;
                    }
                }
                "--width" => {
                    if i + 1 < args.len() {
                        config.width = args[i + 1].parse().unwrap_or(config.width);
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        config
    }
}

// Layer Surface App
#[tokio::main]
async fn main() {
    let config = CameraConfig::parse_args();
    println!("Camera Configuration: {:?}", config);

    Camera::get().fps.set(config.fps);
    Camera::get().device_index.set(config.device_index);
    Camera::get().height.set(config.height);
    Camera::get().width.set(config.width);
    Camera::init();

    let ui_t = std::thread::spawn(move || {
        let _ = launch_ui();
    });
    ui_t.join().unwrap();
}

impl RootComponent<AppParams> for App {
    fn root(&mut self, w: &dyn std::any::Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
    }
}

fn launch_ui() -> anyhow::Result<()> {
    let mut assets: HashMap<String, AssetParams> = HashMap::new();
    let svgs: HashMap<String, String> = HashMap::new();

    let settings = match crate::config::read_settings_yml() {
        Ok(settings) => settings,
        Err(e) => {
            println!("error while reading settings {:?}", e);
            crate::config::MainSettings::default()
        }
    };
    println!("settings {:?}", settings.modules);
    assets.insert(
        "back_icon".to_string(),
        AssetParams::new(settings.modules.back_icon),
    );

    assets.insert(
        "settings_icon".to_string(),
        AssetParams::new(settings.modules.settings_icon),
    );

    let mut fonts = cosmic_text::fontdb::Database::new();
    fonts.load_system_fonts();

    let window_opts = WindowOptions {
        height: settings.window.size.1 as u32,
        width: settings.window.size.0 as u32,
        scale_factor: 1.0,
    };

    let app_id = settings
        .app
        .id
        .unwrap_or(String::from("org.mechanix.camera"));
    let title = if !settings.title.is_empty() {
        settings.title.clone()
    } else {
        "Camera".to_string()
    };
    let namespace = app_id.clone();

    let window_info = WindowInfo {
        id: app_id,
        title,
        namespace,
    };

    let (app_channel_tx, app_channel_rx) = calloop::channel::channel();
    let (mut app, mut event_loop, window_tx) = XdgWindow::open_blocking::<App, AppParams>(
        XdgWindowParams {
            window_info,
            window_opts,
            fonts,
            assets,
            svgs,
            ..Default::default()
        },
        AppParams {
            app_channel: Some(app_channel_tx),
        },
    );
    let handle = event_loop.handle();
    let window_tx_2 = window_tx.clone();
    let window_tx_channel = window_tx.clone();
    let context_handler = context::get_static_context_handler();
    context_handler.register_on_change(Box::new(move || {
        window_tx_channel
            .send(WindowMessage::Send { message: msg!(0) })
            .unwrap();
    }));
    Camera::get().register_context_handler(context_handler);
    State::get().register_context_handler(context_handler);

    let _ = handle.insert_source(app_channel_rx, move |event: Event<AppMessage>, _, app| {
        match event {
            // calloop::channel::Event::Msg(msg) => app.app.push_message(msg),
            calloop::channel::Event::Msg(msg) => match msg {
                AppMessage::Exit => {
                    println!("app channel message {:?}", AppMessage::Exit);
                    let _ = window_tx_2.send(WindowMessage::WindowEvent {
                        event: mctk_smithay::WindowEvent::CloseRequested,
                    });
                }
            },
            calloop::channel::Event::Closed => {
                println!("calloop::event::closed");
            }
        };
    });

    loop {
        let _ = event_loop.dispatch(None, &mut app);

        if app.is_exited {
            break;
        }
    }

    Ok(())
}
