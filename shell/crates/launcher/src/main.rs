use bevy::{asset::AssetMetaCheck, prelude::*, window::ExitCondition, winit::WinitPlugin};
use bevy_wayland::prelude::*;
use mechanix_launcher::LauncherPlugin;

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
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    unapproved_path_mode: bevy::asset::UnapprovedPathMode::Allow,
                    ..Default::default()
                }),
            WaylandPlugin,
            LauncherPlugin,
        ))
        .run();
}
