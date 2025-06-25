use super::systems::update_button;
use bevy::prelude::*;

pub struct StyledAppBundlePlugin;
impl Plugin for StyledAppBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button);
    }
}
