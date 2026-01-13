use app_drawer::prelude::*;
use commons::{assets::Assets, input::*};
use gpui::*;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});

    
    application.run(move |cx| {


        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(540.0), px(620.0)), cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                ..Default::default()
            },
            move |_window, cx| {
                // pass the initial state into the AppDrawer constructor
                cx.new(|_cx| AppDrawer::new( _cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
