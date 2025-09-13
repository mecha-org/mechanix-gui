use bevy::prelude::*;

use crate::icons::SettingsDrawerIcons;

pub fn settings(icons: &SettingsDrawerIcons) -> impl Bundle {
    (
        Node {
            width: Val::Px(32.82),
            height: Val::Px(32.82),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            ImageNode::new(icons.settings.clone()),
            Node {
                width: Val::Px(24.),
                height: Val::Px(24.),
                ..default()
            }
        )],
    )
}
