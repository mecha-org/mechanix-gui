use bevy::prelude::*;

#[derive(Resource, Default, Debug, Clone)]
pub struct MicrophoneEnabled(pub bool);
