use bevy::prelude::*;

use crate::ui::{MicrophoneEnabled, SettingsDrawerState, update_microphone_state};

pub struct MicrophoneUiPlugin;

impl Plugin for MicrophoneUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MicrophoneEnabled>();
        app.add_systems(
            Update,
            update_microphone_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<MicrophoneEnabled>),
        );
    }
}
