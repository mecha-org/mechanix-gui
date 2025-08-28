use crate::components::*;
use bevy::prelude::*;

pub fn ui(commands: &mut Commands) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            ..Default::default()
        },
        children![widgets_and_apps(commands)],
    )
}
