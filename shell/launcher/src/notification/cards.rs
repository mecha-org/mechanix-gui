use bevy::prelude::*;
use bevy_smithay::{
    SmithayPlugin,
    SmithayWindowType,
    prelude::{ layer_shell::LayerShellSettings, subsurface::Anchor },
};
use bevy_core_widgets::{ CoreButton, CoreScrollArea, InteractionDisabled, Orientation };
use freedesktop_notifications_server::notification::{ Notification };
use bevy_plugins::notification::{ NotificationPlugin, NotificationEvent };
use crate::components::apps_grid;
use crate::launcher::HomescreenWindow;
use crate::launcher::spawn_camera;
pub struct NotificationWindow;

#[derive(Component)]
pub struct ActionElement {
    pub id: u32,
    pub action_id: String,
}

pub fn get_actions_row(actions: Vec<String>) -> impl Bundle {
    // Check if we have display text actions (pairs: action_id, display_text)
    let has_actions = actions.len() >= 2;
    
    (
        Node {
            width: Val::Px(509.0),
            height: if has_actions { Val::Auto } else { Val::Px(0.0) },
            margin: if has_actions { 
                UiRect::top(Val::Px(8.0)) 
            } else { 
                UiRect::ZERO 
            },
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default()
        },
    )
}

pub struct NotificationWindowPlugin;

impl Plugin for NotificationWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_count)
            // .add_systems(Update, spawn_multiple_cards)
            .add_systems(Update, process_notifications)
            .add_systems(Update, stack_button_system)
            .add_systems(Update, clear_button_system)
            .add_systems(Update, card_close_button_system)
            .add_systems(Update, card_unstack_on_click_system)
            .add_systems(Update, action_button_system)
            .add_systems(Startup, notification_stacks_init)
            .add_systems(Update, clear_all_button_system)
            .init_resource::<AppStackingState>();
    }
}

#[derive(Component)]
pub struct ClearAllButton;

#[derive(Component)]
pub struct ClearButton {
    pub app_name: String,
}

#[derive(Component)]
pub struct CardCloseButton {
    pub card_entity: Entity,
}

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
                justify_content: JustifyContent::FlexStart, // <-- Start at top
                padding: UiRect::all(Val::Px(16.0)),
                column_gap: Val::Px(16.0),
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
                    // padding: UiRect::all(Val::Px(5.0)),
                    margin: UiRect {
                        top: Val::Px(7.0),
                        bottom: Val::Px(10.0),
                        left: Val::ZERO,
                        right: Val::ZERO,
                    },
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
                            // margin: UiRect::left(Val::Px(10.0)),
                            ..default()
                        },
                        children![(
                            Text::new("Notifications"),
                            TextFont { font_size: 24.0, ..default() },
                            TextColor(Color::WHITE),
                        )],
                    ),
                    (
                        Button,
                        Node {
                            width: Val::Px(78.0),
                            height: Val::Px(30.0),
                            // top: Val::Px(8.0),
                            justify_content: JustifyContent::Center, // <-- Center horizontally
                            align_items: AlignItems::Center,
                            // margin: UiRect::right(Val::Px(2.0)),
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
pub struct Card {
    pub app_name: String,
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

// pub fn spawn_multiple_cards(
//     mut commands: Commands,
//     drawing_surface: Option<Res<NotificationSurfaceEntity>>,
//     asset_server: Res<AssetServer>
// ) {
//     let icon_handle = asset_server.load(format!("icons/{}.png", 1));
//     if let Some(ref surface) = drawing_surface {
//         create_card(&mut commands, surface.0, icon_handle)
//     }
// }

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
                margin: UiRect{bottom: Val::Px(5.0),..default()},
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
                if chunk.len() == 2 {
                    Some((chunk[0].clone(), chunk[1].clone()))
                } else {
                    None
                }
            })
            .collect();
        
        let action_buttons: Vec<Entity> = action_tuples
            .iter()
            .enumerate()
            .map(|(index, (action_id, action_text))| {
                commands.spawn((
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
                        id: index as u32,
                        action_id: action_id.clone(),
                    },
                    children![(
                        Text::new(action_text),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::WHITE),
                    )],
                )).id()
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

fn process_notifications(
    mut events: EventReader<NotificationEvent>,
    mut commands: Commands,
    drawing_surface: Option<Res<NotificationSurfaceEntity>>,
    mut stacks: ResMut<NotificationStacks>,
    asset_server: Res<AssetServer>,
    app_stacking: Res<AppStackingState>,
    stack_query: Query<&Children, With<NotificationStack>>,
    card_query: Query<Entity, With<Card>>
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
                let image_path = save_notification_image_to_assets(&id, &notification);
                let icon_handle = asset_server.load(image_path);
                // let drawing_surface = get_stack_enitity_of_notification()
                if let Some(ref surface) = drawing_surface {
                    let app_stack = get_stack_enitity_of_notification(
                        surface,
                        notification.clone(),
                        &mut stacks,
                        &mut commands
                    );
                    create_card(
                        notification.clone(),
                        *id,
                        &mut commands,
                        app_stack,
                        icon_handle,
                        &app_stacking,
                        &stack_query,
                        &card_query
                    );
                }
            }
            NotificationEvent::Closed(id) => {
                info!("Notification closed: id={}", id);
                // remove_notification_card(&mut commands, id, &mut notification_storage);
            }
        }
    }
}

#[derive(Component)]
pub struct NotificationStack {
    app_name: String,
}

use std::collections::HashMap;
#[derive(Resource, Default)]
pub struct NotificationStacks(pub HashMap<String, Entity>);

pub fn notification_stacks_init(mut commands: Commands) {
    commands.insert_resource(NotificationStacks(HashMap::new()));
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

pub fn spawn_app_name_row(app_name: String) -> impl Bundle {
    (
        Node {
            width: Val::Px(509.0),
            height: Val::Auto,
            // padding: UiRect::all(Val::Px(5.0)),
            margin: UiRect::bottom(Val::Px(15.0)),
            bottom: Val::Px(5.0),
            top: Val::Px(5.0),
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
                    height: Val::Auto,
                    padding: UiRect::left(Val::Px(2.0)),
                    margin: UiRect::right(Val::Px(210.0)),
                    ..default()
                },
                children![(
                    Text::new(&app_name),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                )],
            ),
            (
                Button,
                Node {
                    width: Val::Px(28.0),
                    height: Val::Px(26.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(8.0)),
                StackButton { app_name: app_name.clone(), is_stacked: false },
                BackgroundColor(Color::srgb(100.0, 100.0, 100.0)),
                children![(
                    Text::new(">"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::BLACK),
                )],
            ),

            (
                Button,
                Node {
                    width: Val::Px(28.0),
                    height: Val::Px(26.0),
                    // top: Val::Px(8.0),
                    justify_content: JustifyContent::Center, // <-- Center horizontally
                    align_items: AlignItems::Center,
                    // margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(8.0)),
                ClearButton { app_name: app_name.clone() },
                BackgroundColor(Color::srgb(77.0, 77.0, 77.0)),
                children![(
                    Text::new("×"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::BLACK),
                )],
            )
        ],
    )
}

pub fn save_notification_image_to_assets(id: &u32, notification: &Notification) -> String {
    let notification_image_path = format!("assets/icons/{}.png", id.clone());
    if let Some(image) = notification.get_image() {
        image.save_to_path(notification_image_path.clone().into());
        info!("assets/icons/{}.png saved", &id);
    }
    let notification_image_path = format!("icons/{}.png", id.clone());
    notification_image_path
}

pub fn clear_button_system(
    mut interaction_query: Query<
        (&Interaction, &ClearButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut commands: Commands,
    mut stacks: ResMut<NotificationStacks>
) {
    for (interaction, clear_btn) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let app_name = &clear_btn.app_name;
            if let Some(&stack) = stacks.0.get(app_name) {
                commands.entity(stack).despawn();
                stacks.0.remove(app_name);
            }
        }
    }
}

fn card_close_button_system(
    mut interaction_query: Query<
        (&Interaction, &CardCloseButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut commands: Commands,
    card_query: Query<&Card>,
    stack_query: Query<&Children, With<NotificationStack>>,
    mut stacks: ResMut<NotificationStacks>,
    mut app_stacking: ResMut<AppStackingState>
) {
    for (interaction, close_btn) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let card_entity = close_btn.card_entity;

            // Get the card's app_name to find its stack
            if let Ok(card) = card_query.get(card_entity) {
                let app_name = &card.app_name;

                // Despawn only the specific card
                commands.entity(card_entity).despawn();

                // Check if this was the last card in the stack
                if let Some(&stack_entity) = stacks.0.get(app_name) {
                    if let Ok(children) = stack_query.get(stack_entity) {
                        // Count remaining cards (excluding the header row at index 0)
                        let remaining_cards = children
                            .iter()
                            .skip(1) // Skip header
                            .filter(|&child| {
                                // Check if it's still a valid card and not the one we just despawned
                                child != card_entity && card_query.get(child).is_ok()
                            })
                            .count();

                        // If no cards remain, remove the entire stack
                        if remaining_cards == 0 {
                            commands.entity(stack_entity).despawn();
                            stacks.0.remove(app_name);
                            // Also clear the stacking state for this app
                            app_stacking.0.remove(app_name);
                        }
                    }
                }
            }
        }
    }
}

fn clear_all_button_system(
    mut interaction_query: Query<
        (&Interaction, &ClearAllButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut commands: Commands,
    stack_query: Query<Entity, With<NotificationStack>>,
    mut stacks: ResMut<NotificationStacks>,
    mut app_stacking: ResMut<AppStackingState>
) {
    for (interaction, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Handle the "Clear all" button press here
                println!("Clear All button pressed!");

                // Despawn all notification stacks (which will recursively despawn their children including cards and headers)
                for stack_entity in stack_query.iter() {
                    commands.entity(stack_entity).despawn();
                }

                // Clear the stacks resource
                stacks.0.clear();

                // Clear the app stacking state
                app_stacking.0.clear();
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct StackButton {
    pub app_name: String,
    pub is_stacked: bool,
}

// Persistent stacking state per app
#[derive(Resource, Default)]
pub struct AppStackingState(pub std::collections::HashMap<String, bool>);

fn stack_button_system(
    mut stack_btn_interaction_query: Query<
        (&Interaction, &StackButton),
        (Changed<Interaction>, With<Button>)
    >,
    stacks: Res<NotificationStacks>,
    stack_query: Query<&Children, With<NotificationStack>>,
    mut node_query: Query<(&ZIndex, &mut Node), With<Card>>,
    mut app_stacking: ResMut<AppStackingState>,
    mut commands: Commands
) {
    for (interaction, stack_btn) in stack_btn_interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let app_name = &stack_btn.app_name;
            // Toggle stacking state for this app
            let is_stacked = app_stacking.0
                .entry(app_name.clone())
                .and_modify(|v| {
                    *v = !*v;
                })
                .or_insert(true);

            if let Some(&stack_entity) = stacks.0.get(app_name) {
                if let Ok(children) = stack_query.get(stack_entity) {
                    if children.is_empty() {
                        continue;
                    }

                    let header_entity = children[0];

                    // Apply stacking or unstacking
                    if *is_stacked {
                        // Stacking: hide header and stack cards
                        commands.entity(header_entity).despawn();

                        let mut card_index = 0;
                        for child in children.iter() {
                            if let Ok((_, mut node)) = node_query.get_mut(child) {
                                node.position_type = PositionType::Absolute;
                                node.bottom = Val::Px((card_index as f32) * 4.0);
                                node.top = Val::Px(5.0);
                                if card_index == 0 {
                                    node.position_type = PositionType::Relative;
                                }
                                card_index += 1;
                            }
                        }
                    } else {
                        // Unstacking: show cards separately and respawn header
                        for child in children.iter() {
                            if let Ok((_, mut node)) = node_query.get_mut(child) {
                                node.position_type = PositionType::Relative;
                                node.top = Val::Auto;
                                node.bottom = Val::Px(2.0);
                                node.left = Val::Auto;
                                node.width = Val::Px(508.0);
                            }
                        }

                        // Respawn header at the beginning
                        let new_header_entity = commands
                            .spawn(spawn_app_name_row(app_name.clone()))
                            .id();
                        commands.entity(stack_entity).insert_children(0, &[new_header_entity]);
                    }
                }
            }
        }
    }
}

fn card_unstack_on_click_system(
    mut card_interaction_query: Query<(&Interaction, &Card), (Changed<Interaction>, With<Button>)>,
    stacks: Res<NotificationStacks>,
    stack_query: Query<&Children, With<NotificationStack>>,
    mut node_query: Query<(&ZIndex, &mut Node), With<Card>>,
    mut app_stacking: ResMut<AppStackingState>,
    mut commands: Commands
) {
    for (interaction, card) in card_interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let app_name = &card.app_name;
            println!("Clicked on {}'s Card", &app_name);
            // Check if stacking was enabled for this app
            let was_stacked = app_stacking.0.get(app_name).copied().unwrap_or(false);
            // Set stacking to false (unstack)
            if was_stacked == true {
                app_stacking.0.insert(app_name.clone(), false);

                if let Some(&stack_entity) = stacks.0.get(app_name) {
                    if let Ok(children) = stack_query.get(stack_entity) {
                        // Collect cards in this stack, sorted by ZIndex ascending (bottom to top)
                        let mut cards: Vec<(Entity, i32)> = children
                            .iter()
                            .filter_map(|child| {
                                node_query
                                    .get(child)
                                    .ok()
                                    .map(|(zidx, _)| (child, zidx.0))
                            })
                            .collect();
                        cards.sort_by(|a, b| a.1.cmp(&b.1));

                        // Apply stacking or unstacking
                        for (_, (card_ent, _)) in cards.iter().enumerate() {
                            if let Ok((_, mut node)) = node_query.get_mut(*card_ent) {
                                node.position_type = PositionType::Relative;
                                node.top = Val::Auto;
                                node.bottom = Val::Auto;
                                node.left = Val::Auto;
                                node.width = Val::Px(508.0);
                                // Only respawn header row if we were previously stacked
                                commands.entity(*card_ent).insert(node.clone());
                            }
                        }
                        let header_entity = commands
                            .spawn(spawn_app_name_row(app_name.clone()))
                            .id();
                        commands.entity(stack_entity).insert_children(0, &[header_entity]);
                    }
                }
            }
        }
    }
}

fn action_button_system(
    mut interaction_query: Query<
        (&Interaction, &ActionElement),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, action_element) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            println!("Action button pressed: id={}, action_id={}", 
                action_element.id, action_element.action_id);
            // Here you can add logic to handle the specific action
            // For example, send the action back to the notification server
        }
    }
}
