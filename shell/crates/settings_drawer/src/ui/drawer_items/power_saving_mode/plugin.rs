use bevy::prelude::*;

use crate::ui::{PowerSavingModeEnabled, SettingsDrawerState, update_power_saving_mode_state};

pub struct PowerSavingModeUiPlugin;

impl Plugin for PowerSavingModeUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerSavingModeEnabled>();
        app.add_systems(
            Update,
            update_power_saving_mode_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<PowerSavingModeEnabled>),
        );
    }
}
