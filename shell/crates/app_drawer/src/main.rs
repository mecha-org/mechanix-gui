use app_drawer::prelude::*;
use commons::prelude::*;
use gpui::*;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});

    // Initialize the AppDrawerState
    let state = AppDrawerState {
        apps: vec![
            AppInfo {
                id: 1,
                name: "Files".into(),
                category: "Recently used".into(),
                icon_path: IconName::Files,
            },
            AppInfo {
                id: 2,
                name: "Telegram".into(),
                category: "Recently used".into(),
                icon_path: IconName::Telegram,
            },
            AppInfo {
                id: 3,
                name: "Chromium".into(),
                category: "Productivity".into(),
                icon_path: IconName::Chromium,
            },
            AppInfo {
                id: 4,
                name: "Firefox".into(),
                category: "Productivity".into(),
                icon_path: IconName::Firefox,
            },
            AppInfo {
                id: 5,
                name: "Mecha".into(),
                category: "Settings".into(),
                icon_path: IconName::Mecha,
            },
            AppInfo {
                id: 6,
                name: "Firefox".into(),
                category: "Entertainment".into(),
                icon_path: IconName::Firefox,
            },
            AppInfo {
                id: 7,
                name: "Mecha".into(),
                category: "Photography".into(),
                icon_path: IconName::Mecha,
            },
            AppInfo {
                id: 8,
                name: "Chromium".into(),
                category: "Recently used".into(),
                icon_path: IconName::Chromium,
            },
            AppInfo {
                id: 9,
                name: "Firefox".into(),
                category: "Recently used".into(),
                icon_path: IconName::Firefox,
            },
            AppInfo {
                id: 10,
                name: "Mecha".into(),
                category: "Recently used".into(),
                icon_path: IconName::Mecha,
            },
        ],
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
