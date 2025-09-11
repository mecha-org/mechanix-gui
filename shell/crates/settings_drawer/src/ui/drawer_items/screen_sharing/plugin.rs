use bevy::prelude::*;

use crate::ui::{ScreenSharingEnabled, SettingsDrawerState, update_screen_sharing_state};

pub struct ScreenSharingUiPlugin;

impl Plugin for ScreenSharingUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScreenSharingEnabled>();
        app.add_systems(
            Update,
            update_screen_sharing_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<ScreenSharingEnabled>),
        );
    }
}
