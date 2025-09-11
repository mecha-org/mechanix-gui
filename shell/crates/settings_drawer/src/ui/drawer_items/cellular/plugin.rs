use crate::ui::{CellularEnabled, SettingsDrawerState, update_cellular_state};
use bevy::prelude::*;

pub struct CellularUiPlugin;

impl Plugin for CellularUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CellularEnabled>();
        app.add_systems(
            Update,
            update_cellular_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<CellularEnabled>),
        );
    }
}
