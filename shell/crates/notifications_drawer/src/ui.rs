use crate::components::*;
use bevy::prelude::*;
use bevy_core_widgets::{ CoreScrollArea };
pub fn ui(_commands: &Commands) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            ..Default::default()
        },
        CoreScrollArea,
        ScrollPosition {
            offset_x: 0.0,
            offset_y: 0.0,
        },
        children![notifications_list()],
    )
}
