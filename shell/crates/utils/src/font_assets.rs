use bevy::prelude::*;
use bevy::state::state::States;
use bevy_asset_loader::prelude::*;
use types::prelude::*;

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum FontAssetsLoadingState {
    #[default]
    Loading,
    Loaded,
    Failed,
}

pub struct FontAssetsPlugin;

impl bevy::prelude::Plugin for FontAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<FontAssetsLoadingState>();
        app.add_loading_state(
            LoadingState::new(FontAssetsLoadingState::Loading)
                .continue_to_state(FontAssetsLoadingState::Loaded)
                .on_failure_continue_to_state(FontAssetsLoadingState::Failed)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("fonts.ron")
                .load_collection::<FontAssets>(),
        );
    }
}

pub fn fonts_loaded(load_state: Res<State<FontAssetsLoadingState>>) -> bool {
    *load_state == FontAssetsLoadingState::Loaded
}
