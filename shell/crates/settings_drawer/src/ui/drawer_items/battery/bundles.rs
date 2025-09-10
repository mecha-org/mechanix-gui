use bevy::prelude::*;
use types::prelude::FontAssets;

use crate::icons::SettingsDrawerIcons;

pub fn battery(icons: &SettingsDrawerIcons, fonts: &FontAssets) -> impl Bundle {
    (
        Node {
            height: Val::Px(24.),
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new("60%"),
                TextFont {
                    font: fonts.primary_500.clone(),
                    font_size: 16.,
                    ..default()
                },
                TextColor(Color::oklch(0.934, 0., 0.))
            ),
            (
                ImageNode::new(icons.battery_10.clone()),
                Node {
                    margin: UiRect::left(Val::Px(4.)),
                    width: Val::Px(24.),
                    height: Val::Px(24.),
                    ..default()
                }
            )
        ],
    )
}
