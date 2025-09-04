use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId},
    prelude::*,
};

use crate::types::{DesktopApp, SearchResult, SearchResultType};

pub fn search_results(
    results: Vec<SearchResult>,
    results_for: String,
    browser_apps: Vec<DesktopApp>,
    asset_server: &AssetServer,
) -> impl Bundle {
    let arrow_up_right: Handle<Image> = asset_server.load("icons/arrow_up_right.png");
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            top: Val::Px(0.),
            left: Val::Px(0.),
            right: Val::Px(0.),
            bottom: Val::Px(0.),
            padding: UiRect::all(Val::Px(16.)),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            display: Display::Flex,
            overflow: Overflow::scroll_y(),
            ..Default::default()
        },
        // CoreScrollArea,
        ScrollPosition {
            offset_x: 0.0,
            offset_y: 0.0,
        },
        // SearchResultsComponent,
        BackgroundColor(Color::oklch(0.173, 0., 0.)),
        ZIndex(99),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for result in results {
                parent.spawn(search_result_item(result, arrow_up_right.clone()));
            }

            browser_apps.iter().for_each(|app| {
                parent.spawn(search_result_item(
                    SearchResult {
                        name: format!("Search {} on {}", results_for.clone(), app.name.clone()),
                        icon: app.icon.clone(),
                        on_click: None,
                        _type: SearchResultType::App,
                    },
                    arrow_up_right.clone(),
                ));
            });
        })),
    )
}
fn separator() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(2.),
            ..Default::default()
        },
        BackgroundColor(Color::oklch(0.2435, 0., 0.)),
    )
}

pub fn search_result_item(result: SearchResult, arrow_up_right_icon: Handle<Image>) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        children![
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(28.),
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.),
                    margin: UiRect::vertical(Val::Px(12.)),
                    ..Default::default()
                },
                // CoreButton {
                //     on_click: None,
                //     on_long_press: None,
                // },
                Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
                    parent.spawn((
                        Node {
                            width: Val::Px(20.),
                            height: Val::Px(20.),
                            padding: UiRect::all(Val::Px(3.)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        ImageNode::new(result.icon.clone()),
                    ));
                    parent.spawn((
                        Node {
                            flex_grow: 1.,
                            ..Default::default()
                        },
                        Text::new(result.name.clone()),
                        TextFont {
                            font_size: 16.,
                            ..Default::default()
                        },
                        TextColor(Color::oklch(0.7572, 0., 0.)),
                    ));
                    parent.spawn((
                        ImageNode::new(arrow_up_right_icon.clone()),
                        Node {
                            width: Val::Px(18.),
                            height: Val::Px(18.),
                            ..Default::default()
                        },
                    ));
                }))
            ),
            separator(),
        ],
    )
}
