use bevy::prelude::*;
use crate::components::buttons::{ClearButton, StackButton};

pub fn spawn_app_name_row(app_name: String) -> impl Bundle {
    (
        Node {
            width: Val::Px(509.0),
            height: Val::Auto,
            // padding: UiRect::all(Val::Px(5.0)),
            margin: UiRect::bottom(Val::Px(15.0)),
            bottom: Val::Px(5.0),
            top: Val::Px(5.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            ..default()
        },
        BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
        BorderRadius::all(Val::Px(8.0)),
        children![
            (
                Node {
                    width: Val::Px(220.0),
                    height: Val::Auto,
                    padding: UiRect::left(Val::Px(2.0)),
                    margin: UiRect::right(Val::Px(210.0)),
                    ..default()
                },
                children![(
                    Text::new(&app_name),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                )],
            ),
            (
                Button,
                Node {
                    width: Val::Px(28.0),
                    height: Val::Px(26.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::right(Val::Px(5.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(8.0)),
                StackButton { app_name: app_name.clone(), is_stacked: false },
                BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
                children![(
                    Text::new("⌄"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::WHITE),
                )],
            ),

            (
                Button,
                Node {
                    width: Val::Px(28.0),
                    height: Val::Px(26.0),
                    // top: Val::Px(8.0),
                    justify_content: JustifyContent::Center, // <-- Center horizontally
                    align_items: AlignItems::Center,
                    // margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(8.0)),
                ClearButton { app_name: app_name.clone() },
                BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
                children![(
                    Text::new("×"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::WHITE),
                )],
            )
        ],
    )
}