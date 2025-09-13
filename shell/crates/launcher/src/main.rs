use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{ExitCondition, PrimaryWindow, WindowResolution},
    winit::WinitPlugin,
};
use bevy_wayland::prelude::*;
use mechanix_launcher::LauncherPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .disable::<WinitPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1., 1.),
                        transparent: true,
                        ..Default::default()
                    }),
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
        .add_systems(PreStartup, pre_startup)
        .run();
}

fn pre_startup(mut commands: Commands, primary_window: Single<Entity, With<PrimaryWindow>>) {
    commands
        .entity(*primary_window)
        .insert((LayerShellSettings {
            anchor: Anchor::empty(),
            layer: Layer::Bottom,
            exclusive_zone: 0,
            keyboard_interactivity: KeyboardInteractivity::None,
            ..default()
        },));
    commands.spawn((
        Camera2d,
        MeshPickingCamera,
        Camera {
            target: bevy::render::camera::RenderTarget::Window(bevy::window::WindowRef::Entity(
                *primary_window,
            )),
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
    ));
}
