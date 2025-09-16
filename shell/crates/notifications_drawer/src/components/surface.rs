use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct NotificationSurface;

#[derive(Resource)]
pub struct NotificationSurfaceEntity(pub Entity);

#[derive(Component)]
pub struct NotificationStack {
    pub app_name: String,
}

#[derive(Resource, Default)]
pub struct NotificationStacks(pub HashMap<String, Entity>);

pub fn notification_stacks_init(mut commands: Commands) {
    commands.insert_resource(NotificationStacks(HashMap::new()));
}