use bevy::{
    color::palettes::css::*,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId},
    prelude::*,
};
use headless_widgets::CoreSlider;

use crate::{
    icons::SettingsDrawerIcons,
    ui::{Brightness, BrightnessFilledArea},
};

pub fn brightness(on_change: SystemId<In<f32>>, icons: &SettingsDrawerIcons) -> impl Bundle {
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
                ImageNode::new(icons.brightness_low.clone()),
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
                    grid_template_columns: RepeatedGridTrack::flex(19, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(7, 1.0),
                    column_gap: Val::Px(7.),
                    row_gap: Val::Px(7.),
                    ..default()
                },
                Brightness,
                CoreSlider {
                    value: 0.,
                    min: 0.,
                    max: 100.,
                    thumb_size: 0.,
                    on_change: Some(on_change),
                    ..default()
                },
                Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
                    for _ in 0..19 {
                        for _ in 0..7 {
                            parent.spawn((
                                Node {
                                    width: Val::Px(2.),
                                    height: Val::Px(2.),
                                    ..default()
                                },
                                BackgroundColor(Color::oklch(0.5761, 0., 0.)),
                                BorderRadius::all(Val::Percent(50.)),
                            ));
                        }
                    }
                    parent.spawn((
                        Node {
                            width: Val::Percent(20.),
                            height: Val::Percent(100.),
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.),
                            left: Val::Px(0.),
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        BackgroundColor(Color::oklch(0.934, 0., 0.)),
                        BorderRadius::all(Val::Px(2.)),
                        BrightnessFilledArea,
                    ));
                })),
            )
        ],
    )
}
