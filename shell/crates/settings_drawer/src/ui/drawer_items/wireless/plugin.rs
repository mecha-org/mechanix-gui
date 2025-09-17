use crate::ui::{SettingsDrawerState, update_active_network_strength, update_wireless_state};
use bevy::prelude::*;
use service_plugins::network_manager::{ActiveNetworkStrength, WirelessEnabled};

pub struct WirelessUiPlugin;

impl Plugin for WirelessUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_wireless_state
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<WirelessEnabled>),
        );

        app.add_systems(
            Update,
            update_active_network_strength
                .run_if(in_state(SettingsDrawerState::Opened))
                .run_if(resource_changed::<ActiveNetworkStrength>),
        );
    }
}
