use bevy::{prelude::*, window::ExitCondition, winit::WinitPlugin};
use bevy_wayland::prelude::*;
use notifications_drawer::components::cards::NotificationDrawerPlugin;
use service_plugins::notification::{ NotificationPlugin };

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
            NotificationPlugin,
            NotificationDrawerPlugin,
            WaylandPlugin,
        ))
        .run();
}
