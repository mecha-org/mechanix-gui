use std::result;

use bevy::{
    prelude::*,
    window::{CompositeAlphaMode, WindowResolution},
};
use bevy_wayland::prelude::*;
use types::prelude::FontAssets;

use crate::{
    components::BAR_SIZE,
    icons::UniversalSearchIcons,
    ui::{BrowserApps, FrequentlyUsedApps, SearchItems, SearchResults, SearchText, ui},
};

pub const WINDOW_SIZE: (f32, f32) = (540., 576.);

#[derive(Component)]
pub struct UniversalSearchWindow;

#[derive(Resource)]
pub struct UniversalSearchWindowCamera(pub Entity);

pub fn camera_setup(mut commands: Commands) {
    let exclusive_zone = if cfg!(feature = "standalone") {
        BAR_SIZE.1 as i32
    } else {
        -1
    };

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
                exclusive_zone,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                ..default()
            },
            UniversalSearchWindow,
            // InputRegion(Rect::new(
            //     0.,
            //     WINDOW_SIZE.1 - BAR_SIZE.1,
            //     BAR_SIZE.0,
            //     WINDOW_SIZE.1,
            // )),
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

    commands.insert_resource(UniversalSearchWindowCamera(camera_ent));
}

pub fn setup(
    mut commands: Commands,
    mut has_run: Local<bool>,
    asset_server: Res<AssetServer>,
    f_apps: Res<FrequentlyUsedApps>,
    searches: Res<SearchItems>,
    s_results: Res<SearchResults>,
    s_text: Res<SearchText>,
    browser_apps: Res<BrowserApps>,
    camera_ent: Res<UniversalSearchWindowCamera>,
    font_assets: Res<FontAssets>,
    icons: Res<UniversalSearchIcons>,
) {
    if *has_run {
        return;
    }

    commands.spawn((
        UiTargetCamera(camera_ent.0),
        ui(
            &commands,
            &asset_server,
            f_apps.0.clone(),
            searches.0.clone(),
            s_results.0.clone(),
            s_text.0.clone(),
            browser_apps.0.clone(),
            &font_assets,
            &icons,
        ),
    ));

    *has_run = true;
}

pub fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
