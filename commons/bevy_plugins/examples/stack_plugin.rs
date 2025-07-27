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
            .init_resource::<SeparatedCards>()
            .add_systems(Startup, setup_camera)
            .add_systems(Startup, spawn_stacked_cards)
            .add_systems(Update, card_hover_system)
            .add_systems(Update, button_system);
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

    let surface_entity = commands
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
        .id();
    commands.entity(surface_entity).with_children(|surface| {
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
    // Store the surface entity for button spawning
    commands.insert_resource(StackSurfaceEntity(surface_entity));
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

use std::collections::HashSet;

#[derive(Resource, Default)]
struct SeparatedCards {
    separated: bool,
    // Track if buttons are spawned
    buttons_spawned: bool,
}

fn separate_all_cards(cards: &mut ResMut<SeparatedCards>) {
    cards.separated = true;
}

fn restack_all_cards(cards: &mut ResMut<SeparatedCards>) {
    cards.separated = false;
    cards.buttons_spawned = false;
}

fn spawn_control_buttons(commands: &mut Commands, parent: Entity) {
    // Horizontal bar above the cards, right-aligned
    commands.entity(parent).with_children(|p| {
        p.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..default()
            },
            // Optionally, add a background color for the bar
            // BackgroundColor(Color::srgb(0.18, 0.18, 0.19)),
        ))
        .with_children(|bar| {
            // Restack button: '<'
            bar.spawn((
                Button,
                Node {
                    width: Val::Px(10.0),
                    height: Val::Px(10.0),
                    margin: UiRect::all(Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                BorderRadius::all(Val::Px(8.0)),
                RestackButton,
                children![
                    (
                        Text::new("<"),
                        TextFont { font_size: 5.0, ..default() },
                        TextColor(Color::BLACK),
                    )
                ]
            ));
            // Remove button: 'x'
            bar.spawn((
                Button,
                Node {
                    width: Val::Px(10.0),
                    height: Val::Px(10.0),
                    margin: UiRect::all(Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                BorderRadius::all(Val::Px(8.0)),
                RemoveButton,
                children![
                    (
                        Text::new("x"),
                        TextFont { font_size: 5.0, ..default() },
                        TextColor(Color::BLACK),
                    )
                ]
            ));
        });
    });
}

#[derive(Component)]
struct RemoveButton;
#[derive(Component)]
struct RestackButton;

fn card_hover_system(
    mut query: Query<(&Interaction, &Card, &mut Node), (Changed<Interaction>, With<Card>, With<Button>)>,
    mut separated: ResMut<SeparatedCards>,
    mut commands: Commands,
    surface: Option<Res<StackSurfaceEntity>>,
) {
    let mut any_pressed = false;
    for (interaction, card, mut node) in &mut query {
        let base_offset = (card.index as f32) * 2.0;
        if separated.separated {
            node.bottom = Val::Percent(base_offset + 20.0 * (card.index as f32 + 1.0));
            if let Interaction::Pressed = *interaction {
                println!("Card {} was clicked when unstacked", card.index);
            }
            continue;
        }
        match *interaction {
            Interaction::Hovered => {
                node.bottom = Val::Percent(base_offset + 1.0);
            }
            Interaction::None => {
                node.bottom = Val::Percent(base_offset);
            }
            Interaction::Pressed => {
                any_pressed = true;
            }
        }
    }
    // If any card was pressed and not already separated, separate all and spawn buttons
    if any_pressed && !separated.separated {
        separate_all_cards(&mut separated);
        if let Some(surface) = surface {
            if !separated.buttons_spawned {
                spawn_control_buttons(&mut commands, surface.0);
                separated.buttons_spawned = true;
            }
        }
    }
}

#[derive(Resource, Clone, Copy)]
struct StackSurfaceEntity(Entity);

fn button_system(
    mut commands: Commands,
    mut remove_query: Query<(Entity, &Interaction), (With<RemoveButton>, Changed<Interaction>)>,
    mut restack_query: Query<(Entity, &Interaction), (With<RestackButton>, Changed<Interaction>)>,
    mut separated: ResMut<SeparatedCards>,
    stack_surface: Option<Res<StackSurfaceEntity>>,
) {
    let mut restack_clicked = false;
    for (entity, interaction) in &mut remove_query {
        if let Interaction::Pressed = *interaction {
            // Remove all cards and buttons
            if let Some(ref surface) = stack_surface {
                commands.entity(surface.0).despawn_recursive();
            }
        }
    }
    for (entity, interaction) in &mut restack_query {
        if let Interaction::Pressed = *interaction {
            // Restack all cards and remove buttons
            restack_all_cards(&mut separated);
            if let Some(ref surface) = stack_surface {
                commands.entity(surface.0).despawn_recursive();
            }
            restack_clicked = true;
        }
    }
    // Respawn the stack if restack was clicked
    if restack_clicked {
        spawn_stacked_cards(commands);
    }
}
