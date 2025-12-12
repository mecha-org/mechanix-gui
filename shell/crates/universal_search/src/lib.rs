mod data;
mod ui;

use gpui::layer_shell::{KeyboardInteractivity, LayerShellOptions};
use gpui::*;
use settings::prelude::*;
use ui::input::*;
use ui::models::UniversalSearch;

pub mod prelude {
    pub use crate::data::*;
    pub use crate::run_app;
    pub use crate::ui::input::*;
    pub use crate::ui::models::TextInput;
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

    // Register key bindings for the text input
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, None),
        KeyBinding::new("delete", Delete, None),
        KeyBinding::new("left", Left, None),
        KeyBinding::new("right", Right, None),
        KeyBinding::new("shift-left", SelectLeft, None),
        KeyBinding::new("shift-right", SelectRight, None),
        KeyBinding::new("cmd-a", SelectAll, None),
        KeyBinding::new("ctrl-a", SelectAll, None), // Add Windows/Linux alternative
        KeyBinding::new("home", Home, None),
        KeyBinding::new("end", End, None),
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
        KeyBinding::new("cmd-v", Paste, None),
        KeyBinding::new("ctrl-v", Paste, None), // Add Windows/Linux alternative
        KeyBinding::new("cmd-c", Copy, None),
        KeyBinding::new("ctrl-c", Copy, None), // Add Windows/Linux alternative
        KeyBinding::new("cmd-x", Cut, None),
        KeyBinding::new("ctrl-x", Cut, None), // Add Windows/Linux alternative
    ]);

    let window_bounds = WindowBounds::Windowed(Bounds::centered(None, size, cx));

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
