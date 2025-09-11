use bevy::prelude::*;

use crate::ui::{AirplaneModeEnabled, SettingsDrawerState, update_airplane_mode_state};

pub struct AirplaneModeUiPlugin;

impl Plugin for AirplaneModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AirplaneModeEnabled>();
        app.add_systems(
            Update,
            update_airplane_mode_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<AirplaneModeEnabled>),
        );
    }
}
