use bevy::{
    color::palettes::css::*,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId},
    prelude::*,
};
use headless_widgets::CoreSlider;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{Volume, VolumeFilledArea},
};

pub fn volume(on_change: SystemId<In<f32>>, icons: &SettingsDrawerIcons) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            padding: UiRect::left(Val::Px(8.)),
            column_gap: Val::Px(6.),
            ..default()
        },
        BackgroundColor(Color::oklch(0.2435, 0., 0.)),
        BorderRadius::all(Val::Px(8.)),
        children![
            (
                ImageNode::new(icons.sound_low.clone()),
                Node {
                    width: Val::Px(36.),
                    height: Val::Px(36.),
                    ..default()
                }
            ),
            (
                Node {
                    width: Val::Px(172.),
                    height: Val::Px(56.),
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(29, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                    column_gap: Val::Px(4.),
                    ..default()
                },
                Volume,
                CoreSlider {
                    value: 0.,
                    min: 0.,
                    max: 100.,
                    thumb_size: 0.,
                    on_change: Some(on_change),
                    ..default()
                },
                Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
                    for i in 0..29 {
                        parent.spawn((
                            Node {
                                width: Val::Px(2.),
                                height: Val::Percent(100.),
                                ..default()
                            },
                            VolumeFilledArea(i),
                            BackgroundColor(Color::oklch(0.4202, 0., 0.)),
                            BorderRadius::all(Val::Px(2.)),
                        ));
                    }
                })),
            )
        ],
    )
}
