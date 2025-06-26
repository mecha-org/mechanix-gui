use bevy::{color::palettes::css::DARK_GRAY, prelude::*, winit::WinitSettings};
use bevy_asset_loader::prelude::*;
use bevy_styled_widgets::prelude::ThemeManager;
use systems::control_click_system;
mod systems;

use crate::{
    StyledWidgetsPlugin,
    settings::settings_drawer::SettingsDrawerSettings,
    utils::{FontAssets, Icon},
    widgets::button::{ButtonSize, ButtonVariant, StyledButton},
};

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Component, Debug, Clone)]
pub struct ControlName(pub String);

#[derive(Default, Debug, Clone)]
pub struct GuiSettings {
    pub settings_drawer: SettingsDrawerSettings,
}

pub fn run_settings_drawer() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPlugin))
        .insert_resource(ThemeManager::default())
        .insert_resource(WinitSettings::desktop_app())
        .init_state::<AssetsLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<FontAssets>(),
        )
        .add_systems(OnEnter(AssetsLoadingState::Loaded), setup_view_root)
        .run();
}
fn setup_view_root(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn(Camera2d);

    let GuiSettings { settings_drawer } = GuiSettings::default();
    let SettingsDrawerSettings {
        width,
        height,
        menus,
    } = settings_drawer;

    let current_menu = "sm"; //

    let list_menus = menus.get(current_menu).unwrap_or(&vec![]).clone();

    // Create a root node
    commands
        .spawn((
            Node {
                width: Val::Vw(width),
                height: Val::Vh(height),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                padding: UiRect::all(Val::Px(28.0)),
                row_gap: Val::Px(14.0),
                column_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(DARK_GRAY.into()),
        ))
        .with_children(
            |parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                for control_name in list_menus.clone() {
                    spawn_menu_widget(parent, &font_assets, control_name.as_str());
                }
            },
        );
}

fn spawn_menu_widget(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    font_assets: &FontAssets,
    control_name: &str,
) {
    let grid_column = if control_name == "brightness" || control_name == "volume" {
        GridPlacement::span(4)
    } else {
        GridPlacement::span(1)
    };
    let FontAssets { font_icons, .. } = font_assets;

    let click_system_id = parent
        .commands()
        .register_system(control_click_system(control_name.to_string()));

    let icon = match control_name {
        "airplane_mode" => Icon::AirplaneMode,
        "auto_rotation" => Icon::AutoRotation,
        "external_display" => Icon::Monitor,
        "screen_record" => Icon::ScreenRecord,
        "wifi" => Icon::WifiConnectedStrong,
        "bluetooth" => Icon::Bluetooth,
        "camera" => Icon::Camera,
        "battery" => Icon::Battery,
        "terminal" => Icon::Terminal,
        "voice_record" => Icon::Mic,
        "calc" => Icon::Calc,
        "theme" => Icon::Moon,
        "brightness" => Icon::Brightness,
        "volume" => Icon::VolumeOn,
        _ => Icon::Moon,
    };

    parent.spawn((
        Node {
            display: Display::Grid,
            grid_column: grid_column,
            padding: UiRect::all(Val::Px(4.0)),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..Default::default()
        },
        Children::spawn(Spawn((
            StyledButton::builder()
                .icon(icon)
                .font(font_icons.clone())
                .variant(ButtonVariant::Primary)
                .border_radius(20.)
                .on_click(click_system_id)
                .build(),
            ControlName(control_name.to_string()),
        ))),
    ));
}
