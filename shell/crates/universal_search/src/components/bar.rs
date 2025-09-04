use bevy::{color::palettes::css::*, prelude::*};

use crate::{components::BAR_SIZE, systems::NORMAL_BUTTON};

#[derive(Component)]
pub struct Bar;

pub fn bar() -> impl Bundle {
    (
        Bar,
        Node {
            width: Val::Px(BAR_SIZE.0),
            height: Val::Px(BAR_SIZE.1),
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.),
            left: Val::Px(0.),
            ..default()
        },
        BackgroundColor(DARK_GRAY.into()),
    )
}
