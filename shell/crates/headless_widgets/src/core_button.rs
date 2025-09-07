use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::system::SystemId,
    input::keyboard::KeyboardInput,
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::{ButtonPressed, InteractionDisabled, events::ButtonClicked};

/// Headless button widget. The `on_click` field is a system that will be run when the button
/// is clicked, or when the Enter or Space key is pressed while the button is focused. If the
/// `on_click` field is `None`, the button will emit a `ButtonClicked` event when clicked.
#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::Button)))]
#[require(ButtonPressed)]
pub struct CoreButton {
    pub on_click: Option<SystemId>,
}

pub(crate) fn button_on_key_event(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_state: Query<(&CoreButton, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((bstate, disabled)) = q_state.get(trigger.target()) {
        if !disabled {
            let event = &trigger.event().input;
            if !event.repeat
                && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
            {
                if let Some(on_click) = bstate.on_click {
                    trigger.propagate(false);
                    commands.run_system(on_click);
                } else {
                    commands.trigger_targets(ButtonClicked, trigger.target());
                }
            }
        }
    }
}

pub(crate) fn button_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut q_state: Query<(&CoreButton, &mut ButtonPressed, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((bstate, pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if pressed.0 && !disabled {
            if let Some(on_click) = bstate.on_click {
                commands.run_system(on_click);
            } else {
                commands.trigger_targets(ButtonClicked, trigger.target());
            }
        }
    }
}

pub(crate) fn button_on_pointer_down(
    mut trigger: Trigger<Pointer<Pressed>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = true;
            focus.0 = Some(trigger.target());
            focus_visible.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_up(
    mut trigger: Trigger<Pointer<Released>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}

pub(crate) fn button_on_pointer_cancel(
    mut trigger: Trigger<Pointer<Cancel>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.0 = false;
        }
    }
}

pub struct CoreButtonPlugin;

impl Plugin for CoreButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(button_on_key_event)
            .add_observer(button_on_pointer_down)
            .add_observer(button_on_pointer_up)
            .add_observer(button_on_pointer_click)
            .add_observer(button_on_pointer_drag_end)
            .add_observer(button_on_pointer_cancel);
    }
}
