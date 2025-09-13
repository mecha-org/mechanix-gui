use bevy::prelude::*;

use crate::ui::{ScreenRecordingEnabled, SettingsDrawerState, update_screen_recording_state};

pub struct ScreenRecodingUiPlugin;

impl Plugin for ScreenRecodingUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenRecordingEnabled>();
        app.add_systems(
            Update,
            update_screen_recording_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<ScreenRecordingEnabled>),
        );
    }
}
