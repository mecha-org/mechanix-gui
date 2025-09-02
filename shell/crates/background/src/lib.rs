mod components;
mod systems;
mod ui;

use bevy::prelude::*;
use systems::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup);
        app.add_systems(Update, exit_on_esc);
    }
}

pub mod prelude {
    pub use crate::BackgroundPlugin;
}
