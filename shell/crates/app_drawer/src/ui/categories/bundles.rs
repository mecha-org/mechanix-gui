use std::collections::HashMap;

use bevy::{
    asset::ron::de,
    color::palettes::css::*,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId},
    prelude::*,
};
use headless_widgets::{CoreButton, CoreScrollArea};
use types::prelude::FontAssets;

use crate::{
    setup::WINDOW_SIZE,
    ui::{
        CategoriesSubState, app_button,
        categories::{self, AppsCategoriesList, CategoriesList},
        category_expanded::CategoryPopupFor,
    },
};
use utils::prelude::*;

pub fn categories_list(
    categories: HashMap<String, Vec<DesktopApp>>,
    curve_image: Handle<Image>,
    primary_500: Handle<Font>,
) -> impl Bundle {
    let mut ordered_categories = Vec::new();

    // 1. Push Webbrowsers and Development first if they exist
    if let Some(apps) = categories.get("WebBrowser").cloned() {
        ordered_categories.push(("WebBrowser".to_string(), apps));
    }
    if let Some(apps) = categories.get("Development").cloned() {
        ordered_categories.push(("Development".to_string(), apps));
    }

    // 2. Push remaining categories
    let mut names = categories.keys().into_iter().collect::<Vec<_>>();
    names.sort();

    for name in names {
        if name != "WebBrowser" && name != "Development" {
            if let Some(apps) = categories.get(name) {
                ordered_categories.push((name.to_string(), apps.clone()));
            }
        }
    }

    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(WINDOW_SIZE.1 - 56.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                top: Val::Px(8.),
                left: Val::Px(16.),
                right: Val::Px(16.),
                bottom: Val::Px(8.),
            },
            row_gap: Val::Px(16.),
            overflow: Overflow::scroll_y(),
            ..Default::default()
        },
        CoreScrollArea,
        ScrollPosition {
            offset_x: 0.0,
            offset_y: 0.0,
        },
        CategoriesList,
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for (name, apps) in ordered_categories {
                let mut commands = parent.world_mut().commands();
                let name_cloned = name.clone();

                let on_click = commands.register_system(
                    move |mut state: ResMut<NextState<CategoriesSubState>>,
                          mut commands: Commands| {
                        commands.insert_resource(CategoryPopupFor(name_cloned.clone()));
                        state.set(CategoriesSubState::Popup);
                        println!("clicked {:?}", name_cloned.clone());
                    },
                );
                parent.spawn(category(
                    name,
                    apps,
                    on_click,
                    curve_image.clone(),
                    primary_500.clone(),
                ));
            }
        })),
    )
}

pub fn category(
    name: String,
    apps: Vec<DesktopApp>,
    on_click: SystemId,
    curve_image: Handle<Image>,
    primary_500: Handle<Font>,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            min_height: Val::Px(142.),
            padding: UiRect {
                left: Val::Px(22.),
                right: Val::Px(22.),
                top: Val::Px(16.),
                ..default()
            },
            ..Default::default()
        },
        CoreButton::new().on_click(on_click),
        BorderRadius::all(Val::Px(12.)),
        BackgroundColor(Color::oklch(0.209, 0., 0.)),
        children![
            (
                Node {
                    display: Display::Grid,
                    width: Val::Px(464.),
                    height: Val::Px(80.),
                    grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                    column_gap: Val::Px(48.0),
                    ..Default::default()
                },
                Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
                    for app in apps.iter().take(4) {
                        parent.spawn(app_button(app));
                    }
                }))
            ),
            (
                ImageNode::new(curve_image),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(38.),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    align_items: AlignItems::Center,
                    padding: UiRect::left(Val::Px(16.)),
                    ..default()
                },
                children![(
                    Text::new(name.chars().into_iter().take(9).collect::<String>()),
                    TextFont {
                        font: primary_500,
                        font_size: 16.,
                        ..default()
                    },
                    TextColor(Color::oklch(0.5761, 0., 0.))
                )]
            )
        ],
    )
}
