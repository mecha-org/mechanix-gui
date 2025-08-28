use crate::components::*;
use bevy::{color::palettes::css::*, prelude::*};

pub fn ui(mut commands: &Commands) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            ..Default::default()
        },
        children![notifications_list()],
    )
}
