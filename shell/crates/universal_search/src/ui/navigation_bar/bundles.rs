use crate::ui::BAR_SIZE;

use super::components::Bar;
use bevy::prelude::*;

pub fn bar(left_nav_bar: Handle<Image>) -> impl Bundle {
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
        ImageNode::new(left_nav_bar.clone()),
        ZIndex(9999),
    )
}
