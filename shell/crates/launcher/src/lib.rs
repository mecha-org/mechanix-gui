use std::process::{Child, Command};

use commons::prelude::*;
use dispatcher::{Dispatcher, prelude::*};
use gpui::{foreign_toplevel_management::ForeignToplevelHandle, *};
use settings::prelude::*;

struct Launcher {
    top_levels: Vec<ForeignToplevelHandle>,
}

impl Launcher {
    fn new() -> Self {
        Self {
            top_levels: Vec::new(),
        }
    }

    fn launch_app(&self, app_id: String, exec: String) {
        if let Some(tl) = self.get_app_top_level(&app_id) {
            tl.activate();
            return;
        } else {
            let _ = self.start_app(exec);
        }
    }

    fn get_app_top_level(&self, app_id: &String) -> Option<&ForeignToplevelHandle> {
        self.top_levels.iter().find(|tl| {
            let Some(tl_app_id) = tl.app_id() else {
                return false;
            };
            tl_app_id.to_lowercase() == SharedString::from(app_id.to_lowercase())
        })
    }

    fn start_app(&self, exec: String) {
        if !exec.is_empty() {
            let mut args: Vec<String> = vec!["-c".to_string()];
            args.push(exec.to_string());
            let res = spawn_command("sh".to_string(), args);
            if let Err(why) = res {
                println!("Launcher::start_app() error {:?}", why);
            };
        }
    }
}

pub fn spawn_command(command: String, args: Vec<String>) -> Result<Child> {
    println!("spawning command {:?} args {:?}", command, args);
    let child = match Command::new(command).args(args).spawn() {
        Ok(output) => output,
        Err(e) => {
            println!("failed to execute command: {}", e);
            return Err(e.into());
        }
    };

    Ok(child)
}

impl Render for Launcher {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let top_levels = window.foreign_toplevels();
        if top_levels.len() != self.top_levels.len() {
            self.top_levels = top_levels;
            cx.notify();
        }

        div().size_full()
    }
}

pub fn run_app(cx: &mut gpui::App) {
    let settings = Settings::global(cx).launcher.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
    } = settings.layer_shell.clone();

    let window_bounds = Some(WindowBounds::Windowed(Bounds::centered(
        None,
        Size {
            width: size.width,
            height: size.height,
        },
        cx,
    )));

    let window = cx.open_window(
        WindowOptions {
            window_bounds,
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(layer_shell::LayerShellOptions {
                namespace,
                layer,
                anchor,
                exclusive_zone: Some(exclusive_zone),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let regions = Vec::new();
            window.set_input_regions(Some(regions));
            cx.new(|_cx| Launcher::new())
        },
    );
    let entity = window.unwrap().entity(cx).unwrap();
    listen_dispatcher(cx, entity);
}

fn listen_dispatcher(cx: &mut gpui::App, entity: Entity<Launcher>) {
    let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();
    _ = cx
        .spawn(async move |cx| {
            while let Ok(msg) = dispatcher_rx.recv().await {
                match msg {
                    dispatcher::Message::LaunchApp { app_id, exec } => {
                        _ = entity.update(cx, |this, cx| {
                            this.launch_app(app_id, exec);
                            cx.notify();
                        });
                    }

                    _ => (),
                }
            }
        })
        .detach();
}

pub fn run() {
    Application::new().with_assets(Assets {}).run(|cx| {
        //init global settings
        settings::init(cx);

        //init dispatcher
        dispatcher::init(cx);

        //init global theme
        theme::init(cx);

        //init shell state
        shell_state::init(cx);

        //run launcher window
        crate::run_app(cx);

        let installed_apps = cx.new(|cx| InstalledApps::new(cx));

        status_bar::run_app(cx);

        homescreen::run_app(cx);

        running_apps::run_app(installed_apps, cx);

        settings_drawer::run_app(cx);

        notifications::run_app(cx);

        volume_slider::run_app(cx);

        cx.activate(true);
        cx.refresh_windows();
    });
}
