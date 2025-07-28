// --- UI constants ---
// ...existing code...

use bevy::ecs::system::ParamSet;
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
            .init_resource::<CardUnstackAnimation>()
            .add_systems(Startup, setup_camera)
            .add_systems(Startup, spawn_stacked_cards)
            .add_systems(Update, card_hover_system)
            .add_systems(Update, button_system)
            .add_systems(Update, card_unstack_animation_system);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource, Default)]
struct CardUnstackAnimation {
    active: bool,
    timer: bevy::time::Timer,
}


#[derive(Component)]
pub struct Card {
    pub index: usize,
}

fn spawn_stacked_cards(mut commands: Commands) {
    let mut notifications = HashMap::new();
    notifications.insert(1, Notification {
        app_name: "Files".to_string(),
        replaces_id: 0,
        app_icon: "folder".to_string(),
        summary: "Files".to_string(),
        body: "Content of notification goes here, maximum length of 484px".to_string(),
        actions: vec![],
        expire_timeout: 0,
    });
    notifications.insert(2, Notification {
        app_name: "Files".to_string(),
        replaces_id: 0,
        app_icon: "folder".to_string(),
        summary: "Files".to_string(),
        body: "Another notification for Files app".to_string(),
        actions: vec![],
        expire_timeout: 0,
    });
    notifications.insert(3, Notification {
        app_name: "Things".to_string(),
        replaces_id: 0,
        app_icon: "folder".to_string(),
        summary: "Things".to_string(),
        body: "Another notification for Things app".to_string(),
        actions: vec![],
        expire_timeout: 0,
    });
    // You can spawn multiple stacks for different apps if needed
    let stack_entity = spawn_notification_stack(&mut commands, "Files", &notifications);
    commands.insert_resource(StackSurfaceEntity(stack_entity));
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

use std::collections::HashMap;
use std::time::Duration;

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

#[derive(Component)]
struct RemoveButton;
#[derive(Component)]
struct RestackButton;
#[derive(Component)]
struct ButtonRow;

fn card_hover_system(
    mut param_set: ParamSet<(
        Query<(&Interaction, &Card, &mut Node), (Changed<Interaction>, With<Card>, With<Button>)>,
        Query<(Entity, &mut Node), With<ButtonRow>>,
    )>,
    mut separated: ResMut<SeparatedCards>,
    mut animation: ResMut<CardUnstackAnimation>,
    mut commands: Commands,
    surface: Option<Res<StackSurfaceEntity>>,
) {
    let mut any_pressed = false;
    for (interaction, card, mut node) in &mut param_set.p0() {
        let base_offset = (card.index as f32) * 1.0;
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
    // If any card was pressed and not already separated, just separate all and show button row
    if any_pressed && !separated.separated {
        separate_all_cards(&mut separated);
        // Start animation
        animation.active = true;
        animation.timer = bevy::time::Timer::from_seconds(10000.000, bevy::time::TimerMode::Once);
        // Show the button row
        for (entity, mut node) in &mut param_set.p1() {
            node.display = Display::Flex;
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
                commands.entity(surface.0).despawn();
            }
        }
    }
    for (entity, interaction) in &mut restack_query {
        if let Interaction::Pressed = *interaction {
            // Restack all cards and remove buttons
            restack_all_cards(&mut separated);
            if let Some(ref surface) = stack_surface {
                commands.entity(surface.0).despawn();
            }
            restack_clicked = true;
        }
    }
    // Respawn the stack if restack was clicked
    if restack_clicked {
        spawn_stacked_cards(commands);
    }
}

// Animate the card positions when unstacking
fn card_unstack_animation_system(
    mut animation: ResMut<CardUnstackAnimation>,
    time: Res<Time>,
    mut query: Query<(&Card, &mut Node)>,
    separated: Res<SeparatedCards>,
) {
    if !animation.active {
        return;
    }
    animation.timer.tick(time.delta());
    let t = (animation.timer.elapsed_secs() / 0.3).min(1.0);
    for (card, mut node) in &mut query {
        // Animate from stacked to separated
        let base_offset = (card.index as f32) * 1.0;
        let target = base_offset + 20.0 * (card.index as f32 + 1.0);
        let start = base_offset;
        node.bottom = Val::Percent(start + (target - start) * t);
    }
    if animation.timer.finished() {
        animation.active = false;
    }
}

// Spawns a notification stack entity: a column with a button row (app_name left, buttons right) and a stack of cards below
fn spawn_notification_stack(
    commands: &mut Commands,
    app_name: &str,
    notifications: &HashMap<u32, Notification>,
) -> Entity {
    let stack_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.18, 0.19)),
            BorderRadius::all(Val::Px(18.0)),
        ))
        .id();
    // Button row: app_name left, buttons right (hidden by default)
    commands.entity(stack_entity).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                display: Display::None, // hidden by default
                ..default()
            },
            ButtonRow,
        ))
        .with_children(|row| {
            // App name (left)
            row.spawn((
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    margin: UiRect::left(Val::Px(12.0)),
                    ..default()
                },
                children![
                    (
                        Text::new(app_name),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(Color::WHITE),
                    )
                ]
            ));
            // Button group (right)
            row.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|bar| {
                // Restack button: '<'
                bar.spawn((
                    Button,
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
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
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(Color::BLACK),
                        )
                    ]
                ));
                // Remove button: 'x'
                bar.spawn((
                    Button,
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
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
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(Color::BLACK),
                        )
                    ]
                ));
            });
        });
    });
    // Cards column
    commands.entity(stack_entity).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(60.0),
                height: Val::Percent(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Relative,
                ..default()
            },
        ))
        .with_children(|cards| {
            for (i, notif) in notifications.values().enumerate() {
                cards.spawn(create_notification_card(notif, i));
            }
        });
    });
    stack_entity
}
