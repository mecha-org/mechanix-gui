use bevy::{prelude::*, window::ExitCondition, winit::WinitPlugin};
use bevy_wayland::prelude::*;
use status_bar::StatusBarPlugin;

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
            StatusBarPlugin,
        ))
        .run();
}
