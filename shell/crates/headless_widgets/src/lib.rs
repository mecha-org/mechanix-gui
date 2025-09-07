use bevy::{
    app::{App, Plugin, Update},
    input_focus::InputDispatchPlugin,
};
mod core_button;
mod events;

mod interaction_states;
pub use core_button::{CoreButton, CoreButtonPlugin};
pub use interaction_states::{ButtonPressed, Checked, InteractionDisabled};

pub struct CoreWidgetsPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CoreButtonPlugin, ));
    }
}

pub mod prelude {
    pub use crate::core_button::{CoreButton, CoreButtonPlugin};
}
