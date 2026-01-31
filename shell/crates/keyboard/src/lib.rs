use gpui::{layer_shell::KeyboardInteractivity, *};

mod builder;
mod config;
mod keymap;
mod layout;
mod prediction;
mod trie;
mod types;
mod ui;
mod utils;

use icons::prelude::*;
use settings::prelude::*;
use shell_state::ShellState;
use trie::util::get_trie;

use crate::{config::Layout, ui::OnScreenKeyboard};

pub mod prelude {
    pub use crate::run_app;
}

pub fn run_app(cx: &mut App) {
    if !cx.has_global::<Settings>() {
        theme::init(cx);
    }
    if !cx.has_global::<Settings>() {
        settings::init(cx);
    }

    if !cx.has_global::<Settings>() {
        settings::init(cx);
    }

    if !cx.has_global::<Icons>() {
        icons::init(cx);
    }

    let settings = Settings::global(cx).keyboard.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
    } = settings.layer_shell.clone();
    let InputRegions {
        minimized,
        maximized,
    } = settings.input_regions.clone();
    let window_bounds = WindowBounds::Windowed(Bounds::centered(
        None,
        Size {
            width: px(1.),
            height: px(1.),
        },
        cx,
    ));
    let default_layout = settings.default_layout;

    let layout = Layout::from_file(default_layout).unwrap();
    let parsed_layout = layout.build(size.width.to_f64()).unwrap();
    let trie = get_trie();

    let _window = cx
        .open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                window_background: WindowBackgroundAppearance::Transparent,
                is_movable: false,
                is_resizable: false,
                kind: WindowKind::LayerShell(layer_shell::LayerShellOptions {
                    namespace,
                    layer,
                    anchor,
                    exclusive_zone: Some(exclusive_zone),
                    keyboard_interactivity: KeyboardInteractivity::None,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let mut regions = Vec::new();

                regions.push(Bounds {
                    origin: minimized.origin,
                    size: minimized.size,
                });
                window.set_input_regions(Some(regions));

                cx.new(|cx| {
                    cx.observe_global::<ShellState>(|this: &mut OnScreenKeyboard, cx| {})
                        .detach();

                    OnScreenKeyboard::new(parsed_layout, trie, cx)
                })
            },
        )
        .unwrap();
}
