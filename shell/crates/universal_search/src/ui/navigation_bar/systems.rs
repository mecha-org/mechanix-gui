use bevy::{ecs::system::command, prelude::*};

use crate::{UniversalSearchClose, UniversalSearchOpen, states::Action, ui::Bar};

pub fn on_bar_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    mut q_bar: Query<(), With<Bar>>,
) {
    if let Ok(()) = q_bar.get_mut(trigger.target()) {
        trigger.propagate(false);
        println!("on_drag_start()");
    }
}

pub fn on_bar_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut q_bar: Query<(), With<Bar>>,
    // mut event_writer: EventWriter<Action>,
    mut is_open: Local<bool>,
    mut last_above_threshold: Local<bool>,
    mut commands: Commands,
) {
    if let Ok(()) = q_bar.get_mut(trigger.target()) {
        trigger.propagate(false);
        let distance_y = trigger.event().distance.y.abs();

        // Hysteresis bounds
        let open_threshold = 32.0;
        let close_threshold = 28.0;

        if distance_y > open_threshold && !*last_above_threshold {
            *last_above_threshold = true;
            if !*is_open {
                *is_open = true;
                println!("sending Action::Open");
                // event_writer.write(Action::Open);
                commands.trigger(UniversalSearchOpen);
            }
        } else if distance_y < close_threshold && *last_above_threshold {
            *last_above_threshold = false;
            if *is_open {
                *is_open = false;
                println!("sending Action::Close");
                // event_writer.write(Action::Close);
                commands.trigger(UniversalSearchClose);
            }
        }
        // If between close_threshold and open_threshold, do nothing and keep last state.
    }
}

pub fn on_bar_drag_end(mut trigger: Trigger<Pointer<DragEnd>>, mut q_bar: Query<(), With<Bar>>) {
    if let Ok(()) = q_bar.get_mut(trigger.target()) {
        trigger.propagate(false);
        let distance = trigger.event().distance;
        //Action::Open

        //Action::Close
    }
}
