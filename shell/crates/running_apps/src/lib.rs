use bevy::prelude::*;

mod components;
mod systems;
mod ui;

use systems::*;

pub struct RunningAppsPlugin;
impl Plugin for RunningAppsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, exit_on_esc);
    }
}

pub mod prelude {
    pub use crate::RunningAppsPlugin;
}
