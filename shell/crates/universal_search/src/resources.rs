use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct IsOpen(pub bool);
