mod ui;

pub use ui::{listen_for_extensions, Toast};

use gpui::layer_shell::LayerShellOptions;
use gpui::*;
use settings::prelude::Settings;

pub mod prelude {
    pub use crate::{listen_for_extensions, run_app, Toast};
}

/// Open the toast window and start listening for dispatcher events.
pub fn run_app(cx: &mut App) {
    let settings = Settings::global(cx).toast.clone();

    let layer_shell = settings.layer_shell;
    let input_region = settings.input_regions.maximized;

    let window = cx.open_window(
        WindowOptions {
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace: layer_shell.namespace,
                layer: layer_shell.layer,
                anchor: layer_shell.anchor,
                exclusive_zone: Some(layer_shell.exclusive_zone),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            // Set input region based on toast visibility
            window.set_input_regions(Some(vec![gpui::Bounds {
                origin: point(input_region.origin.x, input_region.origin.y),
                size: gpui::Size {
                    width: input_region.size.width,
                    height: input_region.size.height,
                },
            }]));
            cx.new(|cx| Toast::new(cx))
        },
    );

    if let Ok(window) = window {
        if let Ok(entity) = window.entity(cx) {
            listen_for_extensions(cx, entity);
        }
    }
}
