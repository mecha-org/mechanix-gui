use crate::components::*;
use bevy::prelude::*;

pub fn ui(mut commands: &Commands, asset_server: &Res<AssetServer>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![background(&asset_server)],
    )
}
