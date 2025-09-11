use bevy::prelude::*;

use crate::ui::{RotationEnabled, SettingsDrawerState, update_rotation_state};

pub struct RotationUiPlugin;

impl Plugin for RotationUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RotationEnabled>();
        app.add_systems(
            Update,
            update_rotation_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<RotationEnabled>),
        );
    }
}
