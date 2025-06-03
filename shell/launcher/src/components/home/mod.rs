use bevy::{
    color::palettes::css::{LIGHT_GREY}, prelude::*, winit::WinitSettings
};
use bevy_asset_loader::prelude::*;
use bevy_styled_widgets::prelude::ThemeManager;

use crate::{
    StyledWidgetsPlugin,
    settings::home::HomeScreenSettings,
    utils::{FontAssets, Icon},
    widgets::app_bundle::{ButtonVariant, StyledAppBundle},
};
mod styles;
use styles::*;

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}

/// Loads image assets
#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(key = "images.file.manager.app")]
    file_manager: Handle<Image>,

    #[asset(key = "images.app.icon")]
    app_icon: Handle<Image>,
}

#[derive(Component, Debug, Clone)]
pub struct ControlName(pub String);

#[derive(Default, Debug, Clone)]
pub struct GuiSettings {
    pub home: HomeScreenSettings,
}

pub fn run_home() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPlugin))
        .insert_resource(ThemeManager::default())
        .insert_resource(WinitSettings::desktop_app())
        .init_state::<AssetsLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>(),
        )
        .add_systems(OnEnter(AssetsLoadingState::Loaded), setup_view_root)
        .run();
}
fn setup_view_root(mut commands: Commands,  image_assets: Res<ImageAssets>, font_assets: Res<FontAssets>) {
    commands.spawn(Camera2d);

    let GuiSettings { home } = GuiSettings::default();
    let HomeScreenSettings {
        width,
        height,
        pinned_apps,
        widgets,
    } = home;

    let current_menu = "sm"; //

    let pinned_apps = pinned_apps.get(current_menu).unwrap_or(&vec![]).clone();
    let widgets = widgets.get(current_menu).unwrap_or(&vec![]).clone();

    // Create a root node
    commands
        .spawn((
            Node {
                width: Val::Vw(width),
                height: Val::Vh(height),
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                padding: ROOT_PADDING,
                row_gap: ROW_GAP,
                column_gap: COLUMN_GAP,
                ..default()
            },
        ))
        .with_children(
            |parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                for control_name in pinned_apps.clone() {
                    spawn_menu_widget(parent, &image_assets, &font_assets, control_name.as_str());
                }
            },
        );
}

fn spawn_menu_widget(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    image_assets: &ImageAssets,
    font_assets: &FontAssets,
    control_name: &str,
) {
    let FontAssets { font_icons, .. } = font_assets;

    let icon = match control_name {
        "App 1" => Icon::Terminal,
        "App 2" => Icon::Folder,
        "App 3" => Icon::FileManager,
        "App 4" => Icon::App,
        "App 5" => Icon::App,
        "App 6" => Icon::App,
        "App 7" => Icon::App,
        "App 8" => Icon::App,
        "App 9" => Icon::App,
        "App 10" => Icon::App,
        "App 11" => Icon::App,
        "App 12" => Icon::App,
        "App 13" => Icon::App,
        "App 14" => Icon::Moon,
        "App 15" => Icon::Moon,
        "App 16" => Icon::Moon,
        _ => Icon::Moon,
    };

    

   let (grid_column, grid_row) = if control_name == "App 23"  {
        (GridPlacement::span(2), GridPlacement::span(2))
    } else {
        (GridPlacement::span(1), GridPlacement::span(1))
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
            StyledAppBundle::builder()
                .image(image_assets.app_icon.clone())
                .text(control_name.to_string())
                .text_color(LIGHT_GREY.into())
                .variant(ButtonVariant::Primary)
                .border_radius(20.)
                .width(ICON_WIDTH)
                .height(ICON_HEIGHT)
                .build(),
            ControlName(control_name.to_string()),
        ))),
    ));
}
