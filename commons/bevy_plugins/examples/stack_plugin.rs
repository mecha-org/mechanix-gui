// --- UI constants ---
const CARD_WIDTH: f32 = 484.0;
const CARD_MIN_HEIGHT: f32 = 64.0;
const CARD_MARGIN: f32 = 8.0;
const CARD_PADDING: f32 = 16.0;
const CARD_BORDER_RADIUS: f32 = 12.0;
const CARD_BORDER_COLOR: Color = Color::linear_rgba(0.2, 0.2, 0.2, 1.0);
const CARD_BG_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);
const CARD_Z_OFFSET: f32 = 6.0;
const CARD_SEPARATION: f32 = 10.0;
const CARD_HOVER_OFFSET: f32 = 1.0;
const STACK_CONTAINER_WIDTH: f32 = 100.0;
const STACK_CONTAINER_HEIGHT: f32 = 100.0;
const STACK_CONTAINER_BG: Color = Color::srgb(0.18, 0.18, 0.19);
const STACK_CONTAINER_RADIUS: f32 = 18.0;
const BUTTON_ROW_HEIGHT: f32 = 40.0;
const BUTTON_ROW_MARGIN_LEFT: f32 = 12.0;
const BUTTON_SIZE: f32 = 24.0;
const BUTTON_MARGIN: f32 = 4.0;
const BUTTON_RADIUS: f32 = 8.0;
const BUTTON_BG: Color = Color::WHITE;
const BUTTON_TEXT_SIZE: f32 = 12.0;
const BUTTON_TEXT_COLOR: Color = Color::BLACK;
const APP_NAME_TEXT_SIZE: f32 = 18.0;
const APP_NAME_TEXT_COLOR: Color = Color::WHITE;
const CARD_BODY_TEXT_SIZE: f32 = 15.0;
const CARD_BODY_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const CARD_SUMMARY_TEXT_SIZE: f32 = 18.0;
const CARD_SUMMARY_TEXT_COLOR: Color = Color::WHITE;
const CARD_TIME_TEXT_SIZE: f32 = 14.0;
const CARD_TIME_TEXT_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const CARDS_COLUMN_WIDTH: f32 = 60.0;
const CARDS_COLUMN_HEIGHT: f32 = 80.0;
const CARDS_COLUMN_MIN_HEIGHT: f32 = 80.0;
const CARD_SET_MIN_HEIGHT: f32 = 120.0;

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
    // You can spawn multiple stacks for different apps if needed
    let stack_entity = spawn_notification_stack(&mut commands, "Files", &notifications);
    commands.insert_resource(StackSurfaceEntity(stack_entity));
}

/// Creates a notification card UI from a Notification struct.
fn create_notification_card(notification: &Notification, index: usize) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(CARD_WIDTH),
            min_height: Val::Px(CARD_MIN_HEIGHT),
            margin: UiRect::all(Val::Px(CARD_MARGIN)),
            padding: UiRect::all(Val::Px(CARD_PADDING)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            position_type: PositionType::Absolute,
            bottom: Val::Percent(index as f32 * CARD_Z_OFFSET),
            ..default()
        },
        BorderColor(CARD_BORDER_COLOR),
        BorderRadius::all(Val::Px(CARD_BORDER_RADIUS)),
        BackgroundColor(CARD_BG_COLOR),
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
                            width: Val::Px(BUTTON_SIZE),
                            height: Val::Px(BUTTON_SIZE),
                            margin: UiRect::right(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.7, 0.2)),
                    ),
                    (
                        Text::new(&notification.summary),
                        TextFont { font_size: CARD_SUMMARY_TEXT_SIZE, ..default() },
                        TextColor(CARD_SUMMARY_TEXT_COLOR),
                    ),
                    (
                        Text::new("   ·   3h"),
                        TextFont { font_size: CARD_TIME_TEXT_SIZE, ..default() },
                        TextColor(CARD_TIME_TEXT_COLOR),
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
                        TextFont { font_size: CARD_BODY_TEXT_SIZE, ..default() },
                        TextColor(CARD_BODY_TEXT_COLOR),
                    )
                ]
            )
        ],
    )
}

use std::collections::HashMap;

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
    mut commands: Commands,
    surface: Option<Res<StackSurfaceEntity>>,
) {
    let mut any_pressed = false;
    for (interaction, card, mut node) in &mut param_set.p0() {
        let base_offset = (card.index as f32) * CARD_HOVER_OFFSET;
        if separated.separated {
            node.bottom = Val::Percent(base_offset + CARD_SEPARATION * (card.index as f32 + 1.0));
            if let Interaction::Pressed = *interaction {
                println!("Card {} was clicked when unstacked", card.index);
            }
            continue;
        }
        match *interaction {
            Interaction::Hovered => {
                node.bottom = Val::Percent(base_offset + CARD_HOVER_OFFSET);
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

// Spawns a notification stack entity: a column with a button row (app_name left, buttons right) and a stack of cards below
fn spawn_notification_stack(
    commands: &mut Commands,
    app_name: &str,
    notifications: &HashMap<u32, Notification>,
) -> Entity {
    let stack_entity = commands
        .spawn((
            Node {
                width: Val::Percent(STACK_CONTAINER_WIDTH),
                height: Val::Percent(STACK_CONTAINER_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(STACK_CONTAINER_BG),
            BorderRadius::all(Val::Px(STACK_CONTAINER_RADIUS)),
        ))
        .id();
    // Button row: app_name left, buttons right (hidden by default)
    commands.entity(stack_entity).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(BUTTON_ROW_HEIGHT),
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
                    margin: UiRect::left(Val::Px(BUTTON_ROW_MARGIN_LEFT)),
                    ..default()
                },
                children![
                    (
                        Text::new(app_name),
                        TextFont { font_size: APP_NAME_TEXT_SIZE, ..default() },
                        TextColor(APP_NAME_TEXT_COLOR),
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
                        width: Val::Px(BUTTON_SIZE),
                        height: Val::Px(BUTTON_SIZE),
                        margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BUTTON_BG),
                    BorderRadius::all(Val::Px(BUTTON_RADIUS)),
                    RestackButton,
                    children![
                        (
                            Text::new("<"),
                            TextFont { font_size: BUTTON_TEXT_SIZE, ..default() },
                            TextColor(BUTTON_TEXT_COLOR),
                        )
                    ]
                ));
                // Remove button: 'x'
                bar.spawn((
                    Button,
                    Node {
                        width: Val::Px(BUTTON_SIZE),
                        height: Val::Px(BUTTON_SIZE),
                        margin: UiRect::all(Val::Px(BUTTON_MARGIN)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(BUTTON_BG),
                    BorderRadius::all(Val::Px(BUTTON_RADIUS)),
                    RemoveButton,
                    children![
                        (
                            Text::new("x"),
                            TextFont { font_size: BUTTON_TEXT_SIZE, ..default() },
                            TextColor(BUTTON_TEXT_COLOR),
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
                width: Val::Percent(CARDS_COLUMN_WIDTH),
                height: Val::Percent(CARDS_COLUMN_HEIGHT),
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
