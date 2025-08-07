use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId},
    prelude::*,
};
use bevy_core_widgets::CoreButton;

use crate::{
    desktop_apps::{self, DesktopApp, DesktopApps},
    launcher::SearchWindow,
    utils::{FontAssets, Icon},
};

#[derive(Debug, Resource, Clone)]
pub struct FrequentlyUsedApps(pub Vec<String>);

#[derive(Debug, Resource)]
pub struct RecentSearches(pub Vec<String>);

#[derive(Debug, Component)]
pub struct SearchInput;

#[derive(Debug, Component)]
pub struct UniversalSearch;

#[derive(Debug, Component)]
pub struct SearchResultsComponent;

#[derive(Debug, Resource)]
pub struct SearchText(pub String);

pub fn universal_search(
    freq_used_apps: Vec<DesktopApp>,
    recent_searches: Vec<String>,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(16.)),
            ..Default::default()
        },
        UniversalSearch,
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            let mut commands = parent.world_mut().commands();
            let on_search_click = commands.register_system(move |mut commands: Commands| {});
            let on_cancel_click = commands.register_system(
                |mut commands: Commands, q_search: Option<Single<Entity, With<SearchWindow>>>| {
                    if let Some(entity) = q_search {
                        commands.entity(entity.into_inner()).despawn();
                    }
                },
            );

            let on_clear_click = commands.register_system(
                |mut commands: Commands,
                q_search: Query<Entity, With<SearchResultsComponent>>,
                 mut q_input: Query<(&mut Text, &mut TextColor), With<SearchInput>>| {
                    for (mut text, mut text_color) in q_input.iter_mut() {
                        text.0 = "Just type...".to_string();
                        text_color.0 = Color::oklch(0.5999, 0., 0.);
                    }
                    for entity in q_search.iter() {
                        commands.entity(entity).despawn();
                    }
                },
            );

            parent.spawn(search_box(on_search_click, on_cancel_click, on_clear_click));
            parent.spawn(frequently_used_apps(freq_used_apps.clone()));
            parent.spawn(search_items(recent_searches));
        })),
    )
}

fn frequently_used_apps(freq_used_apps: Vec<DesktopApp>) -> impl Bundle {
    (
        Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Px(94.),
            grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
            column_gap: Val::Px(32.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect {
                left: Val::Px(30.),
                right: Val::Px(30.),
                top: Val::Px(14.),
                bottom: Val::Px(16.),
            },
            ..Default::default()
        },
        BorderRadius::all(Val::Px(12.)),
        BackgroundColor(Color::oklcha(0.2221, 0., 0., 0.95)),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for app in freq_used_apps.into_iter().take(5) {
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
        CoreButton {
            on_click: Some(app.on_click),
            on_long_press: None,
        },
        BorderRadius::all(Val::Px(11.82)),
        BackgroundColor(Color::oklch(0.2891, 0., 0.)),
    )
}

fn search_items(recent_searches: Vec<String>) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(200.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            overflow: Overflow::scroll_y(),
            ..Default::default()
        },
        ScrollPosition {
            offset_x: 0.0,
            offset_y: 0.0,
        },
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for item in recent_searches.into_iter().take(5) {
                let mut commands = parent.world_mut().commands();
                let cloned_item = item.clone();
                let on_click =
                    commands.register_system(
                        move |mut commands: Commands,
                              desktop_apps: Res<DesktopApps>,
                              mut q_universal_search: Query<Entity, With<UniversalSearch>>,
                              mut q_input: Query<
                            (&mut Text, &mut TextColor),
                            With<SearchInput>,
                        >| {
                            commands.insert_resource(SearchText(cloned_item.clone()));
                            for (mut text, mut text_color) in q_input.iter_mut() {
                                text.0 = cloned_item.clone();
                                text_color.0 = Color::WHITE;
                            }

                            for universal_search in q_universal_search.iter_mut() {
                                let mut browser_apps: Vec<SearchResult> = vec![];
                                let mut results: Vec<SearchResult> = vec![];

                                desktop_apps.apps.clone().into_iter().for_each(|app| {
                                    if ["firefox_firefox", "microsoft-edge"]
                                        .contains(&app.app_id.as_str())
                                    {
                                        let cloned_exec = app.exec.clone();
                                        let cloned_item2 = cloned_item.clone();
                                        let on_click = commands.register_system(move || {
                                            let url = format!(
                                                "https://www.google.com/search?q={}",
                                                cloned_item2.clone()
                                            );
                                            let cmd = cloned_exec.to_string()
                                                + " --new-tab "
                                                + url.as_str();
                                            info!("Running command: {}", cmd);
                                            let _ = DesktopApps::run_app_exec(cmd);
                                        });
                                        browser_apps.push(SearchResult {
                                            name: format!(
                                                "Search {} on {}",
                                                cloned_item.clone(),
                                                app.name.clone()
                                            ),
                                            icon: app.icon.clone(),
                                            on_click: on_click,
                                            _type: SearchResultType::App,
                                        });
                                    }

                                    if app.name.contains(cloned_item.as_str()) {
                                        results.push(SearchResult {
                                            name: app.name.clone(),
                                            icon: app.icon.clone(),
                                            on_click: app.on_click,
                                            _type: SearchResultType::File,
                                        });
                                    }
                                });

                                let entity = commands.spawn_empty().id();
                                results.append(&mut browser_apps);
                                let sr =
                                    commands.entity(entity).insert(search_results(results)).id();

                                commands.entity(universal_search).add_child(sr);
                            }
                        },
                    );
                parent.spawn((search_item(&item, on_click),));
            }
        })),
    )
}

fn search_item(txt: &String, on_click: SystemId) -> impl Bundle {
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
                CoreButton {
                    on_click: Some(on_click),
                    on_long_press: None,
                },
                children![
                    (
                        Text::new(Icon::Search),
                        TextFont {
                            font_size: 16.,
                            ..Default::default()
                        },
                    ),
                    (
                        Node {
                            flex_grow: 1.,
                            ..Default::default()
                        },
                        Text::new(txt.clone()),
                        TextFont {
                            font_size: 16.,
                            ..Default::default()
                        },
                        TextColor(Color::oklch(0.9551, 0., 0.)),
                    ),
                    (
                        Text::new(Icon::ArrowUpRight),
                        TextFont {
                            font_size: 16.,
                            ..Default::default()
                        },
                    ),
                ],
            ),
            separator()
        ],
    )
}

fn separator() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(2.),
            ..Default::default()
        },
        BackgroundColor(Color::oklch(0.2987, 0., 0.)),
    )
}

fn search_box(
    on_search_click: SystemId,
    on_cancel_click: SystemId,
    on_clear_click: SystemId,
) -> impl Bundle {
    (
        Node {
            left: Val::Px(0.),
            bottom: Val::Px(0.),
            width: Val::Percent(100.),
            height: Val::Px(71.),
            column_gap: Val::Px(10.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            padding: UiRect::horizontal(Val::Px(16.)),
            ..Default::default()
        },
        ZIndex(999),
        BackgroundColor(Color::oklch(0.2308, 0., 0.)),
        children![
            (
                Node {
                    flex_grow: 1.,
                    height: Val::Px(52.),
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(12.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CoreButton {
                    on_click: Some(on_search_click),
                    on_long_press: None,
                },
                BorderRadius::all(Val::Px(32.),),
                BackgroundColor(Color::oklcha(0.3211, 0., 0., 0.95)),
                children![
                    (
                        Node {
                            justify_self: JustifySelf::Center,
                            margin: UiRect::right(Val::Px(12.)),
                            ..Default::default()
                        },
                        Text::new(Icon::Search),
                        TextFont {
                            font_size: 24.,
                            ..Default::default()
                        },
                        TextColor(Color::oklch(0.5999, 0., 0.))
                    ),
                    (
                        Text::new("Just type..."),
                        TextColor(Color::oklch(0.5999, 0., 0.,)),
                        Node {
                            justify_self: JustifySelf::Center,
                            flex_grow: 1.,
                            ..Default::default()
                        },
                        SearchInput
                    ),
                    (
                        Node {
                            justify_self: JustifySelf::Center,
                            margin: UiRect::right(Val::Px(12.)),
                            ..Default::default()
                        },
                        Text::new(Icon::XCircle),
                        TextFont {
                            font_size: 24.,
                            ..Default::default()
                        },
                        TextColor(Color::oklch(0.5999, 0., 0.)),
                        CoreButton {
                            on_click: Some(on_clear_click),
                            on_long_press: None,
                        },
                    ),
                ]
            ),
            (
                Text::new("Cancel"),
                CoreButton {
                    on_click: Some(on_cancel_click),
                    on_long_press: None,
                }
            )
        ],
    )
}

fn search_results(results: Vec<SearchResult>) -> impl Bundle {
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
            ..Default::default()
        },
        SearchResultsComponent,
        BackgroundColor(Color::oklch(0.173, 0., 0.)),
        ZIndex(99),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for result in results {
                parent.spawn(search_result_item(result));
            }
        })),
    )
}

#[derive(Debug, Clone)]
pub enum SearchResultType {
    App,
    File,
    Action,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub icon: Handle<Image>,
    pub on_click: SystemId,
    pub _type: SearchResultType,
}

fn search_result_item(result: SearchResult) -> impl Bundle {
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
                CoreButton {
                    on_click: Some(result.on_click),
                    on_long_press: None,
                },
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
                        Text::new(match result._type {
                            SearchResultType::App => Icon::ArrowSquareUp,
                            SearchResultType::File => Icon::ArrowUpRight,
                            SearchResultType::Action => Icon::ArrowUpRight,
                        }),
                        TextFont {
                            font_size: 16.,
                            ..Default::default()
                        },
                        TextColor(Color::oklch(0.5999, 0., 0.)),
                    ));
                }))
            ),
            separator(),
        ],
    )
}
