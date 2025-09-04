use animation::{
    combinator::{parallel, sequence, tween},
    interpolate::{background_color_to, node_to},
    prelude::*,
    tween::AnimationTarget,
};
use bevy::{color::palettes::css::*, ecs::query, prelude::*};

use crate::{
    components::{Bar, Container},
    ui::ParentContainer,
};

// #[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy, States)]
// pub enum AnimationState {
//     Opening,
//     Opened,
//     Closing,
//     #[default]
//     Closed,
// }

#[derive(Debug, Clone, Copy, Event)]
pub enum Action {
    Open,
    Close,
    Drag(Vec2),
}

pub fn listen_action(
    mut event_reader: EventReader<Action>,
    mut q_container: Query<(Entity, &mut Node), With<Container>>,
    mut q_parent_container: Query<Entity, With<ParentContainer>>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        match event {
            Action::Open => {
                println!("got Action::Open {:?}", q_container.iter().count());
                for parent in q_parent_container.iter() {
                    let start_node = Node {
                        width: Val::Percent(0.),
                        height: Val::Percent(0.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    };
                    let container_entity = commands
                        .entity(parent)
                        .insert((
                            start_node.clone(),
                            Container,
                            AnimationTarget,
                            BackgroundColor(DARK_GRAY.into()),
                        ))
                        .id();
                    let mut end_node = start_node.clone();
                    end_node.width = Val::Percent(100.);
                    end_node.height = Val::Percent(100.);

                    println!("Adding Open animation {:?}", container_entity);
                    let mut entity_commands = commands.entity(container_entity);
                    let node_target = entity_commands.id().into_target();

                    let mut node_state = node_target.state(start_node);
                    let mut bg_color_state = node_target.state(DARK_GRAY.into());
                    entity_commands.animation().insert(sequence((
                        parallel((
                            tween(
                                Duration::from_millis(700),
                                EaseKind::QuadraticOut,
                                node_state.with(node_to(end_node)),
                            ),
                            tween(
                                Duration::from_millis(700),
                                EaseKind::QuadraticOut,
                                bg_color_state.with(background_color_to(BLACK.into())),
                            ),
                        )),
                        animation::combinator::event("UniversalSearchOpened"),
                    )));
                }
            }
            Action::Close => {
                println!("got Action::Close {:?}", q_container.iter().count());
                for (entity, node) in q_container.iter() {
                    //update node width and height to 100 with animation
                    let start_node = node.clone();
                    let mut end_node = start_node.clone();
                    end_node.width = Val::Percent(0.);
                    end_node.height = Val::Percent(0.);

                    // Zprintln!("inserting close animation");
                    println!("Adding Close animation {:?}", entity);
                    let mut entity_commands = commands.entity(entity);
                    let node_target = entity_commands.id().into_target();

                    let mut node_state = node_target.state(start_node);
                    let mut bg_color_state = node_target.state(DARK_GRAY.into());

                    entity_commands.animation().insert(sequence((
                        parallel((
                            tween(
                                Duration::from_millis(700),
                                EaseKind::QuadraticOut,
                                node_state.with(node_to(end_node)),
                            ),
                            tween(
                                Duration::from_millis(700),
                                EaseKind::QuadraticOut,
                                bg_color_state.with(background_color_to(BLACK.into())),
                            ),
                        )),
                        animation::combinator::event("UniversalSearchClosed"),
                    )));
                    // info!("animation inserted");
                }
            }
            Action::Drag(pos) => {
                for mut node in q_container.iter_mut() {
                    //Update node's top left corner
                }
            }
        }
    }
}

// fn apply_animation() {
// let value = animation_target.value();
// let ui_node.
//}

// commands.spawn((
//  Node {}
//  AnimationTarget(value.x).move_to(final_value);
//))

// pub fn animate_opening(mut next_state: ResMut<NextState<AnimationState>>) {
//     //On last iteration set below
//     next_state.set(AnimationState::Opened);
// }

// pub fn animate_closing(mut next_state: ResMut<NextState<AnimationState>>) {
//     //on last iteration set below
//     next_state.set(AnimationState::Closed);
// }
