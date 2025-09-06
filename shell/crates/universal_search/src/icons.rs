use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource, Clone)]
#[allow(dead_code)]
pub struct UniversalSearchIcons {
    #[asset(key = "icons.search")]
    pub search: Handle<Image>,

    #[asset(key = "icons.arrow_up_right")]
    pub arrow_up_right: Handle<Image>,

    #[asset(key = "icons.default_app_icon")]
    pub default_app_icon: Handle<Image>,

    #[asset(key = "icons.close")]
    pub close: Handle<Image>,

    #[asset(key = "icons.left_nav_bar")]
    pub left_nav_bar: Handle<Image>,
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum UniversalSearchIconsState {
    #[default]
    Loading,
    Loaded,
    Failed,
}

pub struct UniversalSearchIconsPlugin;

impl bevy::prelude::Plugin for UniversalSearchIconsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UniversalSearchIconsState>();
        app.add_loading_state(
            LoadingState::new(UniversalSearchIconsState::Loading)
                .continue_to_state(UniversalSearchIconsState::Loaded)
                .on_failure_continue_to_state(UniversalSearchIconsState::Failed)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("icons.ron")
                .load_collection::<UniversalSearchIcons>(),
        );
    }
}

pub fn icons_loaded(load_state: Res<State<UniversalSearchIconsState>>) -> bool {
    *load_state == UniversalSearchIconsState::Loaded
}
