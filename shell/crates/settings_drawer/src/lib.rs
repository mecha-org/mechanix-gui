use bevy::prelude::*;

mod components;
mod states;
mod systems;
mod ui;

use systems::*;

use crate::states::{Action, AnimationState, animate_closing, animate_opening, listen_action};

pub struct SettingsDrawerPlugin;
impl Plugin for SettingsDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (button_system, exit_on_esc));
        app.add_event::<Action>();
        app.init_state::<AnimationState>();

        app.add_observer(on_bar_drag_start);
        app.add_observer(on_bar_drag);
        app.add_observer(on_bar_drag_end);

        app.add_systems(Update, listen_action);
        app.add_systems(
            Update,
            animate_opening.run_if(in_state(AnimationState::Opening)),
        );
        app.add_systems(
            Update,
            animate_closing.run_if(in_state(AnimationState::Closing)),
        );
    }
}

pub mod prelude {
    pub use crate::SettingsDrawerPlugin;
}
