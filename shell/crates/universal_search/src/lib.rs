use bevy::prelude::*;

mod components;
mod systems;
mod ui;

use systems::*;

pub struct UniversalSearchPlugin;
impl Plugin for UniversalSearchPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (button_system, exit_on_esc));
    }
}

pub mod prelude {
    pub use crate::UniversalSearchPlugin;
}
