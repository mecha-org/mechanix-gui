use app_drawer::prelude::*;
use app_drawer::ui::utils::prelude::DesktopApps;
use commons::prelude::*;
use gpui::*;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    let desktop_apps = DesktopApps::scan();

    let state = AppDrawerState {
        apps: desktop_apps,
        ..Default::default()
    };

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
                cx.new(|_cx| AppDrawer::new(state.clone(), _cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
