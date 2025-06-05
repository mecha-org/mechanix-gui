use bevy::{color::palettes::css::{LIGHT_BLUE, LIGHT_GREY}, prelude::*, winit::WinitSettings};
use bevy_asset_loader::prelude::*;
use bevy_styled_widgets::prelude::ThemeManager;

use crate::{
    StyledWidgetsPlugin,
    settings::home::{HomeBundleType, HomeEntry, HomeScreenSettings},
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

    #[asset(key = "images.widget")]
    widget_icon: Handle<Image>,
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
fn setup_view_root(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
) {
    commands.spawn(Camera2d);

    let GuiSettings { home } = GuiSettings::default();
    let HomeScreenSettings {
        width,
        height,
        grid_template_columns,
        grid_template_rows,
        home_entries,
    } = home;

    let current_menu = "sm"; //
    let home_entries = home_entries.get(current_menu).unwrap_or(&vec![]).clone();

    // Create a root node
    commands
        .spawn((Node {
            width: Val::Vw(width),
            height: Val::Vh(height),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(grid_template_columns, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(grid_template_rows, 1.0),
            padding: ROOT_PADDING,
            row_gap: ROW_GAP,
            column_gap: COLUMN_GAP,
            ..default()
        },))
        .with_children(
            |parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
                for home_entry in home_entries.clone() {
                    spawn_menu_widget(parent, &image_assets, &home_entry);
                }
            },
        );
}

fn spawn_menu_widget(
    parent: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>,
    image_assets: &ImageAssets,
    home_entry: &HomeEntry,
) {
    let HomeEntry {
        name, bundle_type, ..
    } = home_entry;

    // kept for widget to use in future
    let (grid_column, grid_row) = if *bundle_type == HomeBundleType::Widget {
        (GridPlacement::span(2), GridPlacement::span(2))
    } else {
        (GridPlacement::span(1), GridPlacement::span(1))
    };

    let icon = match *bundle_type {
        HomeBundleType::App => image_assets.app_icon.clone(),
        HomeBundleType::Widget => image_assets.widget_icon.clone(),
    };

    parent.spawn((
        Node {
            display: Display::Grid,
            grid_column: grid_column,
            grid_row: grid_row,
            padding: UiRect::all(Val::Px(4.0)),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..Default::default()
        }, 
        Children::spawn(Spawn((
            StyledAppBundle::builder()
                .image(icon) // todo: dynamic image
                .text(name.to_string())
                .text_color(LIGHT_GREY.into())
                .variant(ButtonVariant::Primary)
                .border_radius(20.)
                 .width(ICON_WIDTH)
                 .height(ICON_HEIGHT)
                .build(),
            ControlName(name.to_string()),
        ))),
    ));
}
