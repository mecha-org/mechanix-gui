use bevy::prelude::*;

use crate::ui::{SettingsDrawerState, VolumeValue, update_volume_state};

pub struct VolumeUiPlugin;

impl Plugin for VolumeUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VolumeValue>();
        app.add_systems(
            Update,
            update_volume_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<VolumeValue>),
        );
    }
}
