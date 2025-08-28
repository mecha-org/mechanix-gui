use bevy::prelude::*;

mod components;
mod systems;
mod ui;

use systems::*;

pub struct NotificationsDrawerPlugin;
impl Plugin for NotificationsDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (button_system, exit_on_esc));
    }
}

pub mod prelude {
    pub use crate::NotificationsDrawerPlugin;
}
