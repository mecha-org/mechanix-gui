use bevy::color::palettes::basic::RED;
use bevy::{ prelude::*, winit::WinitSettings };
use bevy_plugins::notification::{ NotificationPlugin, NotificationEvent };
use freedesktop_notifications_server::notification::Notification;
use tokio::time::Duration;
use std::collections::HashMap;
use std::time::Instant;
// Resource to store active notifications
#[derive(Resource, Default)]
struct NotificationStorage {
    notifications: HashMap<u32, Notification>,
}

// #[derive(Clone)]
// struct NotificationData {
//     id: u32,
//     title: String,
//     message: String,
//     created_at: std::time::Instant,
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NotificationPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<NotificationStorage>()
        .add_systems(Startup, (setup, setup_notification_container))
        .add_systems(Update, (
            process_notifications,
            handle_dismiss_buttons,
            auto_dismiss_notifications,
        ))
        .run();
}

#[derive(Component)]
struct NotificationCard {
    id: u32,
    created_at: std::time::Instant,
}
// Component to mark the notification container
#[derive(Component)]
struct NotificationContainer;

#[derive(Component)]
struct DismissButton {
    notification_id: u32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_notification_container(mut commands: Commands) {
    // Main notification container - positioned at top right
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            width: Val::Px(350.0),
            ..default()
        },
        NotificationContainer,
    ));
}

fn create_notification_card(
    commands: &mut Commands,
    parent: Entity,
    notification_id: u32,
    notification: Notification,
    asset_server: &Res<AssetServer>,
    created_time: Instant
) {
    commands.entity(parent).with_children(|parent| {
        parent
            .spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    min_height: Val::Px(20.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
                BorderRadius::all(Val::Px(8.0)),
                NotificationCard {
                    id: notification_id.clone(),
                    created_at: created_time,
                },
            ))
            .with_children(|card| {
                // Notification content (left side)
                card.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        margin: UiRect::right(Val::Px(5.0)),
                        ..default()
                    },
                )).with_children(|content| {
                    // Title
                    content.spawn((
                        Text::new(&notification.summary),
                        TextFont {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"), // Use your font
                            font_size: 8.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(4.0)),
                            ..default()
                        },
                    ));

                    // Message
                    content.spawn((
                        Text::new(&notification.body),
                        TextFont {
                            // font: asset_server.load("fonts/FiraSans-Regular.ttf"), // Use your font
                            font_size: 8.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                    ));
                });

                // Dismiss button (right side)
                card.spawn((
                    Button,
                    Node {
                        width: Val::Px(10.0),
                        height: Val::Px(10.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 0.8)),
                    BorderRadius::all(Val::Px(12.0)),
                    DismissButton {
                        notification_id: notification_id,
                    },
                )).with_children(|button| {
                    button.spawn((
                        Text::new("x"),
                        TextFont {
                            font_size: 8.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            });
    });
}

fn process_notifications(
    mut events: EventReader<NotificationEvent>,
    mut commands: Commands,
    mut notification_storage: ResMut<NotificationStorage>,
    container_query: Query<Entity, With<NotificationContainer>>,
    asset_server: Res<AssetServer>
) {
    let container = match container_query.get_single() {
        Ok(entity) => entity,
        Err(_) => {
            return;
        }
    };

    for event in events.read() {
        match event {
            NotificationEvent::Recieved(id, notification) => {
                info!("Notification received: id={:?}, notification={:?}", &id, &notification);
                notification_storage.notifications.insert(id.clone(), notification.clone());

                let current_time = std::time::Instant::now();
                // Create the notification card
                create_notification_card(
                    &mut commands,
                    container,
                    id.clone(),
                    notification.clone(),
                    &asset_server,
                    current_time
                );
            }
            NotificationEvent::Closed(id) => {
                info!("Notification closed: id={}", id);
                // remove_notification_card(&mut commands, id, &mut notification_storage);
            }
        }
    }
}

fn handle_dismiss_buttons(
    mut interaction_query: Query<
        (&Interaction, &DismissButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut commands: Commands,
    mut notification_storage: ResMut<NotificationStorage>,
    card_query: Query<(Entity, &NotificationCard)>
) {
    for (interaction, dismiss_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Remove from storage
            notification_storage.notifications.remove(&dismiss_button.notification_id);

            // Find and despawn the notification card
            for (entity, card) in &card_query {
                if card.id == dismiss_button.notification_id {
                    commands.entity(entity).despawn_recursive();
                    break;
                }
            }
        }
    }
}

fn auto_dismiss_notifications(
    mut commands: Commands,
    mut notification_storage: ResMut<NotificationStorage>,
    card_query: Query<(Entity, &NotificationCard)>,
    time: Res<Time>
) {
    let current_time = std::time::Instant::now();
    let mut to_remove = Vec::new();

    for (entity, card) in &card_query {
        if current_time.duration_since(card.created_at) > Duration::from_secs(10) {
            to_remove.push((entity, card.id.clone()));
        }
    }

    for (entity, id) in to_remove {
        notification_storage.notifications.remove(&id);
        commands.entity(entity).despawn_recursive();
    }
}

fn print_notifications(mut events: EventReader<NotificationEvent>) {
    for event in events.read() {
        match event {
            NotificationEvent::Recieved(id, notification) => {
                info!("Notification received: id={:?}, notification={:?}", id, notification);
            }
            NotificationEvent::Closed(id) => {
                info!("Notification closed: id={}", id);
            }
        }
    }
}
