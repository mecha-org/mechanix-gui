use bevy::prelude::*;

pub const BAR_SIZE: (f32, f32) = (120., 30.);

pub fn bar() -> impl Bundle {
    (
        Node {
            width: Val::Px(BAR_SIZE.0),
            height: Val::Px(BAR_SIZE.1),
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        children![(
            Node {
                width: Val::Percent(100.),
                height: Val::Px(4.),
                ..default()
            },
            BorderRadius::all(Val::Px(4.)),
            BackgroundColor(Color::oklch(0.4202, 0., 0.)),
        )],
    )
}
