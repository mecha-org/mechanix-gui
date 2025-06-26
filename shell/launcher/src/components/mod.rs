// pub mod settings_drawer;
pub mod status_bar;

pub use status_bar::*;
// pub use settings_drawer::*;

use bevy::state::state::States;

/// Defines app asset loading states
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}
