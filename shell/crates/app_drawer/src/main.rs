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
