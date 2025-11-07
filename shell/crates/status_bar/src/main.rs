use commons::prelude::*;
use gpui::*;
use status_bar::prelude::*;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(540.0), px(36.0)), cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| StatusBar::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}
