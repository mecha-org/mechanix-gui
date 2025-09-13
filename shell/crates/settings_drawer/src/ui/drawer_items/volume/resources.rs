use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct VolumeValue(pub f32);
