use bevy::{
    prelude::*,
    window::{CompositeAlphaMode, WindowResolution},
};
use bevy_wayland::prelude::*;
use types::prelude::FontAssets;

use crate::{icons::SettingsDrawerIcons, ui::ui};

pub const WINDOW_SIZE: (f32, f32) = (540., 584.);

#[derive(Component)]
pub struct SettingsDrawerWindow;

#[derive(Resource)]
pub struct SettingsDrawerWindowCamera(pub Entity);

pub fn camera_setup(mut commands: Commands) {
    // ui camera
    let window_ent = commands
        .spawn((
            Window {
                resolution: WindowResolution::new(WINDOW_SIZE.0, WINDOW_SIZE.1),
                transparent: true,
                composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                ..default()
            },
            LayerShellSettings {
                anchor: Anchor::BOTTOM,
                layer: Layer::Top,
                exclusive_zone: -1,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..default()
            },
            SettingsDrawerWindow,
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

    commands.insert_resource(SettingsDrawerWindowCamera(camera_ent));
}

pub fn setup(
    mut commands: Commands,
    mut has_run: Local<bool>,
    camera_ent: Res<SettingsDrawerWindowCamera>,
    font_assets: Res<FontAssets>,
    icons: Res<SettingsDrawerIcons>,
) {
    if *has_run {
        return;
    }

    let ui = ui(&mut commands, &font_assets, &icons);

    commands.spawn((UiTargetCamera(camera_ent.0), ui));

    *has_run = true;
}

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
