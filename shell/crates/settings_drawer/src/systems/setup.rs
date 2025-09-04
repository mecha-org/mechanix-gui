use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_wayland::prelude::{Anchor, InputRegion, KeyboardInteractivity, Layer, LayerShellSettings};


#[derive(Resource)]
pub struct SettingsDrawerCamera(pub Entity);

pub fn pre_setup(mut commands: Commands) {
    let width = 540.;
    let height = 531.;

    // ui camera
    let window_ent = commands
        .spawn((
            Window {
                resolution: WindowResolution::new(width, height),
                ..default()
            },
            LayerShellSettings {
                anchor: Anchor::TOP,
                layer: Layer::Top,
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
    commands.insert_resource(SettingsDrawerCamera(camera_ent));
}

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
