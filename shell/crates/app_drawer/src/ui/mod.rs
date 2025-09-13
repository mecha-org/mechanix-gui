mod app_button;
pub mod apps_list;
pub mod categories;
mod category_expanded;
pub mod search_input;

use std::collections::HashMap;

use app_button::*;
use bevy::{ecs::spawn::SpawnWith, prelude::*};
use types::prelude::FontAssets;
use utils::prelude::{DesktopApp, DesktopApps, fonts_loaded};

use crate::{
    icons::AppDrawerIcons,
    setup::AppDrawerWindowCamera,
    ui::{apps_list::apps_list, categories::categories_list, search_input::search_input},
};

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy, States)]
pub enum AppDrawerState {
    #[default]
    AssetsLoading,
    Categories,
    AppList,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppDrawerState = AppDrawerState::Categories)]
pub enum CategoriesSubState {
    #[default]
    NoPopup,
    Popup,
}

pub fn init_state(mut commands: Commands, mut done: Local<bool>) {
    if *done {
        return;
    }

    commands.set_state(AppDrawerState::Categories);

    *done = true;
}

#[derive(Component)]
pub struct AppDrawerRoot;

pub fn ui(
    mut commands: &Commands,
    font_assets: &FontAssets,
    icons: &AppDrawerIcons,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        AppDrawerRoot,
        children![search_input(font_assets, icons),],
    )
}
