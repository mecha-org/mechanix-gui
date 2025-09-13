use bevy::prelude::*;

use crate::ui::{BrightnessValue, SettingsDrawerState, update_brightness_state};

pub struct BrightnessUiPlugin;

impl Plugin for BrightnessUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BrightnessValue>();
        app.add_systems(
            Update,
            update_brightness_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<BrightnessValue>),
        );
    }
}
