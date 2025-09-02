mod components;
mod constants;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (button_system, exit_on_esc));
    }
}

pub mod prelude {
    pub use crate::StatusBarPlugin;
}
