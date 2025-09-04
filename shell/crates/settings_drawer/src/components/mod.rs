mod bar;
mod grid;
pub mod styled_card;

use bevy::prelude::Component;
pub use bar::{BAR_SIZE, Bar, bar};
pub use grid::{Grid, grid};

#[derive(Component)]
pub struct Clock;

#[derive(Component)]
pub struct SettingsItem;