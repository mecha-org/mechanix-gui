use super::systems::update_button;
use bevy::prelude::*;

pub struct StyledButtonPlugin;
impl Plugin for StyledButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button);
    }
}
