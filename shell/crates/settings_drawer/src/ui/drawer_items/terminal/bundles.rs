use bevy::{ecs::system::SystemId, prelude::*};
use headless_widgets::CoreButton;
use types::prelude::FontAssets;

use crate::{icons::SettingsDrawerIcons, ui::ButtonType3};

pub fn terminal(
    on_click: SystemId,
    fonts: &FontAssets,
    icons: &SettingsDrawerIcons,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            border: UiRect::all(Val::Px(1.)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BorderColor(Color::oklch(0.2435, 0., 0.)),
        BorderRadius::all(Val::Px(8.)),
        BackgroundColor(Color::oklch(0.209, 0., 0.)),
        CoreButton::new().on_click(on_click),
        ButtonType3,
        children![(
            Node {
                width: Val::Px(88.),
                height: Val::Px(58.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    ImageNode::new(icons.terminal.clone()),
                    Node {
                        width: Val::Px(40.),
                        height: Val::Px(40.),
                        margin: UiRect::bottom(Val::Px(4.)),
                        ..default()
                    }
                ),
                (
                    Text::new("Terminal"),
                    TextFont {
                        font: fonts.primary_400.clone(),
                        font_size: 12.,
                        ..default()
                    },
                    TextColor(Color::oklch(0.9672, 0., 0.)),
                )
            ]
        )],
    )
}
