pub mod homescreen;
pub mod navigation_bar;
pub mod settings_drawer;
pub mod status_bar;
pub mod universal_search;
pub mod screenshot;
pub use homescreen::*;
pub use navigation_bar::*;
pub use settings_drawer::*;
pub use status_bar::*;
pub use universal_search::*;

use bevy::state::state::States;

/// Defines app asset loading states
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}
