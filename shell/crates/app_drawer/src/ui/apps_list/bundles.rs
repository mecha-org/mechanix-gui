use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};
use headless_widgets::{CoreButton, CoreScrollArea};
use types::prelude::FontAssets;
use utils::prelude::*;

use crate::{button_system::NORMAL_BUTTON, setup::WINDOW_SIZE, ui::apps_list::AppsList};

pub fn apps_list(font_assets: &FontAssets, desktop_apps: Vec<DesktopApp>) -> impl Bundle {
    let desktop_apps = desktop_apps.clone();
    let font_assets = font_assets.clone();
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(WINDOW_SIZE.1 - 56.),
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(16.)),
            row_gap: Val::Px(10.),
            overflow: Overflow::scroll_y(),
            ..Default::default()
        },
        CoreScrollArea,
        ScrollPosition {
            offset_x: 0.0,
            offset_y: 0.0,
        },
        BackgroundColor(Color::BLACK),
        AppsList,
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for app in desktop_apps {
                parent.spawn(app_list_app(&font_assets, &app));
                parent.spawn(separator());
            }
        })),
    )
}

pub fn app_list_app(font_assets: &FontAssets, app: &DesktopApp) -> impl Bundle {
    (
        Node {
            width: Val::Percent(90.),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.),
            padding: UiRect::vertical(Val::Px(8.)),
            ..Default::default()
        },
        CoreButton::new().on_click(app.on_click),
        children![
            (
                Node {
                    width: Val::Px(36.),
                    height: Val::Px(36.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(6.26)),
                BackgroundColor(NORMAL_BUTTON),
                Button,
                children![(
                    Node {
                        width: Val::Px(23.47),
                        height: Val::Px(23.47),
                        ..Default::default()
                    },
                    ImageNode::new(app.icon.clone()),
                )],
            ),
            (
                Text::new(app.name.clone()),
                TextFont {
                    font_size: 16.,
                    font: font_assets.primary_500.clone(),
                    ..Default::default()
                },
                TextColor(Color::oklch(0.7572, 0., 0.))
            )
        ],
    )
}

pub fn separator() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            min_height: Val::Px(1.),
            ..Default::default()
        },
        BackgroundColor(Color::oklch(0.2435, 0., 0.)),
    )
}
