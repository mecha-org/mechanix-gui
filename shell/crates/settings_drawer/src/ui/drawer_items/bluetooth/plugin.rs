use crate::ui::{SettingsDrawerState, update_bluetooth_state};
use bevy::prelude::*;
use service_plugins::bluetooth::BluetoothEnabledStatus;

pub struct BluetoothUiPlugin;

impl Plugin for BluetoothUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_bluetooth_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<BluetoothEnabledStatus>),
        );
    }
}
