use std::time::Duration;

use commons::prelude::*;
use gpui::{layer_shell::*, *};
use running_apps::prelude::*;
use settings_drawer::prelude::*;
use status_bar::prelude::*;
use types::{GlobalState, load_global_state};
use universal_search::prelude::*;

use crate::{
    settings::{GlobalSettings, Settings, load_settings},
    ui::{
        // models::{GlobalState, load_global_state},
        theme::load_theme,
    },
};

pub fn run() {
    Application::new().with_assets(Assets {}).run(|cx| {
        if let Err(e) = load_theme(cx) {
            println!("error loading theme: {}", e);
        };

        if let Err(e) = load_settings(cx) {
            println!("error loading settings: {}", e);
        };

        load_global_state(cx);

        let settings = cx.global::<GlobalSettings>().settings.read(cx);
        let Settings { window_size, .. } = settings.clone();
        let state = cx.global::<GlobalState>();
        let wireless_enabled = state.wireless_enabled.clone();

        cx.spawn({
            let wireless_enabled = wireless_enabled.clone();
            async move |cx| {
                loop {
                    cx.background_executor().timer(Duration::from_secs(5)).await;
                    let _ = wireless_enabled.write(cx, true);
                }
            }
        })
        .detach();

        let window_bounds = WindowBounds::Windowed(Bounds::new(
            point(px(0.), px(0.)),
            size(px(window_size.0), px(36.)),
        ));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "mechanix.statusbar".to_string(),
                    layer: Layer::Top,
                    anchor: Anchor::TOP,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    margin: None,
                    exclusive_zone: Some(px(36.)),
                    // exclusive_edge: Some(Anchor::TOP),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                window.set_input_regions(Some(Vec::new()));
                let status_bar = cx.new(|_cx| StatusBar::new());
                status_bar
            },
        )
        .unwrap();

        const NAVBAR_SIZE: (f32, f32) = (180., 29.);

        let window_bounds = WindowBounds::Windowed(Bounds::new(
            point(px(0.), px(0.)),
            size(px(window_size.0), px(window_size.1)),
        ));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "mechanix.settings.drawer".to_string(),
                    layer: Layer::Top,
                    anchor: Anchor::BOTTOM,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    exclusive_zone: Some(px(-1.)),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let mut regions = Vec::new();
                regions.push(Bounds {
                    origin: point(
                        px(window_size.0 - NAVBAR_SIZE.0),
                        px(window_size.1 - NAVBAR_SIZE.1),
                    ),
                    size: size(px(NAVBAR_SIZE.0), px(NAVBAR_SIZE.1)),
                });
                window.set_input_regions(Some(regions));
                let settings_drawer = cx.new(|cx| SettingsDrawer::new(cx));
                settings_drawer
            },
        )
        .unwrap();

        let window_bounds = WindowBounds::Windowed(Bounds::new(
            point(px(0.), px(0.)),
            size(px(window_size.0), px(window_size.1)),
        ));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "mechanix.universal.search".to_string(),
                    layer: Layer::Top,
                    anchor: Anchor::BOTTOM,
                    keyboard_interactivity: KeyboardInteractivity::OnDemand,
                    exclusive_zone: Some(px(-1.)),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let mut regions = Vec::new();
                regions.push(Bounds {
                    origin: point(px(0.), px(window_size.1 - NAVBAR_SIZE.1)),
                    size: size(px(NAVBAR_SIZE.0), px(NAVBAR_SIZE.1)),
                });
                window.set_input_regions(Some(regions));
                let universal_search = cx.new(|cx| UniversalSearch::new(cx));
                universal_search
            },
        )
        .unwrap();

        let window_bounds = WindowBounds::Windowed(Bounds::new(
            point(px(0.), px(0.)),
            size(px(window_size.0), px(window_size.1)),
        ));

        const RUNNING_APPS_NAVBAR_SIZE: (f32, f32) = (180., 29.);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(LayerShellOptions {
                    namespace: "mechanix.running.apps".to_string(),
                    layer: Layer::Top,
                    anchor: Anchor::BOTTOM,
                    keyboard_interactivity: KeyboardInteractivity::None,
                    exclusive_zone: Some(px(-1.)),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let mut regions = Vec::new();
                regions.push(Bounds {
                    origin: point(
                        px((window_size.0 - RUNNING_APPS_NAVBAR_SIZE.0) / 2.),
                        px(window_size.1 - RUNNING_APPS_NAVBAR_SIZE.1),
                    ),
                    size: size(
                        px(RUNNING_APPS_NAVBAR_SIZE.0),
                        px(RUNNING_APPS_NAVBAR_SIZE.1),
                    ),
                });
                window.set_input_regions(Some(regions));
                let running_apps = cx.new(|cx| RunningApps::new());
                running_apps
            },
        )
        .unwrap();

        cx.activate(true);
        cx.refresh_windows();
    });
}
