use bevy::{color::palettes::css::*, ecs::system::SystemId, prelude::*};
use types::prelude::*;

use crate::icons::UniversalSearchIcons;
pub fn search_input(
    // on_search_click: SystemId,
    // on_cancel_click: SystemId,
    // on_clear_click: SystemId,
    font_assets: &FontAssets,
    icons: &UniversalSearchIcons,
) -> impl Bundle {
    (
        Node {
            left: Val::Px(0.),
            bottom: Val::Px(0.),
            width: Val::Percent(100.),
            height: Val::Px(56.),
            position_type: PositionType::Absolute,
            padding: UiRect {
                left: Val::Px(16.),
                right: Val::Px(16.),
                top: Val::Px(8.),
                bottom: Val::Px(8.),
            },
            ..Default::default()
        },
        ZIndex(999),
        BackgroundColor(Color::oklch(0.2308, 0., 0.)),
        BorderRadius::top(Val::Px(16.)),
        children![
            (
                Node {
                    flex_grow: 1.,
                    height: Val::Px(40.),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                // CoreButton {
                //     on_click: Some(on_search_click),
                //     on_long_press: None,
                // },
                children![
                    (
                        Node {
                            justify_self: JustifySelf::Center,
                            margin: UiRect::right(Val::Px(8.)),
                            ..Default::default()
                        },
                        ImageNode::new(icons.search.clone()),
                    ),
                    (
                        Text::new("Type here..."),
                        TextColor(Color::oklch(0.7252, 0., 0.)),
                        TextFont {
                            font: font_assets.primary_400.clone(),
                            font_size: 16.,
                            ..Default::default()
                        },
                        Node {
                            justify_self: JustifySelf::Center,
                            flex_grow: 1.,
                            ..Default::default()
                        },
                        // SearchInput
                    ),
                    (
                        Node {
                            align_items: AlignItems::Center,
                            justify_self: JustifySelf::Center,
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Val::Px(1.)),
                            width: Val::Px(48.),
                            height: Val::Px(40.),
                            ..default()
                        },
                        BorderColor(Color::oklch(0.4202, 0., 0.)),
                        BorderRadius::all(Val::Px(22.)),
                        children![(
                            Node {
                                width: Val::Px(14.),
                                height: Val::Px(14.),
                                ..Default::default()
                            },
                            ImageNode::new(icons.close.clone()),
                            // CoreButton {
                            //     on_click: Some(on_clear_click),
                            //     on_long_press: None,
                            // },
                        )]
                    ),
                ]
            ),
            // (
            //     Text::new("Cancel"),
            //     CoreButton {
            //         on_click: Some(on_cancel_click),
            //         on_long_press: None,
            //     }
            // )
        ],
    )
}
