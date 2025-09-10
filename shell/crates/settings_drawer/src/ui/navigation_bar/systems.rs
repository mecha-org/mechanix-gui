use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{
    events::{SettingsDrawerClose, SettingsDrawerOpen},
    icons::SettingsDrawerIcons,
    ui::{Bar, SettingsDrawerRoot, bar},
};

pub fn spawn_navigation_bar(
    mut commands: Commands,
    q_root: Single<Entity, With<SettingsDrawerRoot>>,
    icons: Res<SettingsDrawerIcons>,
) {
    let root = q_root.into_inner();

    commands
        .entity(root)
        .with_children(|parent: &mut RelatedSpawnerCommands<ChildOf>| {
            parent.spawn(bar(icons.right_nav_bar.clone()));
        });
}

pub fn on_bar_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut q_bar: Query<(), With<Bar>>,
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
                commands.trigger(SettingsDrawerOpen);
            }
        } else if distance_y < close_threshold && *last_above_threshold {
            *last_above_threshold = false;
            if *is_open {
                *is_open = false;
                println!("sending Action::Close");
                // event_writer.write(Action::Close);
                commands.trigger(SettingsDrawerClose);
            }
        }
        // If between close_threshold and open_threshold, do nothing and keep last state.
    }
}
