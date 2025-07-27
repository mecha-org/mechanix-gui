
use bevy::prelude::*;
// Import Notification struct for demonstration
// (In real use, import from the correct path)
#[derive(Debug, Clone)]
pub struct Notification {
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub expire_timeout: i32,
}

pub struct CardStackPlugin;

impl Plugin for CardStackPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Startup, spawn_stacked_cards)
            .add_systems(Update, card_hover_system);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
pub struct Card {
    pub index: usize,
}

fn spawn_stacked_cards(mut commands: Commands) {
    // Example notifications for demonstration
    let notifications = vec![
        Notification {
            app_name: "Files".to_string(),
            replaces_id: 0,
            app_icon: "folder".to_string(),
            summary: "Files".to_string(),
            body: "Content of notification goes here, maximum length of 484px".to_string(),
            actions: vec![],
            expire_timeout: 0,
        },
        Notification {
            app_name: "Mail".to_string(),
            replaces_id: 0,
            app_icon: "mail".to_string(),
            summary: "Mail".to_string(),
            body: "You have a new message!".to_string(),
            actions: vec![],
            expire_timeout: 0,
        },
    ];

    // Outer surface: a larger centered background panel
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.18, 0.19)),
            BorderRadius::all(Val::Px(18.0)),
        ))
        .with_children(|surface| {
            // Inner container for the stack, centered inside the surface
            surface.spawn((
                Node {
                    width: Val::Percent(60.0),
                    height: Val::Percent(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    ..default()
                },
            ))
            .with_children(|parent| {
                // Create a notification card for each notification
                for (i, notif) in notifications.iter().enumerate() {
                    parent.spawn(create_notification_card(notif, i));
                }
            });
        });
}

/// Creates a notification card UI from a Notification struct.
fn create_notification_card(notification: &Notification, index: usize) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(484.0),
            min_height: Val::Px(64.0),
            margin: UiRect::all(Val::Px(8.0)),
            padding: UiRect::all(Val::Px(16.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            position_type: PositionType::Absolute,
            bottom: Val::Percent(index as f32 * 6.0),
            ..default()
        },
        BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
        BorderRadius::all(Val::Px(12.0)),
        BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
        ZIndex(index as i32),
        Card { index },
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
                    ),
                ]
            ),
            // Body/message
            (
                Node {
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
                children![
                    (
                        Text::new(&notification.body),
                        TextFont { font_size: 15.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    )
                ]
            )
        ],
    )
}

fn card_hover_system(
    mut query: Query<
        (&Interaction, &Card, &mut Node),
        (Changed<Interaction>, With<Card>, With<Button>)
    >
) {
    for (interaction, card, mut node) in &mut query {
        // Base offset step (percent). Adjust as desired.
        let base_offset = (card.index as f32) * 2.0;
        match *interaction {
            Interaction::Hovered => {
                // Spread cards further apart on hover
                println!("Card {} hovered", card.index);
                node.bottom = Val::Percent(base_offset + 1.0);
            }
            Interaction::None => {
                // Reset to original stacking offset
                node.bottom = Val::Percent(base_offset);
            }
            Interaction::Pressed => {
                // Optionally bring pressed card even higher
                node.bottom = Val::Percent(base_offset + 20.0);
            }
        }
    }
}
