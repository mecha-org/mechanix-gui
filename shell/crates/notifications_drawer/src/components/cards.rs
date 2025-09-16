use std::collections::HashMap;
use bevy::prelude::*;
use notification::notification::Notification;
use std::time::SystemTime;
use crate::systems::expiry::{NotificationTimestamp, format_time_elapsed};
use crate::components::buttons::{ActionElement, get_actions_row, CardCloseButton};
use crate::components::surface::{NotificationStack, NotificationStacks, NotificationSurfaceEntity};
use crate::components::app_name_row::spawn_app_name_row;
use bevy::state::state::States;


pub struct NotificationWindow;

#[derive(Resource, Default)]
pub struct Count(pub i32);

pub fn init_count(mut commands: Commands) {
    commands.insert_resource(Count(0));
}

/// Defines app asset loading states
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Component)]
pub struct Card {
    pub app_name: String,
}

// Persistent stacking state per app
#[derive(Resource, Default)]
pub struct AppStackingState(pub HashMap<String, bool>);

pub fn create_card(
    notification: Notification,
    id: u32,
    commands: &mut Commands,
    drawing_surface: Entity,
    icon_handle: Handle<Image>,
    app_stacking: &AppStackingState,
    stack_query: &Query<&Children, With<NotificationStack>>,
    card_query: &Query<Entity, With<Card>>
) {
    let app_name = &notification.app_name;
    let is_stacked = app_stacking.0.get(app_name).copied().unwrap_or(false);

    // Count existing cards in this stack to determine positioning
    let existing_card_count = if let Ok(children) = stack_query.get(drawing_surface) {
        children
            .iter()
            .filter(|&child| card_query.get(child).is_ok())
            .count()
    } else {
        0
    };

    // Spawn the card node first, get its Entity
    let card_entity = commands
        .spawn((
            Node {
                width: Val::Px(508.0),
                height: Val::Auto,
                min_height: Val::Px(81.0),
                margin: UiRect::bottom(Val::Px(7.0)),
                padding: UiRect::all(Val::Px(10.0)),
                bottom: if is_stacked && existing_card_count > 0 {
                    Val::Px((existing_card_count as f32) * 4.0)
                } else {
                    Val::Auto
                },
                position_type: if is_stacked && existing_card_count > 0 {
                    PositionType::Absolute
                } else {
                    PositionType::Relative
                },
                top: if is_stacked && existing_card_count > 0 {
                    Val::Px(5.0)
                } else {
                    Val::Auto
                },
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Button,
            ActionElement {
                id: id as u32,
                action_id: "default".to_string(),
            },
            BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
            BorderRadius::all(Val::Px(8.0)),
            BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
            ZIndex(id as i32),
            BoxShadow::new(
                Color::BLACK.with_alpha(0.5), // shadow color
                Val::Px(4.0), // x offset
                Val::Px(-4.0), // y offset (negative for "down")
                Val::Px(2.0), // spread
                Val::Px(8.0) // blur radius
            ),
            Card { app_name: notification.app_name.clone() },
        ))
        .id();

    // Now add the children to the card, including the close button that references the card entity
    let card_header = commands
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Px(24.0),
                margin: UiRect::right(Val::Px(12.0)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
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
                            BorderRadius::all(Val::Px(4.0)),
                        ),
                        (
                            Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::left(Val::Px(10.0)),
                                ..default()
                            },
                            children![(
                                Text::new(notification.app_name),
                                TextFont { font_size: 20.0, ..default() },
                                TextColor(Color::WHITE),
                            )],
                        ),
                        (
                            Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::left(Val::Px(8.0)),
                                ..default()
                            },
                            children![(
                                Text::new("•"),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::linear_rgba(0.7, 0.7, 0.7, 1.0)),
                            )],
                        ),
                        (
                            Node {
                                width: Val::Auto,
                                height: Val::Auto,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::left(Val::Px(8.0)),
                                ..default()
                            },
                            NotificationTimestamp { created_at: SystemTime::now() },
                            children![(
                                Text::new(format_time_elapsed(SystemTime::now())),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::linear_rgba(0.7, 0.7, 0.7, 1.0)),
                            )],
                        )
                    ],
                ),
                (
                    Button,
                    Node {
                        width: Val::Px(20.0),
                        height: Val::Px(20.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    CardCloseButton { card_entity },
                    children![(
                        Text::new("×"),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(Color::WHITE),
                    )],
                )
            ],
        ))
        .id();

    let card_body = commands
        .spawn((
            Node {
                width: Val::Px(484.0),
                height: Val::Px(25.0),
                top: Val::Px(10.0),
                left: Val::Px(3.0),
                margin: UiRect { bottom: Val::Px(5.0), ..default() },
                ..default()
            },
            children![(
                Text::new(notification.summary),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            )],
        ))
        .id();

    let actions_row = commands.spawn(get_actions_row(notification.actions.clone())).id();

    // Add action buttons as children to the actions row if actions exist
    if !notification.actions.is_empty() {
        // Split actions into tuples of (action_id, action_text)
        let action_tuples: Vec<(String, String)> = notification.actions
            .chunks(2)
            .filter_map(|chunk| {
                if chunk.len() == 2 { Some((chunk[0].clone(), chunk[1].clone())) } else { None }
            })
            .collect();

        let action_buttons: Vec<Entity> = action_tuples
            .iter()
            .enumerate()
            .flat_map(|(index, (action_id, action_text))| {
                let mut elements = vec![
                    commands
                        .spawn((
                            Button,
                            Node {
                                width: Val::Auto,
                                height: Val::Px(28.0),
                                padding: UiRect::horizontal(Val::Px(12.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ActionElement {
                                id: id as u32,
                                action_id: action_id.clone(),
                            },
                            children![(
                                Text::new(action_text),
                                TextFont { font_size: 16.0, ..default() },
                                TextColor(Color::WHITE),
                            )],
                        ))
                        .id()
                ];

                // Add separator if not the last element
                if index < action_tuples.len() - 1 {
                    elements.push(
                        commands
                            .spawn((
                                Node {
                                    width: Val::Auto,
                                    height: Val::Px(28.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::horizontal(Val::Px(8.0)),
                                    ..default()
                                },
                                children![(
                                    Text::new("|"),
                                    TextFont { font_size: 16.0, ..default() },
                                    TextColor(Color::linear_rgba(0.6, 0.6, 0.6, 1.0)),
                                )],
                            ))
                            .id()
                    );
                }

                elements
            })
            .collect();

        commands.entity(actions_row).add_children(&action_buttons);
    }

    // Add the header and body as children to the card
    commands.entity(card_entity).add_children(&[card_header, card_body, actions_row]);

    // Insert the card - if stacked, insert at index 1 (after header), otherwise at the end
    let insert_index = if is_stacked { 1 } else { 1 };
    commands.entity(drawing_surface).insert_children(insert_index, &[card_entity]);
}

pub fn get_stack_enitity_of_notification(
    drawing_surface: &Res<NotificationSurfaceEntity>,
    notification: Notification,
    stacks: &mut NotificationStacks,
    commands: &mut Commands
) -> Entity {
    let app_name = notification.app_name.clone();

    // Check if a stack already exists for this app_name
    if let Some(entity) = stacks.0.get(&app_name) {
        *entity
    } else {
        // Get the parent entity directly
        let parent = drawing_surface.0;

        let stack_entity = commands
            .spawn((
                Node {
                    width: Val::Px(509.0),
                    height: Val::Auto,
                    margin: UiRect {
                        top: Val::Percent(1.0),
                        bottom: Val::Percent(1.0),
                        left: Val::ZERO,
                        right: Val::ZERO,
                    },
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..default()
                },
                NotificationStack {
                    app_name: app_name.clone(),
                },
                children![spawn_app_name_row(app_name.clone())],
            ))
            .id();

        // Insert the new stack as the first child (or change index as needed)
        commands.entity(parent).insert_children(1, &[stack_entity]);

        // Save in resource
        stacks.0.insert(app_name, stack_entity);

        stack_entity
    }
}
