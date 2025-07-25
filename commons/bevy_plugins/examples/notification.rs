use std::path::PathBuf;
use bevy::color::palettes::basic::RED;
use bevy::{ prelude::*, winit::WinitSettings };
use bevy_plugins::notification::{ NotificationPlugin, NotificationEvent };
use freedesktop_notifications_server::notification::Notification;
use tokio::time::Duration;
use std::collections::HashMap;
use std::time::Instant;
// Resource to store active notifications
#[derive(Resource, Default)]
struct AllNotifications {
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
        .init_resource::<AllNotifications>()
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

// --- Bundle functions for notification card UI ---

/// Generates the icon bundle for a notification card.
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

/// Generates the title row bundle: app name, dot, timestamp.
fn notification_title_row(summary: &str, created_time: Instant) -> impl Bundle {
    let elapsed = created_time.elapsed().as_secs();
    let timestamp = if elapsed >= 3600 {
        format!("{}h", elapsed / 3600)
    } else if elapsed >= 60 {
        format!("{}m", elapsed / 60)
    } else {
        format!("{}s", elapsed)
    };

    (
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new(summary),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::WHITE),
            ),
            (
                Text::new(" • "),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
            ),
            (
                Text::new(timestamp),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
            )
        ]
    )
}

/// Generates the message bundle for a notification card.
fn notification_message(body: &str) -> impl Bundle {
    let mut truncated = body.to_string();
    if truncated.len() > 80 {
        truncated.truncate(77);
        truncated.push_str("...");
    }
    (
        Text::new(truncated),
        TextFont { font_size: 12.0, ..default() },
        TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
        Node { margin: UiRect::top(Val::Px(4.0)), ..default() },
    )
}

/// Generates the dismiss button bundle for a notification card.
/// Interactive: clicking this button will dismiss the notification.
fn notification_dismiss_button(notification_id: u32) -> impl Bundle {
    (
        Button,
        Node {
            margin: UiRect::left(Val::Px(8.0)),
            ..default()
        },
        DismissButton { notification_id },
        // Interactive: clicking this button will dismiss the notification
        children![(
            Text::new("Dismiss"),
            TextFont { font_size: 11.0, ..default() },
            TextColor(Color::srgb(0.9, 0.2, 0.2)), // Custom red color
        )]
    )
}

/// Generates the notification card bundle, composed of sub-bundles.
fn notification_card_bundle(
    notification_id: u32,
    notification: &Notification,
    icon_handle: Handle<Image>,
    created_time: Instant,
) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Start,
            width: Val::Px(484.0),
            min_height: Val::Px(48.0),
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.13, 0.13, 0.13, 0.98)),
        BorderRadius::all(Val::Px(8.0)),
        NotificationCard { id: notification_id, created_at: created_time },
        children![
            notification_icon(icon_handle.clone()),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    ..default()
                },
                children![
                    notification_title_row(&notification.summary, created_time),
                    notification_message(&notification.body),
                ]
            ),
            notification_dismiss_button(notification_id),
        ]
    )
}

// --- Refactored notification card creation ---

fn create_notification_card(
    commands: &mut Commands,
    parent: Entity,
    notification_id: u32,
    notification: Notification,
    asset_server: &Res<AssetServer>,
    created_time: Instant,
) {
    let icon_handle = asset_server.load(format!("icons/{}.png", notification_id));
    commands.entity(parent).with_children(|parent| {
        parent.spawn(notification_card_bundle(
            notification_id,
            &notification,
            icon_handle,
            created_time,
        ));
    });
}

fn process_notifications(
    mut events: EventReader<NotificationEvent>,
    mut commands: Commands,
    mut notification_storage: ResMut<AllNotifications>,
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
                info!(
                    "Notification received: id={:?}, application={:?}, expires={:?}",
                    &id,
                    &notification.app_name,
                    notification.get_expire_timeout()
                );
                notification_storage.notifications.insert(id.clone(), notification.clone());
                let notification_image_path = format!("assets/icons/{}.png", id.clone());
                if let Some(image) = notification.get_image() {
                    image.save_to_path(notification_image_path.into());
                    info!("assets/icons/{}.png saved", &id);
                }
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
    mut notification_storage: ResMut<AllNotifications>,
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
    mut all_notifications: ResMut<AllNotifications>,
    card_query: Query<(Entity, &NotificationCard)>,
    time: Res<Time>
) {
    let current_time = std::time::Instant::now();
    let mut to_remove = Vec::new();
    for (entity, card) in &card_query {
        match all_notifications.notifications.get(&card.id) {
            Some(notification) => {
                // Check if the notification has expired
                if current_time.duration_since(card.created_at) >= notification.get_expire_timeout() &&
                    notification.get_expire_timeout() != Duration::from_millis(0)
                {
                    to_remove.push((entity, card.id.clone()));
                }
            }
            None => {
                // If the notification is not found, it might have been closed manually
                to_remove.push((entity, card.id.clone()));
            }
        };
    }

    for (entity, id) in to_remove {
        all_notifications.notifications.remove(&id);
        commands.entity(entity).despawn_recursive();
    }
}

fn print_notifications(mut events: EventReader<NotificationEvent>) {
    for event in events.read() {
        match event {
            NotificationEvent::Recieved(id, notification) => {
                info!("Notification received: id={:?}", id);
            }
            NotificationEvent::Closed(id) => {
                info!("Notification closed: id={}", id);
            }
        }
    }
}
