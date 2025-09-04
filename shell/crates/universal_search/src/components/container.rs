use animation::tween::AnimationTarget;
use bevy::{color::palettes::css::*, prelude::*};

use crate::systems::NORMAL_BUTTON;

#[derive(Component)]
pub struct Container;

pub fn container() -> impl Bundle {
    (
        Node {
            width: Val::Percent(0.),
            height: Val::Percent(0.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        Container,
        AnimationTarget,
        BackgroundColor(DARK_GRAY.into()),
        // children![(
        //     Button,
        //     Node {
        //         width: Val::Px(150.0),
        //         height: Val::Px(40.0),
        //         border: UiRect::all(Val::Px(5.0)),
        //         // horizontally center child text
        //         justify_content: JustifyContent::Center,
        //         // vertically center child text
        //         align_items: AlignItems::Center,
        //         ..default()
        //     },
        //     BorderColor(Color::BLACK),
        //     BorderRadius::MAX,
        //     BackgroundColor(NORMAL_BUTTON),
        //     children![(
        //         Text::new("Settings Drawer"),
        //         TextFont {
        //             font_size: 16.0,
        //             ..default()
        //         },
        //         TextColor(Color::srgb(0.9, 0.9, 0.9)),
        //         TextShadow::default(),
        //     )]
        // )],
    )
}
