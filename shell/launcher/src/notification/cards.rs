use bevy::prelude::*;
use bevy_smithay::{
    SmithayPlugin,
    SmithayWindowType,
    prelude::{ layer_shell::LayerShellSettings, subsurface::Anchor },
};
use bevy_core_widgets::{ CoreButton, CoreScrollArea, InteractionDisabled, Orientation };

use bevy_plugins::notification::{ NotificationPlugin, NotificationEvent };
use crate::components::apps_grid;
use crate::launcher::HomescreenWindow;
use crate::launcher::spawn_camera;
pub struct NotificationWindow;

pub struct NotificationWindowPlugin;

impl Plugin for NotificationWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_count)
            .add_systems(Update, spawn_multiple_cards)
            .add_systems(Update, process_notifications);
    }
}

#[derive(Component)]
pub struct ClearAllButton;

pub fn spawn_notification_window(commands: &mut Commands) {
    let camera = spawn_camera(
        commands,
        540,
        531,
        "Home screen".to_string(),
        LayerShellSettings {
            layer: bevy_smithay::prelude::subsurface::Layer::Bottom,
            anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
            exclusive_zone: 0,
            ..default()
        },
        HomescreenWindow
    );

    let notification_surface_entity = commands
        .spawn((
            UiTargetCamera(camera),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center, // <-- Stretch children
                justify_content: JustifyContent::FlexStart, // <-- Start at top
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(16.0),
                overflow: Overflow::scroll_y(), // <-- Enable vertical scrolling
                ..Default::default()
            },
            CoreScrollArea,
            ScrollPosition {
                offset_x: 0.0,
                offset_y: 0.0,
            },
            BackgroundColor(Color::BLACK),
            NotificationSurface,
            children![(
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    min_height: Val::Px(30.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    bottom: Val::Px(15.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Start,
                    ..default()
                },
                BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
                BorderRadius::all(Val::Px(8.0)),
                children![
                    (
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(30.0),
                            margin: UiRect::right(Val::Px(200.0)),
                            ..default()
                        },
                        children![(
                            Text::new("Notifications"),
                            TextFont { font_size: 24.0, ..default() },
                            TextColor(Color::WHITE),
                        )],
                    ),
                    (
                        Node {
                            width: Val::Px(78.0),
                            height: Val::Px(30.0),
                            // top: Val::Px(8.0),
                            justify_content: JustifyContent::Center, // <-- Center horizontally
                            align_items: AlignItems::Center,
                            margin: UiRect::right(Val::Px(2.0)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(8.0)),
                        ClearAllButton,
                        BackgroundColor(Color::srgb(77.0, 77.0, 77.0)),
                        children![(
                            Text::new("Clear all"),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(Color::BLACK),
                        )],
                    )
                ],
            )],
        ))
        .id();

    commands.insert_resource(NotificationSurfaceEntity(notification_surface_entity));
}

#[derive(Component)]
pub struct NotificationSurface;

#[derive(Resource)]
pub struct NotificationSurfaceEntity(Entity);

#[derive(Resource, Default)]
pub struct Count(pub i32);

pub fn init_count(mut commands: Commands) {
    commands.insert_resource(Count(0));
}

pub fn spawn_multiple_cards(
    mut commands: Commands,
    drawing_surface: Option<Res<NotificationSurfaceEntity>>,
    asset_server: Res<AssetServer>
) {
    let icon_handle = asset_server.load(format!("icons/{}.png", 1));
    if let Some(ref surface) = drawing_surface {
        create_card(&mut commands, surface.0, icon_handle)
    }
}

pub fn create_card(commands: &mut Commands, drawing_surface: Entity, icon_handle: Handle<Image>) {
    let card = commands.entity(drawing_surface).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(508.0),
                min_height: Val::Px(81.0),
                margin: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(16.0)),
                bottom: Val::Percent(1 as f32),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
            BorderRadius::all(Val::Px(8.0)),
            BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
            // ZIndex(1 as i32),
            // Card { index },
            children![
                (
                    Node {
                        width: Val::Auto,
                        height: Val::Px(24.0),
                        min_height: Val::Px(24.0),
                        margin: UiRect::right(Val::Px(12.0)),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    children![
                        (
                            ImageNode::new(icon_handle),
                            Node {
                                width: Val::Px(24.0),
                                height: Val::Auto,
                                ..default()
                            },
                        ),
                        (
                            Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                justify_content: JustifyContent::Center, // <-- Center horizontally
                                align_items: AlignItems::Center,
                                margin: UiRect::left(Val::Px(10.0)),
                                ..default()
                            },
                            children![(
                                Text::new("app_name"),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::WHITE),
                            )],
                        )
                    ],
                ),
                (
                    Node {
                        width: Val::Px(484.0),
                        height: Val::Px(15.0),
                        top: Val::Px(8.0),
                        margin: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    children![(
                        Text::new("Content of notification goes here"),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::WHITE),
                    )],
                )
            ],
        ));
    });
}

fn process_notifications(
    mut events: EventReader<NotificationEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            NotificationEvent::Recieved(id, notification) => {
                info!(
                    "Notification received: id={:?}, application={:?}, expires={:?}",
                    &id,
                    &notification.app_name,
                    notification.get_expire_timeout()
                );
            }
            NotificationEvent::Closed(id) => {
                info!("Notification closed: id={}", id);
                // remove_notification_card(&mut commands, id, &mut notification_storage);
            }
        }
    }
}
