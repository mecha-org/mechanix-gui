pub mod helper;
mod ui;

use dispatcher::Dispatcher;
use gpui::{
    layer_shell::{KeyboardInteractivity, LayerShellOptions},
    *,
};
use settings::prelude::*;
use shell_state::ShellState;
use ui::*;

pub mod prelude {
    pub use crate::run_app;
    pub use crate::ui::SettingsDrawer;
}

pub fn run_app(cx: &mut App) {
    let SettingsDrawerSettings {
        layer_shell,
        input_regions,
        ..
    } = Settings::global(cx).settings_drawer.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
    } = layer_shell;
    let window_bounds = WindowBounds::Windowed(Bounds::centered(None, size, cx));
    cx.open_window(
        WindowOptions {
            window_bounds: Some(window_bounds),
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace,
                layer,
                anchor,
                keyboard_interactivity: KeyboardInteractivity::None,
                exclusive_zone: Some(exclusive_zone),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let mut regions = Vec::new();
            regions.push(Bounds {
                origin: input_regions.minimized.origin,
                size: input_regions.minimized.size,
            });
            window.set_input_regions(Some(regions));

            cx.new(|cx| {
                cx.observe_global::<ShellState>(|this: &mut SettingsDrawer, cx| {
                })
                .detach();

                listen_dispatcher(cx);

                SettingsDrawer::new(cx)
            })
        },
    )
    .unwrap();
}

pub fn listen_dispatcher(cx: &mut Context<SettingsDrawer>) {
    if !cx.has_global::<Dispatcher>() {
        dispatcher::init(cx);
    }

    let mut dispatcher_rx = Dispatcher::global(cx).channel().1.clone();

    let settings = Settings::global(cx).settings_drawer.clone();
    let closed_pos = SettingsDrawer::calculate_closed_position(&settings);

    cx.spawn(async move |this, cx| {
        while let Ok(message) = dispatcher_rx.recv().await {
            match message {
                dispatcher::Message::ShowPowerOptions(show) => {
                    if show {
                        let _ = this.update(cx, |this, cx| {
                            this.is_visible = false;
                            this.position = closed_pos;
                            cx.notify();
                        });
                    }
                }
                _ => {}
            }
        }
    })
    .detach();
}
