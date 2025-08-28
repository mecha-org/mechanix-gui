use bevy::prelude::*;

use crate::components::Bar;

pub fn on_bar_drag_start(
    mut trigger: Trigger<Pointer<DragStart>>,
    mut q_bar: Query<(), With<Bar>>,
) {
    if let Ok(()) = q_bar.get_mut(trigger.target()) {
        trigger.propagate(false);
        println!("on_drag_start()");
    }
}

pub fn on_bar_drag(mut trigger: Trigger<Pointer<Drag>>, mut q_bar: Query<(), With<Bar>>) {
    if let Ok(()) = q_bar.get_mut(trigger.target()) {
        trigger.propagate(false);
        println!("on_drag()");
    }
}

pub fn on_bar_drag_end(mut trigger: Trigger<Pointer<DragEnd>>, mut q_bar: Query<(), With<Bar>>) {
    if let Ok(()) = q_bar.get_mut(trigger.target()) {
        trigger.propagate(false);
        let distance = trigger.event().distance;
    }
}
