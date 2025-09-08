use bevy::{prelude::*, window::ExitCondition, winit::WinitPlugin};
use bevy_wayland::prelude::*;
use headless_widgets::CoreWidgetsPlugin;
use settings_drawer::{Screens, SettingsDrawerPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .disable::<WinitPlugin>()
                .set(WindowPlugin {
                    primary_window: None,
                    exit_condition: ExitCondition::DontExit,
                    ..Default::default()
                }),
            WaylandPlugin,
            SettingsDrawerPlugin,
            CoreWidgetsPlugin
        ))
        .insert_state(Screens::Loading)
        .run();
}
