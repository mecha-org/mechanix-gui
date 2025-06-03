use super::systems::update_button;
use bevy::prelude::*;

pub struct StyledControlPlugin ;
impl Plugin for StyledControlPlugin  {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button);
    }
}
