use crate::models::models::RunningApps;
use commons::prelude::InstalledApps;
use gpui::{
    layer_shell::{KeyboardInteractivity, LayerShellOptions},
    *,
};

use settings::prelude::{InputRegions, LayerShellSettings, RunningAppsSettings, Settings};

pub mod config;
pub mod models;
mod ui;
pub mod prelude {
    pub use crate::config::*;
    pub use crate::models::*;
}

fn window_options(cx: &mut App) -> WindowOptions {
    let LayerShellSettings {
        size,
        namespace,
        layer,
        anchor,
        exclusive_zone,
        ..
    } = cx.global::<Settings>().running_apps.layer_shell.clone();

    let window_bounds = WindowBounds::Windowed(Bounds::new(point(px(0.), px(0.)), size));
    WindowOptions {
        window_bounds: Some(window_bounds),
        window_background: WindowBackgroundAppearance::Transparent,
        kind: WindowKind::LayerShell(LayerShellOptions {
            namespace,
            layer,
            anchor,
            keyboard_interactivity: KeyboardInteractivity::None,
            exclusive_zone: Some(exclusive_zone),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn root_view(
    installed_apps: Entity<InstalledApps>,
) -> impl FnOnce(&mut Window, &mut App) -> Entity<RunningApps> {
    |window: &mut Window, cx: &mut App| {
        let settings = Settings::global(cx).running_apps.clone();

        let RunningAppsSettings {
            navbar_size,
            layer_shell,
            input_regions,
            ..
        } = settings;

        let InputRegions { minimized, .. } = input_regions;

        let LayerShellSettings {
            size: window_size, ..
        } = layer_shell;

        let mut regions = Vec::new();

        regions.push(Bounds {
            origin: minimized.origin,
            size: minimized.size,
        });

        window.set_input_regions(Some(regions));

        let running_apps = cx.new(|cx| RunningApps::new(installed_apps));

        running_apps
    }
}

pub fn run_app(installed_apps: Entity<InstalledApps>, app: &mut App) -> EntityId {
    let window_options = window_options(app);

    let window = app
        .open_window(window_options, root_view(installed_apps))
        .unwrap();
    let entity_id = window.entity(app).unwrap().entity_id();
    entity_id
}
