pub mod app_bundle;
pub mod control_bundle;

use bevy::{
    app::{App, Plugin},
    input_focus::InputDispatchPlugin,
};
use bevy_core_widgets::CoreWidgetsPlugin;
use app_bundle::StyledAppBundlePlugin;
use control_bundle::StyledControlPlugin;

pub struct StyledWidgetsPlugin;

impl Plugin for StyledWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CoreWidgetsPlugin, InputDispatchPlugin, StyledControlPlugin, StyledAppBundlePlugin));
    }
}
