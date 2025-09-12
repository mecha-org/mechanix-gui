use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct SearchText(pub String);

#[derive(Debug, Resource)]
pub struct SearchActive(pub bool);
