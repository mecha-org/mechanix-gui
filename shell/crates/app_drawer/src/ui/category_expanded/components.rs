use bevy::prelude::*;

#[derive(Resource)]
pub struct CategoryPopupFor(pub String);

#[derive(Component)]
pub struct CategoryExpanded;
