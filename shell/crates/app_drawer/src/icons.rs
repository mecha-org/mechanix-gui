use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Clone)]
#[allow(dead_code)]
pub struct AppDrawerIcons {
    #[asset(key = "icons.search")]
    pub search: Handle<Image>,

    #[asset(key = "icons.default_app_icon")]
    pub default_app_icon: Handle<Image>,

    #[asset(key = "icons.close")]
    pub close: Handle<Image>,

    #[asset(key = "icons.sm_curve")]
    pub sm_curve_image: Handle<Image>,

    #[asset(key = "icons.lg_curve")]
    pub lg_curve_image: Handle<Image>,
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AppDrawerIconsState {
    #[default]
    Loading,
    Loaded,
    Failed,
}

pub struct AppDrawerIconsPlugin;

impl bevy::prelude::Plugin for AppDrawerIconsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppDrawerIconsState>();
        app.add_loading_state(
            LoadingState::new(AppDrawerIconsState::Loading)
                .continue_to_state(AppDrawerIconsState::Loaded)
                .on_failure_continue_to_state(AppDrawerIconsState::Failed)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("icons.ron")
                .load_collection::<AppDrawerIcons>(),
        );
    }
}

pub fn icons_loaded(load_state: Res<State<AppDrawerIconsState>>) -> bool {
    *load_state == AppDrawerIconsState::Loaded
}
