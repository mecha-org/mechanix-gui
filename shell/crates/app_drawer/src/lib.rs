use bevy::prelude::*;

mod button_system;
mod icons;
mod setup;
mod ui;

use types::prelude::FontAssets;
use utils::prelude::{DesktopAppsPlugin, FontAssetsPlugin, fonts_loaded};

use crate::{
    icons::{AppDrawerIcons, AppDrawerIconsPlugin, icons_loaded},
    setup::{AppDrawerWindowCamera, camera_setup, exit_on_esc},
    ui::{
        AppDrawerState, CategoriesSubState,
        apps_list::{filter_apps, spawn_apps_list},
        categories::{
            despawn_apps_category_popup, spawn_apps_category_popup, spawn_categories_list,
        },
        init_state,
        search_input::{SearchActive, SearchInputPlugin, SearchText},
    },
};

#[derive(Event)]
pub struct AppDrawerOpen;

#[derive(Event)]
pub struct AppDrawerClose;

pub struct AppDrawerPlugin;
impl Plugin for AppDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FontAssetsPlugin);
        app.add_plugins(AppDrawerIconsPlugin);
        app.add_plugins((animation::DefaultTweenPlugins,));
        app.add_plugins((headless_widgets::CoreWidgetsPlugin));
        app.add_plugins((DesktopAppsPlugin));
        app.add_plugins(SearchInputPlugin);

        app.insert_state(AppDrawerState::default());
        app.add_sub_state::<CategoriesSubState>();
        app.add_systems(Startup, camera_setup);
        app.add_systems(
            Update,
            setup::setup
                .run_if(resource_exists::<AppDrawerWindowCamera>)
                .run_if(fonts_loaded)
                .run_if(icons_loaded),
        );
        app.add_systems(Update, init_state.run_if(fonts_loaded).run_if(icons_loaded));
        app.add_systems(OnEnter(CategoriesSubState::NoPopup), spawn_categories_list);
        app.add_systems(
            OnEnter(CategoriesSubState::Popup),
            spawn_apps_category_popup,
        );
        app.add_systems(
            OnExit(CategoriesSubState::Popup),
            despawn_apps_category_popup,
        );
        app.add_systems(OnEnter(AppDrawerState::AppList), spawn_apps_list);

        app.add_systems(Update, (button_system::button_system, exit_on_esc));

        app.add_systems(
            Update,
            filter_apps.run_if(resource_exists_and_changed::<SearchText>),
        );
    }
}

pub mod prelude {
    pub use crate::AppDrawerPlugin;
}
