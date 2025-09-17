use bevy::prelude::*;

use crate::ui::SettingsDrawerState;

#[derive(Event)]
pub struct SettingsDrawerOpen;

#[derive(Event)]
pub struct SettingsDrawerClose;

pub fn listen_open_event(
    mut trigger: Trigger<SettingsDrawerOpen>,
    mut state: ResMut<NextState<SettingsDrawerState>>,
) {
    trigger.propagate(false);
    state.set(SettingsDrawerState::Opened);
}

pub fn listen_close_event(
    mut trigger: Trigger<SettingsDrawerClose>,
    mut state: ResMut<NextState<SettingsDrawerState>>,
) {
    trigger.propagate(false);
    state.set(SettingsDrawerState::NavigationOnly);
}
