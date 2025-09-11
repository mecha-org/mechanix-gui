use accesskit::{Orientation, Role};
use bevy::{
    a11y::AccessibilityNode,
    ecs::system::SystemId,
    input::{ButtonState, keyboard::KeyboardInput},
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
};

use crate::{InteractionDisabled, ValueChange};

/// A headless slider widget, which can be used to build custom sliders. This component emits
/// [`ValueChange`] events when the slider value changes. Note that the value in the event is
/// unclamped - the reason is that the receiver may want to quantize or otherwise modify the value
/// before clamping. It is the receiver's responsibility to update the slider's value when
/// the value change event is received.
#[derive(Component, Debug)]
#[require(SliderDragState)]
#[require(AccessibilityNode(accesskit::Node::new(Role::Slider)))]
pub struct CoreSlider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub increment: f32,
    pub thumb_size: f32,
    pub on_change: Option<SystemId<In<f32>>>,
}

impl Default for CoreSlider {
    fn default() -> Self {
        Self {
            value: 0.5,
            min: 0.0,
            max: 1.0,
            increment: 1.0,
            thumb_size: 0.0,
            on_change: None,
        }
    }
}

impl CoreSlider {
    /// Get the current value of the slider.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Set the value of the slider, clamping it to the min and max values.
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    /// Set the minimum and maximum value of the slider, clamping the current value to the new
    /// range.
    pub fn set_range(&mut self, min: f32, max: f32) {
        self.min = min;
        self.max = max;
        self.value = self.value.clamp(min, max);
    }

    /// Compute the position of the thumb on the slider, as a value between 0 and 1.
    pub fn thumb_position(&self) -> f32 {
        if self.max > self.min {
            (self.value - self.min) / (self.max - self.min)
        } else {
            0.5
        }
    }
}

/// Component used to manage the state of a slider during dragging.
#[derive(Component, Default)]
pub struct SliderDragState {
    /// Whether the slider is currently being dragged.
    pub dragging: bool,
    /// The value of the slider when dragging started.
    offset: f32,
}

pub(crate) fn slider_on_pointer_down(
    trigger: Trigger<Pointer<Pressed>>,
    q_state: Query<(), With<CoreSlider>>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
) {
    if q_state.contains(trigger.target()) {
        // Set focus to slider and hide focus ring
        focus.0 = Some(trigger.target());
        focus_visible.0 = false;
    }
}

pub(crate) fn slider_on_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    mut q_state: Query<(&CoreSlider, &mut SliderDragState, Has<InteractionDisabled>)>,
) {
    if let Ok((slider, mut drag, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            drag.dragging = true;
            drag.offset = slider.value;
        }
    }
}

pub(crate) fn slider_on_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut q_state: Query<(&ComputedNode, &CoreSlider, &mut SliderDragState)>,
    mut commands: Commands,
) {
    if let Ok((node, slider, drag)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if drag.dragging {
            let distance = trigger.event().distance;
            // Measure node width and slider value.
            let slider_width =
                (node.size().x * node.inverse_scale_factor - slider.thumb_size).max(1.0);
            let range = slider.max - slider.min;

            let new_value = if range > 0. {
                drag.offset + (distance.x * range) / slider_width
            } else {
                slider.min + range * 0.5
            };

            let new_value = new_value.clamp(slider.min, slider.max);

            if let Some(on_change) = slider.on_change {
                commands.run_system_with(on_change, new_value);
            } else {
                commands.trigger_targets(ValueChange(new_value), trigger.target());
            }
        }
    }
}

pub(crate) fn slider_on_drag_end(
    mut trigger: Trigger<Pointer<DragEnd>>,
    mut q_state: Query<(&CoreSlider, &mut SliderDragState)>,
) {
    if let Ok((_slider, mut drag)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if drag.dragging {
            drag.dragging = false;
        }
    }
}

fn slider_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_state: Query<(&CoreSlider, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((slider, disabled)) = q_state.get(trigger.target()) {
        let event = &trigger.event().input;
        if !disabled && event.state == ButtonState::Pressed {
            let new_value = match event.key_code {
                KeyCode::ArrowLeft => (slider.value - slider.increment).max(slider.min),
                KeyCode::ArrowRight => (slider.value + slider.increment).min(slider.max),
                KeyCode::Home => slider.min,
                KeyCode::End => slider.max,
                _ => {
                    return;
                }
            };
            trigger.propagate(false);
            if let Some(on_change) = slider.on_change {
                commands.run_system_with(on_change, new_value);
            } else {
                commands.trigger_targets(ValueChange(new_value), trigger.target());
            }
        }
    }
}

fn update_slider_a11y(mut q_state: Query<(&CoreSlider, &mut AccessibilityNode)>) {
    for (slider, mut node) in q_state.iter_mut() {
        node.set_numeric_value(slider.value.into());
        node.set_min_numeric_value(slider.min.into());
        node.set_max_numeric_value(slider.max.into());
        node.set_numeric_value_step(slider.increment.into());
        node.set_orientation(Orientation::Horizontal);
    }
}

pub struct CoreSliderPlugin;

impl Plugin for CoreSliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(slider_on_pointer_down)
            .add_observer(slider_on_drag_start)
            .add_observer(slider_on_drag_end)
            .add_observer(slider_on_drag)
            .add_observer(slider_on_key_input)
            .add_systems(PostUpdate, update_slider_a11y);
    }
}
