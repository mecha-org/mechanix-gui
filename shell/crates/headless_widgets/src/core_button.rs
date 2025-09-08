use std::time::{Duration, Instant};

use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::system::SystemId,
    input::keyboard::KeyboardInput,
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
    tasks::ComputeTaskPool,
    time::Stopwatch,
};

use crate::{
    ButtonPressed, InteractionDisabled,
    events::{ButtonClicked, ButtonLongClicked},
};

#[derive(Debug, Resource)]
pub struct Ticker(Stopwatch);

/// Headless button widget. The `on_click` field is a system that will be run when the button
/// is clicked, or when the Enter or Space key is pressed while the button is focused. If the
/// `on_click` field is `None`, the button will emit a `ButtonClicked` event when clicked.
#[derive(Component, Debug, Default)]
#[require(AccessibilityNode(accesskit::Node::new(Role::Button)))]
#[require(ButtonPressed)]
pub struct CoreButton {
    pub on_click: Option<SystemId>,
    pub on_long_press: Option<SystemId>,
}

impl CoreButton {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn on_click(mut self, on_click: SystemId) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn on_long_press(mut self, on_long_press: SystemId) -> Self {
        self.on_long_press = Some(on_long_press);
        self
    }
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
        if pressed.is_pressed && !disabled && !pressed.long_press_sent {
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
    mut commands: Commands,
    // event_loop_res: ResMut<EventLoopProxyWrapper<WakeUp>>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(false);
        if !disabled {
            pressed.is_pressed = true;
            pressed.pressed_at = Some(Instant::now());
            pressed.long_press_sent = false;
            pressed.entity = Some(trigger.target());
            focus.0 = Some(trigger.target());
            focus_visible.0 = false;
            commands.insert_resource(Ticker(Stopwatch::new()));

            // let pool = ComputeTaskPool::get();
            // let event_loop_res = event_loop_res.clone();
            // pool.spawn(async move {
            //     let start = std::time::Instant::now();
            //     loop {
            //         if start.elapsed() > Duration::from_secs(3) {
            //             break;
            //         }
            //         // println!("sending {:?}", start.elapsed());
            //         let _ = event_loop_res.send_event(WakeUp);
            //     }
            // })
            // .detach();
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
            pressed.reset();
        }
    }
}

pub(crate) fn button_on_pointer_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    mut q_state: Query<(&mut ButtonPressed, Has<InteractionDisabled>)>,
) {
    if let Ok((mut pressed, disabled)) = q_state.get_mut(trigger.target()) {
        trigger.propagate(true);
        if !disabled {
            pressed.reset();
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
            pressed.reset();
        }
    }
}

fn button_long_press(
    // mut reader: EventReader<WakeUp>,
    mut commands: Commands,
    mut q_state: Query<(&mut ButtonPressed, &CoreButton, Has<InteractionDisabled>)>,
) {
    println!("button_long_press()");
    // for _ in reader.read() {
    //     for (mut pressed, bstate, disabled) in q_state.iter_mut() {
    //         if let Some(pressed_at) = pressed.pressed_at {
    //             if !disabled
    //                 && pressed.is_pressed
    //                 && Instant::now() - pressed_at > Duration::from_secs(2)
    //             {
    //                 if let Some(on_long_press) = bstate.on_long_press {
    //                     commands.run_system(on_long_press);
    //                 } else {
    //                     commands.trigger_targets(ButtonLongClicked, pressed.entity.unwrap());
    //                 }
    //                 pressed.reset();
    //             }
    //         }
    //     }
    //}
}

fn ticker(
    ticker: Option<ResMut<Ticker>>,
    mut commands: Commands,
    mut q_state: Query<(&mut ButtonPressed, &CoreButton, Has<InteractionDisabled>)>,
) {
    if let Some(mut ticker) = ticker {
        ticker.0.tick(Duration::from_secs(1));
        for (mut pressed, bstate, disabled) in q_state.iter_mut() {
            if let Some(pressed_at) = pressed.pressed_at {
                if !disabled
                    && pressed.is_pressed
                    && Instant::now() - pressed_at > Duration::from_secs(2)
                {
                    if let Some(on_long_press) = bstate.on_long_press {
                        commands.run_system(on_long_press);
                        commands.remove_resource::<Ticker>();
                    } else {
                        commands.trigger_targets(ButtonLongClicked, pressed.entity.unwrap());
                    }
                    pressed.reset();
                }
            }
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
            .add_observer(button_on_pointer_drag_start)
            .add_observer(button_on_pointer_cancel)
            .add_systems(Update, ticker);
        // .add_systems(Update, button_long_press);
    }
}
