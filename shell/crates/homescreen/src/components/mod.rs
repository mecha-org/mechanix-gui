use bevy::{
    color::palettes::css::*,
    ecs::{
        relationship::{RelatedSpawner, RelatedSpawnerCommands},
        spawn::SpawnWith,
    },
    prelude::*,
};

pub fn widgets_and_apps(commands: &mut Commands) -> impl Bundle {
    (
        Node {
            height: Val::Percent(100.0),
            aspect_ratio: Some(1.0),
            display: Display::Grid,
            padding: UiRect::all(Val::Px(24.0)),
            grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
            row_gap: Val::Px(12.0),
            column_gap: Val::Px(12.0),
            ..default()
        },
        Children::spawn(SpawnWith(|builder: &mut RelatedSpawner<ChildOf>| {
            item_rect(builder, ORANGE);
            item_rect(builder, BISQUE);
            item_rect(builder, BLUE);
            item_rect(builder, CRIMSON);
            item_rect(builder, AQUA);
            item_rect(builder, ORANGE_RED);
            item_rect(builder, DARK_GREEN);
            item_rect(builder, FUCHSIA);
            item_rect(builder, TEAL);
            item_rect(builder, ALICE_BLUE);
        })),
    )
}

fn item_rect(builder: &mut RelatedSpawner<ChildOf>, color: Srgba) {
    builder
        .spawn((
            Node {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(BLACK.into()),
        ))
        .with_children(|builder| {
            builder.spawn((Node::default(), BackgroundColor(color.into())));
        });
}
