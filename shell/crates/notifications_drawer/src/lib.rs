use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod plugin;
mod ui;

pub use plugin::NotificationDrawerPlugin;

pub mod prelude {
    pub use crate::NotificationDrawerPlugin;
}
