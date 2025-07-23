use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::SystemId},
    prelude::*,
};
use bevy_core_widgets::{CoreButton, CoreScrollArea, InteractionDisabled, Orientation};

use crate::desktop_apps::{self, DesktopApp, DesktopApps};

#[derive(Debug, Component)]
pub struct AppsCategoriesList;

#[derive(Debug, Component)]
pub struct AppsCategoriesListParent;

pub fn apps_grid() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        AppsCategoriesListParent,
        children![(
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.)),
                row_gap: Val::Px(16.),
                overflow: Overflow::scroll_y(),
                ..Default::default()
            },
            CoreScrollArea,
            ScrollPosition {
                offset_x: 0.0,
                offset_y: 0.0,
            },
            AppsCategoriesList,
        ),],
    )
}

pub fn category(
    name: String,
    apps: Vec<DesktopApp>,
    on_group_click: Option<SystemId>,
) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(132.),
            padding: UiRect {
                left: Val::Px(28.),
                right: Val::Px(28.),
                top: Val::Px(20.),
                bottom: Val::Px(12.),
            },
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(14.),
            ..Default::default()
        },
        BorderRadius::all(Val::Px(12.)),
        BackgroundColor(Color::oklcha(0.2221, 0., 0., 0.95)),
        children![
            (
                Node {
                    display: Display::Grid,
                    width: Val::Percent(100.),
                    height: Val::Px(68.),
                    grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                    column_gap: Val::Px(28.0),
                    ..Default::default()
                },
                Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
                    //if apps has length > 4 then
                    //spawn app card for frist 4 apps
                    //and then spawn app group card

                    for app in apps.iter().take(4) {
                        parent.spawn(app_card(app));
                    }
                    if apps.len() > 4 {
                        parent.spawn(app_group_card(
                            apps.into_iter().skip(4).collect(),
                            on_group_click,
                        ));
                    }
                }))
            ),
            (
                Text::new(name),
                TextFont {
                    font_size: 16.,
                    ..Default::default()
                },
                TextColor(Color::oklch(0.7572, 0., 0.))
            )
        ],
    )
}

fn app_card(app: &DesktopApp) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            padding: UiRect::all(Val::Px(8.)),
            ..Default::default()
        },
        children![(ImageNode::new(app.icon.clone()),)],
        CoreButton {
            on_click: Some(app.on_click),
        },
        BorderRadius::all(Val::Px(11.82)),
        BackgroundColor(Color::oklch(0.2891, 0., 0.)),
    )
}

fn app_group_card(apps: Vec<DesktopApp>, on_click: Option<SystemId>) -> impl Bundle {
    (
        Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
            column_gap: Val::Px(5.1),
            row_gap: Val::Px(3.1),
            padding: UiRect::all(Val::Px(5.5)),
            border: UiRect::all(Val::Px(0.85)),
            ..Default::default()
        },
        BackgroundColor(Color::oklch(0.2221, 0., 0.)),
        BorderColor(Color::oklch(0.2891, 0., 0.)),
        BorderRadius::all(Val::Px(11.82)),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            let cloned_apps = apps.clone();
            for app in apps.into_iter().take(9) {
                parent.spawn(small_app_card(app));
            }
        })),
        CoreButton { on_click },
    )
}

fn small_app_card(app: DesktopApp) -> impl Bundle {
    (
        ImageNode::new(app.icon.clone()),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        BorderRadius::all(Val::Px(2.36)),
    )
}

fn popup_apps_group(apps: Vec<DesktopApp>, on_popup_click: SystemId) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            position_type: PositionType::Absolute,

            padding: UiRect {
                left: Val::Px(0.),
                right: Val::Px(0.),
                top: Val::Px(30.),
                bottom: Val::Px(28.),
            },
            overflow: Overflow::scroll_y(),
            ..Default::default()
        },
        ZIndex(999),
        ScrollPosition {
            offset_x: 0.0,
            offset_y: 0.0,
        },
        CoreScrollArea,
        BorderRadius::all(Val::Px(12.)),
        BackgroundColor(Color::oklcha(0.3867, 0., 0., 0.95)),
        CoreButton {
            on_click: Some(on_popup_click),
        },
        children![(
            Node {
                display: Display::Grid,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                column_gap: Val::Px(4.),
                row_gap: Val::Px(37.),
                ..Default::default()
            },
            Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
                for app in apps {
                    parent.spawn(app_group_app(app));
                }
            }))
        ),],
    )
}

fn app_group_app(app: DesktopApp) -> impl Bundle {
    (
        Node {
            width: Val::Px(124.),
            height: Val::Px(126.),
            padding: UiRect {
                left: Val::Px(0.),
                right: Val::Px(0.),
                top: Val::Px(8.),
                bottom: Val::Px(3.),
            },
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
                    padding: UiRect::all(Val::Px(8.)),
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(13.91)),
                BackgroundColor(Color::oklch(0.2891, 0., 0.)),
                CoreButton {
                    on_click: Some(app.on_click),
                },
                children![ImageNode::new(app.icon.clone()),]
            ),
            (
                Text::new(app.name.clone()),
                TextFont {
                    font_size: 16.,
                    ..Default::default()
                },
            ),
        ],
    )
}

#[derive(Debug, Component)]
pub struct AppsList;

pub fn app_list(desktop_apps: Vec<DesktopApp>) -> impl Bundle {
    let desktop_apps = desktop_apps.clone();
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(16.)),
            row_gap: Val::Px(4.),
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
                parent.spawn(app_list_app(&app));
            }
        })),
    )
}

fn app_list_app(app: &DesktopApp) -> impl Bundle {
    (
        Node {
            width: Val::Percent(90.),
            padding: UiRect::all(Val::Px(8.)),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.),
            ..Default::default()
        },
        CoreButton {
            on_click: Some(app.on_click),
        },
        children![
            (
                Node {
                    width: Val::Px(44.),
                    height: Val::Px(44.),
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(7.65)),
                ImageNode::new(app.icon.clone()),
            ),
            (
                Text::new(app.name.clone()),
                TextFont {
                    font_size: 16.,
                    ..Default::default()
                },
            )
        ],
    )
}

pub fn update_apps_list(
    mut commands: Commands,
    mut query: Query<Entity, With<AppsList>>,
    desktop_apps: Res<DesktopApps>,
) {
    if let Ok(parent) = query.single_mut() {
        println!("Updating apps list with {} apps", desktop_apps.apps.len());
        commands.entity(parent).despawn_related::<Children>();
        commands.entity(parent).with_children(|parent| {
            for app in desktop_apps.apps.iter() {
                let exec = app.exec.clone();
                let commands = parent.commands_mut();
                let on_click = commands.register_system(move || {
                    let exec = exec.clone();
                    let _ = DesktopApps::run_app_exec(exec);
                });
                parent.spawn(app_list_app(app));
            }
        });
    }
}

pub fn update_apps_categories(
    mut commands: Commands,
    mut query: Query<Entity, With<AppsCategoriesList>>,
    mut query_parent: Query<Entity, With<AppsCategoriesListParent>>,
    desktop_apps: Res<DesktopApps>,
) {
    let Ok(parent) = query.single_mut() else {
        return;
    };

    let Ok(parent_parent) = query_parent.single_mut() else {
        return;
    };

    commands.entity(parent).despawn_related::<Children>();

    let on_text_input_click = commands.register_system(
        move |mut commands: Commands,
              mut query_parent: Query<Entity, With<AppsCategoriesListParent>>,
              desktop_apps: Res<DesktopApps>| {
            let Ok(parent_parent) = query_parent.single_mut() else {
                return;
            };
            let app_list_popup = commands.spawn_empty().id();
            commands
                .entity(app_list_popup)
                .insert(app_list(desktop_apps.apps.clone()));
            commands.entity(parent_parent).add_child(app_list_popup);
        },
    );

    let on_cancel_click = commands.register_system(
        move |mut commands: Commands, mut query_apps_list: Query<Entity, With<AppsList>>| {
            if let Ok(entity) = query_apps_list.single_mut() {
                commands.entity(entity).despawn();
            }
        },
    );

    let search_entity = commands.spawn_empty().id();
    commands
        .entity(search_entity)
        .insert(search_box(on_text_input_click, on_cancel_click));

    commands
        .entity(parent_parent)
        .insert_children(1, &[search_entity]);

    let categories = desktop_apps.get_apps_by_categories();

    for (catgory, apps) in categories.into_iter() {
        commands.entity(parent).with_children(|parent| {
            let cloned_apps = apps.clone();
            let commands = parent.commands_mut();
            let on_group_click = commands.register_system(move |mut commands: Commands| {
                // Spawn the popup entity and get its id
                let popup_entity_id = commands.spawn_empty().id();
                // Register a system that despawns the popup entity by id
                let on_popup_click = {
                    let popup_entity_id = popup_entity_id;
                    commands.register_system(move |mut commands: Commands| {
                        println!("on poupup click");
                        commands.entity(popup_entity_id).despawn();
                    })
                };
                // Insert the popup_apps_group component into the popup entity
                commands.entity(popup_entity_id).insert(popup_apps_group(
                    cloned_apps.clone().into_iter().skip(4).collect(),
                    on_popup_click,
                ));
                // Insert the popup entity into the parent entity
                commands.entity(parent_parent).add_child(popup_entity_id);
            });

            parent.spawn(category(catgory, apps.clone(), Some(on_group_click)));
        });
    }
}

fn search_box(on_text_input_click: SystemId, on_cancel: SystemId) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Px(52.),
            column_gap: Val::Px(10.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.),
            ..Default::default()
        },
        ZIndex(99),
        children![
            (
                Node {
                    width: Val::Percent(70.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(10.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                CoreButton {
                    on_click: Some(on_text_input_click)
                },
                BorderRadius::all(Val::Px(32.),),
                BackgroundColor(Color::oklcha(0.3211, 0., 0., 0.95)),
                children![(
                    Text::new("Just type"),
                    TextColor(Color::oklch(0.5999, 0., 0.,)),
                )]
            ),
            (
                Text::new("Cancel"),
                CoreButton {
                    on_click: Some(on_cancel)
                }
            )
        ],
    )
}
