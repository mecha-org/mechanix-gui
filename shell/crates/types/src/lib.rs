use bevy::state::state::States;

mod fonts;
mod icons;

pub mod prelude {
    pub use crate::fonts::FontAssets;
    pub use crate::icons::IconAssets;
}

/// Defines app asset loading states
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}
