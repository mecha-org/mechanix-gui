pub mod button;

use bevy::{
    app::{App, Plugin},
    input_focus::InputDispatchPlugin,
};
use bevy_core_widgets::CoreWidgetsPlugin;
use button::StyledButtonPlugin;

pub struct StyledWidgetsPlugin;

impl Plugin for StyledWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CoreWidgetsPlugin, InputDispatchPlugin, StyledButtonPlugin));
    }
}
