use crate::ui::BAR_SIZE;

use super::components::Bar;
use bevy::prelude::*;

pub fn bar(right_nav_bar: Handle<Image>) -> impl Bundle {
    (
        Bar,
        Node {
            width: Val::Px(BAR_SIZE.0),
            height: Val::Px(BAR_SIZE.1),
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.),
            right: Val::Px(0.),
            ..default()
        },
        ImageNode::new(right_nav_bar.clone()),
        ZIndex(9999),
    )
}
