use gpui::{
    layer_shell::{KeyboardInteractivity, LayerShellOptions},
    *,
};
use settings::prelude::{LayerShellSettings, PowerOptionsSettings, Settings};

mod ui;

use crate::ui::*;

pub mod prelude {
    pub use crate::run_app;
}

pub fn run_app(cx: &mut App) {
    let PowerOptionsSettings { layer_shell } = Settings::global(cx).power_options.clone();
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
                origin: point(px(0.0), px(0.0)),
                size: gpui::size(size.width, size.height),
            });
            window.set_input_regions(Some(regions));

            cx.new(|cx| PowerOptions::new(cx))
        },
    )
    .unwrap();
}
