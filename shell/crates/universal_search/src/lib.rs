mod data;
mod ui;

use commons::input::*;
use gpui::layer_shell::{KeyboardInteractivity, LayerShellOptions};
use gpui::*;
use settings::prelude::*;
use ui::models::UniversalSearch;

pub mod prelude {
    pub use crate::data::*;
    // pub use crate::run_app;
    pub use crate::ui::models::UniversalSearch;
}

pub fn run_app(cx: &mut App) {
    let UniversalSearchSettings {
        layer_shell,
        navbar_size,
        ..
    } = Settings::global(cx).universal_search.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
        ..
    } = layer_shell;

    let screen_size = gpui::size(size.width, size.height - navbar_size.height);
    let window_bounds = WindowBounds::Windowed(Bounds::centered(None, screen_size, cx));

    cx.open_window(
        WindowOptions {
            window_bounds: Some(window_bounds),
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace,
                layer,
                anchor,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                exclusive_zone: Some(exclusive_zone),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let mut regions = Vec::new();
            regions.push(Bounds {
                origin: point(px(0.), size.height - navbar_size.height),
                size: gpui::size(navbar_size.width, navbar_size.height),
            });
            window.set_input_regions(Some(regions));
            cx.new(|cx| UniversalSearch::new(cx))
        }, // Pass cx here
    )
    .unwrap();
}
