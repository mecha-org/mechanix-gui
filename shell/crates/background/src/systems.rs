use bevy::{prelude::*, window::WindowResolution};
use bevy_wayland::prelude::*;

use crate::ui::ui;

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let width = 540.;
    let height = 620.;

    // ui camera
    let window_ent = commands
        .spawn((
            Window {
                resolution: WindowResolution::new(width, height),
                ..default()
            },
            LayerShellSettings {
                anchor: Anchor::TOP,
                layer: Layer::Background,
                exclusive_zone: -1,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..default()
            },
            InputRegion(Rect::new(0., 0., 0., 0.)),
        ))
        .id();
    let camera_ent = commands
        .spawn((
            Camera2d,
            Camera {
                target: bevy::render::camera::RenderTarget::Window(
                    bevy::window::WindowRef::Entity(window_ent),
                ),
                clear_color: ClearColorConfig::Custom(Color::NONE),
                ..default()
            },
        ))
        .id();
    commands.spawn((UiTargetCamera(camera_ent), ui(&commands, &asset_server)));
}
