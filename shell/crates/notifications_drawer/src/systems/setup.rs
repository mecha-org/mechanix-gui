use bevy::{
    prelude::*,
    window::{CompositeAlphaMode, WindowResolution},
};
use bevy_wayland::prelude::*;

use crate::ui::ui;

pub fn setup(mut commands: Commands) {
    let width = 540.;
    let height = 576.;

    // ui camera
    let window_ent = commands
        .spawn((
            Window {
                resolution: WindowResolution::new(width, height),
                transparent: true,
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                ..default()
            },
            LayerShellSettings {
                anchor: Anchor::BOTTOM,
                layer: Layer::Bottom,
                exclusive_zone: 0,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..default()
            },
            InputRegion(Rect::new(0., 0., width, height)),
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
    commands.spawn((UiTargetCamera(camera_ent), ui(&commands)));
}

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
