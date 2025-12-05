use commons::prelude::*;
use gpui::{layer_shell::*, *};
use homescreen::prelude::*;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let config = HomescreenConfig::default();
        let window_bounds = WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(config.window.width), px(config.window.height)),
            cx,
        ));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "mechanix.homescreen".to_string(),
                    layer: Layer::Top,
                    anchor: Anchor::LEFT | Anchor::TOP | Anchor::RIGHT | Anchor::BOTTOM,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    margin: None,
                    exclusive_zone: None,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| Homescreen::new(cx, config)),
        )
        .unwrap();
        cx.activate(true);
    });
}
