use bevy::prelude::*;
use bevy::state::state::States;
use bevy_asset_loader::prelude::*;
use types::prelude::*;

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
    Failed,
}

pub struct AssetsLoadingPlugin {
    settings_path: String,
}

impl Default for AssetsLoadingPlugin {
    fn default() -> Self {
        Self {
            settings_path: "settings.ron".to_string(),
        }
    }
}

impl bevy::prelude::Plugin for AssetsLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetsLoadingState>();
        app.add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .on_failure_continue_to_state(AssetsLoadingState::Failed)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(&self.settings_path)
                .load_collection::<FontAssets>(),
        );
    }
}

pub fn assets_loaded(load_state: Res<State<AssetsLoadingState>>) -> bool {
    *load_state == AssetsLoadingState::Loaded
}
