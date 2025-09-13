use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone)]
pub struct PowerSavingModeEnabled(pub bool);
