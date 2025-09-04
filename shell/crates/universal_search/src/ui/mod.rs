mod frequently_used_apps;
mod search_input;
mod search_items;
mod search_results;

pub use crate::ui::{frequently_used_apps::*, search_input::*, search_items::*, search_results::*};
use crate::{
    icons::UniversalSearchIcons,
    types::{DesktopApp, SearchResult},
};

use bevy::prelude::*;
use types::prelude::*;

#[derive(Component)]
pub struct ParentContainer;

pub fn ui(
    mut commands: &Commands,
    asset_server: &AssetServer,
    apps: Vec<DesktopApp>,
    searches: Vec<SearchResult>,
    results: Vec<SearchResult>,
    results_for: String,
    browser_apps: Vec<DesktopApp>,
    font_assets: &FontAssets,
    icons: &UniversalSearchIcons,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            // justify_content: JustifyContent::FlexEnd,
            // align_items: AlignItems::FlexStart,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ParentContainer,
        children![(
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::horizontal(Val::Px(16.)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.),
                ..Default::default()
            },
            children![
                frequently_used_apps(apps),
                search_items(searches, font_assets, icons),
                search_input(font_assets, icons) // search_results(results, results_for, browser_apps, asset_server)
            ]
        )],
        // children![bar()],
    )
}
