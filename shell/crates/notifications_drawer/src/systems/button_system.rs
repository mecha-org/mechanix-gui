use bevy::{color::palettes::css::RED, prelude::*};
use crate::components::NotificationButton;
use crate::components::buttons::*;
use crate::components::surface::*;
use crate::components::cards::*;
use crate::components::app_name_row::spawn_app_name_row;
use service_plugins::notification::NotificationEvent;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &NotificationButton,
        ),
        (Changed<Interaction>, With<Button>, With<NotificationButton>),
    >,
) {
    for (interaction, mut color, mut border_color, notification) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Log the notification interaction
                info!(
                    "Notification pressed - ID: {}, Title: '{}', Content: '{}'",
                    notification.id,
                    notification.title,
                    notification.content
                );
                
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                // border_color.0 = Color::WHITE;
            }
        }
    }
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

pub fn card_close_button_system(
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

pub fn clear_all_button_system(
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

pub fn stack_button_system(
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

pub fn card_unstack_on_click_system(
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

pub fn action_button_system(
    mut interaction_query: Query<
        (&Interaction, &ActionElement),
        (Changed<Interaction>, With<Button>)
    >,
    mut event_writer: EventWriter<NotificationEvent>
) {
    for (interaction, action_element) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            println!(
                "Action button pressed: id={}, action_id={}",
                action_element.id,
                action_element.action_id
            );

            let notification_event = NotificationEvent::ActionInvoked(
                action_element.id.clone(),
                action_element.action_id.clone()
            );
            // println!("Action Invoked: {} for notification {}", action_element.action_id, action_element.id);
            // Forward the event
            event_writer.write(notification_event);

            // Here you can add logic to handle the specific action
            // For example, send the action back to the notification server
        }
    }
}
