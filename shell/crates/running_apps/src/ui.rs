use crate::components::*;
use bevy::prelude::*;

pub fn ui(mut commands: &Commands) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![bar()],
    )
}
