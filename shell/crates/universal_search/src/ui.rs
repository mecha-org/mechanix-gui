use crate::components::*;
use bevy::prelude::*;

pub fn ui(mut commands: &Commands) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        children![bar()],
    )
}
