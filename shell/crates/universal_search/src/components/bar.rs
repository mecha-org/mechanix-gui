use bevy::{color::palettes::css::*, prelude::*};

use crate::systems::NORMAL_BUTTON;

pub const BAR_SIZE: (f32, f32) = (100., 50.);

pub fn bar() -> impl Bundle {
    (
        Node {
            width: Val::Px(BAR_SIZE.0),
            height: Val::Px(BAR_SIZE.1),
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Button"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}
