use bevy::{
    prelude::*,
    window::{ExitCondition, PrimaryWindow},
    winit::WinitPlugin,
};
use bevy_wayland::prelude::*;
use homescreen::HomescreenPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .disable::<WinitPlugin>()
                .set(WindowPlugin {
                    exit_condition: ExitCondition::DontExit,
                    ..Default::default()
                }),
            MeshPickingPlugin,
            WaylandPlugin,
            HomescreenPlugin,
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
