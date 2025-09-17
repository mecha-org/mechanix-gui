use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId},
    prelude::*,
};
use headless_widgets::{CoreButton, CoreScrollArea};
use types::prelude::FontAssets;
use utils::prelude::DesktopApp;

use crate::{
    button_system::NORMAL_BUTTON, icons::AppDrawerIcons, ui::category_expanded::CategoryExpanded,
};
pub fn category_exapanded(
    name: String,
    apps: Vec<DesktopApp>,
    on_popup_click: SystemId,
    font_assets: &FontAssets,
    icons: &AppDrawerIcons,
) -> impl Bundle {
    let font_assets = font_assets.clone();
    let primary_500 = font_assets.primary_500.clone();
    let icons = icons.clone();
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            position_type: PositionType::Absolute,
            padding: UiRect {
                left: Val::Px(16.),
                right: Val::Px(16.),
                top: Val::Px(8.),
                ..default()
            },
            ..Default::default()
        },
        CategoryExpanded,
        ZIndex(9999),
        BackgroundColor(Color::oklcha(0.1286, 0., 0., 0.95)),
        CoreButton::new().on_click(on_popup_click),
        children![(
            Node {
                width: Val::Percent(100.),
                height: Val::Px(407.),
                ..Default::default()
            },
            BorderRadius::all(Val::Px(13.91)),
            BackgroundColor(Color::oklch(0.2435, 0., 0.)),
            children![
                (
                    Node {
                        display: Display::Grid,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        column_gap: Val::Px(48.),
                        row_gap: Val::Px(37.),
                        overflow: Overflow::scroll_y(),
                        padding: UiRect {
                            left: Val::Px(22.),
                            right: Val::Px(22.),
                            top: Val::Px(10.),
                            ..default()
                        },
                        ..Default::default()
                    },
                    ScrollPosition {
                        offset_x: 0.0,
                        offset_y: 0.0,
                    },
                    CoreScrollArea,
                    CoreButton::new(),
                    Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
                        for app in apps {
                            parent.spawn(category_app_button(app, &font_assets));
                        }
                    }))
                ),
                (
                    ImageNode::new(icons.lg_curve_image.clone()),
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Px(42.),
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.),
                        bottom: Val::Px(0.),
                        align_items: AlignItems::Center,
                        padding: UiRect::left(Val::Px(16.)),
                        ..default()
                    },
                    children![(
                        Text::new(name.chars().into_iter().take(11).collect::<String>()),
                        TextFont {
                            font: primary_500.clone(),
                            font_size: 18.,
                            ..default()
                        },
                        TextColor(Color::oklch(0.9158, 0., 0.))
                    )],
                )
            ]
        )],
    )
}

fn category_app_button(app: DesktopApp, font_assets: &FontAssets) -> impl Bundle {
    (
        Node {
            width: Val::Px(80.),
            min_height: Val::Px(126.),
            // padding: UiRect {
            //     left: Val::Px(0.),
            //     right: Val::Px(0.),
            //     top: Val::Px(8.),
            //     bottom: Val::Px(3.),
            // },
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(14.),
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![
            (
                Node {
                    width: Val::Px(80.),
                    height: Val::Px(80.),
                    padding: UiRect::all(Val::Px(12.17)),
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(13.91)),
                BackgroundColor(NORMAL_BUTTON),
                Button,
                CoreButton::new().on_click(app.on_click),
                children![ImageNode::new(app.icon.clone()),]
            ),
            (
                Text::new(app.name.chars().collect::<String>(),),
                TextFont {
                    font: font_assets.primary_500.clone(),
                    font_size: 16.,
                    ..Default::default()
                },
                TextColor(Color::oklch(0.9551, 0., 0.)),
            ),
        ],
    )
}
