use bevy::{
    color::palettes::css::*,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};
pub mod cards;
#[derive(Component)]
pub struct NotificationButton {
    pub id: u32,
    pub title: String,
    pub content: String,
}

pub fn notifications_list() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.),
            padding: UiRect::all(Val::Px(12.0)),
            ..default()
        },
        Children::spawn(SpawnWith(|builder: &mut RelatedSpawner<ChildOf>| {
            notification(builder, ORANGE, 1, "System Update", "System update available");
            notification(builder, BISQUE, 2, "Message", "New message received");
            notification(builder, BLUE, 3, "Calendar", "Meeting reminder");
            notification(builder, CRIMSON, 4, "Alert", "Low battery warning");
        })),
    )
}

fn notification(builder: &mut RelatedSpawner<ChildOf>, color: Srgba, id: u32, title: &str, content: &str) {
    let title_owned = title.to_string();
    let content_owned = content.to_string();
    
    builder.spawn((
        Button,
        Node {
            width: Val::Percent(100.),
            height: Val::Px(100.0),
            padding: UiRect::all(Val::Px(12.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexStart,
            ..default()
        },
        BackgroundColor(color.into()),
        BorderRadius::all(Val::Px(8.0)),
        BorderColor(Color::WHITE),
        NotificationButton {
            id,
            title: title.to_string(),
            content: content.to_string(),
        },
        Children::spawn(SpawnWith(move |builder: &mut RelatedSpawner<ChildOf>| {
            // Title text
            builder.spawn((
                Text::new(&title_owned),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(4.0)),
                    ..default()
                },
            ));
            // Content text
            builder.spawn((
                Text::new(&content_owned),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        })),
    ));
}
