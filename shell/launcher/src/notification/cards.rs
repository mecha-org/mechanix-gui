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
                align_items: AlignItems::Center, // <-- Stretch children
                justify_content: JustifyContent::FlexStart, // <-- Start at top
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(16.0),
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
                    min_height: Val::Px(30.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    margin: UiRect::bottom(Val::Px(10.0)),
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
                            margin: UiRect::right(Val::Px(2.0)),
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
                min_height: Val::Px(81.0),
                margin: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(16.0)),
                // bottom: Val::Px(1.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderColor(Color::linear_rgba(0.2, 0.2, 0.2, 1.0)),
            BorderRadius::all(Val::Px(8.0)),
            BackgroundColor(Color::srgb(0.13, 0.13, 0.13)),
            ZIndex(id as i32),
            Card,
            children![
                (
                    Node {
                        width: Val::Auto,
                        height: Val::Px(24.0),
                        min_height: Val::Px(24.0),
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
                if let Some(ref surface) = drawing_surface {
                    create_card(notification.clone(), *id, &mut commands, surface.0, icon_handle);
                }
            }
            NotificationEvent::Closed(id) => {
                info!("Notification closed: id={}", id);
                // remove_notification_card(&mut commands, id, &mut notification_storage);
            }
        }
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
pub struct StackingState{
    pub is_stacked: bool
}

fn stack_button_system(
    mut interaction_query: Query<
        (&Interaction, &StackButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut stacking_state: ResMut<StackingState>,
    mut card_query: Query<(Entity, &mut Node, &ZIndex), With<Card>>,
    mut commands: Commands
) {
    for (interaction, _) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                println!("Stack button pressed!");
                stacking_state.is_stacked = !stacking_state.is_stacked;

                // Collect cards sorted by Z-index (newest first)
                let mut cards: Vec<_> = card_query.iter_mut().collect();
                cards.sort_by(|a, b| b.2.0.cmp(&a.2.0)); // Sort by ZIndex descending

                if stacking_state.is_stacked {
                    // Apply stacking effect
                    for (i, (entity, mut node, _)) in cards.into_iter().enumerate() {
                        let offset = (i as f32) * 4.0; // 8px offset per card

                        // Modify the node to have absolute positioning with offset
                        node.position_type = PositionType::Absolute;
                        // node.left = Val::Px(16.0 + offset); // Base position + offset
                        node.top = Val::Px(80.0 + offset); // Base position + offset
                        // node.width = Val::Px(508.0 - offset * 2.0); // Slightly smaller width

                        commands.entity(entity).insert(node.clone());
                    }
                } else {
                    // Reset to normal layout
                    for (entity, mut node, _) in cards.into_iter() {
                        node.position_type = PositionType::Relative;
                        node.left = Val::Auto;
                        node.top = Val::Auto;
                        node.width = Val::Px(508.0);

                        commands.entity(entity).insert(node.clone());
                    }
                }
            }
            _ => {}
        }
    }
}
