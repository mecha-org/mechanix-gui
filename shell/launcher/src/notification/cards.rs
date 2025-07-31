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

pub struct NotificationWindowPlugin;

impl Plugin for NotificationWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_count)
            // .add_systems(Update, spawn_multiple_cards)
            .add_systems(Update, process_notifications)
            .add_systems(Update, stack_button_system)
            .add_systems(Startup, init_stacking_state)
            .add_systems(Startup, notification_stacks_init)
            .add_systems(Update, clear_all_button_system);
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
                    padding: UiRect::all(Val::Px(16.0)),
                    margin: UiRect::bottom(Val::Px(2.0)),
                    // top: Val::Px(5.0),
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
                            margin: UiRect::right(Val::Px(120.0)),
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
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::right(Val::Px(4.0)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(8.0)),
                        StackButton,
                        BackgroundColor(Color::srgb(100.0, 100.0, 100.0)),
                        children![(
                            Text::new(">"),
                            TextFont { font_size: 12.0, ..default() },
                            TextColor(Color::BLACK),
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
                            margin: UiRect::right(Val::Px(5.0)),
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
pub struct Card;

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
    icon_handle: Handle<Image>
) {
    // Spawn the card node first, get its Entity
    let card_entity = commands
        .spawn((
            Node {
                width: Val::Px(508.0),
                height: Val::Px(81.0),
                margin: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(10.0)),
                bottom: Val::Px(2.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
            BorderRadius::all(Val::Px(8.0)),
            BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
            // ZIndex(id as i32),
            BoxShadow::new(
                Color::BLACK.with_alpha(0.5), // shadow color
                Val::Px(4.0), // x offset
                Val::Px(-4.0), // y offset (negative for “down”)
                Val::Px(2.0), // spread
                Val::Px(8.0) // blur radius
            ),
            Card,
            children![
                (
                    Node {
                        width: Val::Auto,
                        height: Val::Px(24.0),
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
                    Node {
                        width: Val::Px(484.0),
                        height: Val::Px(15.0),
                        top: Val::Px(8.0),
                        margin: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    children![(
                        Text::new(notification.summary),
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::WHITE),
                    )],
                )
            ],
        ))
        .id();

    // Insert the card as the first child of the drawing_surface
    commands.entity(drawing_surface).insert_children(1, &[card_entity]);
}

fn process_notifications(
    mut events: EventReader<NotificationEvent>,
    mut commands: Commands,
    drawing_surface: Option<Res<NotificationSurfaceEntity>>,
    mut stacks: ResMut<NotificationStacks>,
    asset_server: Res<AssetServer>
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
                    create_card(notification.clone(), *id, &mut commands, app_stack, icon_handle);
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
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..default()
                },
                NotificationStack {
                    app_name: app_name.clone(),
                },
                children![(
                    Node {
                        width: Val::Px(509.0),
                        height: Val::Auto,
                        padding: UiRect::all(Val::Px(16.0)),
                        margin: UiRect::bottom(Val::Px(5.0)),
                        // top: Val::Px(5.0),
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
                                margin: UiRect::right(Val::Px(250.0)),
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
                                margin: UiRect::right(Val::Px(8.0)),
                                ..default()
                            },
                            BorderRadius::all(Val::Px(8.0)),
                            StackButton,
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
                            ClearAllButton,
                            BackgroundColor(Color::srgb(77.0, 77.0, 77.0)),
                            children![(
                                Text::new("X"),
                                TextFont { font_size: 18.0, ..default() },
                                TextColor(Color::BLACK),
                            )],
                        )
                    ],
                )],
            ))
            .id();

        // Insert the new stack as the first child (or change index as needed)
        commands.entity(parent).insert_children(1, &[stack_entity]);

        // Save in resource
        stacks.0.insert(app_name, stack_entity);

        stack_entity
    }
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

fn clear_all_button_system(
    mut interaction_query: Query<
        (&Interaction, &ClearAllButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut commands: Commands,
    card_query: Query<(Entity, &Card)>
) {
    for (interaction, entity) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Handle the "Clear all" button press here
                println!("Clear All button pressed!");
                for (entity, id) in card_query {
                    // all_notifications.notifications.remove(&id);
                    commands.entity(entity).despawn_recursive();
                }
                // Example: commands.entity(entity).despawn_recursive();
            }
            _ => {}
        }
    }
}

pub fn init_stacking_state(mut commands: Commands) {
    commands.insert_resource(StackingState::default());
}

#[derive(Component)]
pub struct StackButton;

#[derive(Resource, Default)]
pub struct StackingState {
    pub is_stacked: bool,
}

fn stack_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StackButton>)>,
    mut stacking_state: ResMut<StackingState>,
    stack_query: Query<(Entity, &Children), With<NotificationStack>>,
    mut node_query: Query<(&ZIndex, &mut Node), With<Card>>,
    mut commands: Commands
) {
    // Toggle on any button press
    if interaction_query.iter().any(|i| *i == Interaction::Pressed) {
        stacking_state.is_stacked = !stacking_state.is_stacked;

        for (stack_entity, children) in stack_query.iter() {
            // Collect cards in this stack, sorted by ZIndex descending
            let mut cards: Vec<(Entity, i32)> = children
                .iter()
                .filter_map(|child| {
                    node_query
                        .get(child)
                        .ok()
                        .map(|(zidx, _)| (child, zidx.0))
                })
                .collect();
            cards.sort_by(|a, b| b.1.cmp(&a.1));

            // Apply per-stack offsets
            for (i, (card_ent, _)) in cards.iter().enumerate() {
                if let Ok((_, mut node)) = node_query.get_mut(*card_ent) {
                    if stacking_state.is_stacked {
                        // Absolute inside stack container
                        node.position_type = PositionType::Absolute;
                        node.bottom = Val::Px((i as f32) * 4.0);
                        if i == 0 {
                            node.position_type = PositionType::Relative;
                        }
                        // node.left = Val::Px((i as f32) * 6.0);
                        // node.width = Val::Px(508.0 - (i as f32) * 12.0);
                    } else {
                        // Reset to normal layout
                        node.position_type = PositionType::Relative;
                        node.top = Val::Auto;
                        node.bottom = Val::Px(2.0);
                        node.left = Val::Auto;
                        node.width = Val::Px(508.0);
                    }
                    // Re-insert updated Node
                    commands.entity(*card_ent).insert(node.clone());
                }
            }
        }
    }
}
