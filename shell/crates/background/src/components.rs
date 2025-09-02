pub use bevy::prelude::*;

pub fn background(asset_server: &Res<AssetServer>) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ImageNode::new(asset_server.load("background.png")),
    )
}
