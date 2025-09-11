use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};
use headless_widgets::prelude::*;

use crate::{button_system::NORMAL_BUTTON, types::DesktopApp};

pub fn frequently_used_apps(freq_used_apps: Vec<DesktopApp>) -> impl Bundle {
    (
        Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Px(76.),
            grid_template_columns: RepeatedGridTrack::flex(6, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
            column_gap: Val::Px(26.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1.)),
            padding: UiRect {
                left: Val::Px(9.),
                right: Val::Px(9.),
                top: Val::Px(8.),
                bottom: Val::Px(8.),
            },
            ..Default::default()
        },
        BorderColor(Color::oklcha(0.3329, 0., 0., 0.95)),
        BorderRadius::all(Val::Px(12.)),
        BackgroundColor(Color::oklcha(0.2221, 0., 0., 0.95)),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for app in freq_used_apps.into_iter().take(6) {
                parent.spawn(frequently_used_app(&app));
            }
        })),
    )
}

pub fn frequently_used_app(app: &DesktopApp) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![(
            ImageNode::new(app.icon.clone()),
            Node {
                width: Val::Percent(75.),
                height: Val::Percent(75.),
                ..Default::default()
            }
        )],
        BorderRadius::all(Val::Px(11.82)),
        BackgroundColor(NORMAL_BUTTON),
        Button,
        CoreButton::new().on_click(app.on_click),
    )
}
