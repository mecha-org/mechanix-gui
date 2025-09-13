use bevy::{
    color::palettes::css::*,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

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
            notification(builder, ORANGE);
            notification(builder, BISQUE);
            notification(builder, BLUE);
            notification(builder, CRIMSON);
        })),
    )
}

fn notification(builder: &mut RelatedSpawner<ChildOf>, color: Srgba) {
    builder.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Px(100.0),
            padding: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BackgroundColor(color.into()),
        BorderRadius::all(Val::Px(8.0)),
    ));
}
