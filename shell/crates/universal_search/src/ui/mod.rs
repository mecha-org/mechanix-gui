mod frequently_used_apps;
mod navigation_bar;
mod search_input;
mod search_items;
mod search_results;

pub use crate::ui::{
    frequently_used_apps::*, navigation_bar::*, search_input::*, search_items::*, search_results::*,
};
use crate::{
    icons::UniversalSearchIcons,
    types::{DesktopApp, SearchResult},
};

use bevy::prelude::*;
use types::prelude::*;

#[derive(Component)]
pub struct ParentContainer;

#[derive(Component)]
pub struct Container;

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
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexStart,
            ..Default::default()
        },
        Container,
        children![bar(icons.left_nav_bar.clone())],
        // children![bar()],
    )
}
