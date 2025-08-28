use bevy::prelude::*;

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy, States)]
pub enum AnimationState {
    Opening,
    Opened,
    Closing,
    #[default]
    Closed,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Event)]
pub enum Action {
    Open,
    Close,
}

pub fn listen_action(
    mut event_reader: EventReader<Action>,
    mut next_state: ResMut<NextState<AnimationState>>,
) {
    for event in event_reader.read() {
        match event {
            Action::Open => {
                //Animate node from its current position to top left corner
                next_state.set(AnimationState::Opening);
            }
            Action::Close => {
                //Animate node from its current position to top bottom right corner
                next_state.set(AnimationState::Closing);
            }
        }
    }
}

pub fn animate_opening(mut next_state: ResMut<NextState<AnimationState>>) {
    //On last iteration set below
    next_state.set(AnimationState::Opened);
}

pub fn animate_closing(mut next_state: ResMut<NextState<AnimationState>>) {
    //on last iteration set below
    next_state.set(AnimationState::Closed);
}
