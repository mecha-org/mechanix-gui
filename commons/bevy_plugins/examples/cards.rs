use bevy::{ prelude::*, winit::WinitSettings };
use std::time::Duration;
use std::thread::sleep;
use freedesktop_notifications_server::notification::Notification;
use std::collections::HashMap;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup_container)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, spawn_multiple_cards)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Spawn the UI camera so our UI elements are visible.
    commands.spawn(Camera2d);
}

#[derive(Component)]
pub struct Container;

#[derive(Resource)]
pub struct ContainerEntity(pub Entity);

pub fn setup_container(mut commands: Commands) {
    // Spawn a container entity with the Container component.
    let container_entity = commands
        .spawn((
            Container,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();
    commands.insert_resource(ContainerEntity(container_entity));
}

fn spawn_multiple_cards(mut commands: Commands, container: Res<ContainerEntity>) {
    let mut notifications = HashMap::new();
    notifications.insert(1, Notification {
        app_name: "Files".to_string(),
        replaces_id: 0,
        app_icon: "folder".to_string(),
        summary: "Files".to_string(),
        body: "Content of notification goes here, maximum length of 484px".to_string(),
        actions: vec![],
        hints: HashMap::new(),
        expire_timeout: 0,
    });
    notifications.insert(2, Notification {
        app_name: "Files".to_string(),
        replaces_id: 0,
        app_icon: "folder".to_string(),
        summary: "Files".to_string(),
        body: "Another notification for Files app".to_string(),
        hints: HashMap::new(),
        actions: vec![],
        expire_timeout: 0,
    });
    notifications.insert(3, Notification {
        app_name: "Things".to_string(),
        replaces_id: 0,
        app_icon: "folder".to_string(),
        summary: "Things".to_string(),
        body: "Another notification for Things app".to_string(),
        hints: HashMap::new(),
        actions: vec![],
        expire_timeout: 0,
    });
    for (id, notification) in notifications.values().enumerate(){
        //     sleep(Duration::from_secs(3));
        spawn_card(&mut commands, ContainerEntity(container.0), notification.clone(), id as u32);
    }
}

#[derive(Component)]
pub struct Card;

fn spawn_card(
    commands: &mut Commands, // Automatically provided by Bevy: allows entity spawning
    container: ContainerEntity, // Automatically provided Bevy resource with container entity
    notification: Notification,
    id: u32
) {
    commands.entity(container.0).with_children(|parent| {
        parent.spawn(build_card(notification, id));
    });
}

fn notification_icon(icon_handle: Handle<Image>) -> impl Bundle {
    (
        ImageNode::new(icon_handle),
        Node {
            width: Val::Px(24.0),
            height: Val::Px(24.0),
            margin: UiRect::right(Val::Px(12.0)),
            ..default()
        },
    )
}

fn build_card(notification: Notification, id: u32) -> impl Bundle {
    (
        Node {
            width: Val::Px(484.0),
            min_height: Val::Px(64.0),
            margin: UiRect::all(Val::Px(8.0)),
            padding: UiRect::all(Val::Px(16.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            position_type: PositionType::Absolute,
            bottom: Val::Percent((id as f32) * 6.0),
            ..default()
        },
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(12.0)),
        BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
        ZIndex(id as i32),
        Card,
        children![
            // Title row: icon, summary, time (mocked as '3h')
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    // Icon placeholder (could be replaced with ImageNode)
                    (
                        Node {
                            width: Val::Px(24.0),
                            height: Val::Px(24.0),
                            margin: UiRect::right(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.7, 0.2)),
                    ),
                    (
                        Text::new(&notification.summary),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new("   ·   3h"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    )
                ],
            ),
            // Body/message
            (
                Node {
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
                children![(
                    Text::new(&notification.body),
                    TextFont { font_size: 15.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            )
        ],
    )
}
