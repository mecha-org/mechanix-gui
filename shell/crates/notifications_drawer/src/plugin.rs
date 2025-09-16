use bevy::prelude::*;
use crate::components::cards::*;
use crate::components::surface::notification_stacks_init;
use crate::systems::{setup::setup, setup::exit_on_esc, expiry::*, button_system::*, notification_processing::*};

pub struct NotificationDrawerPlugin;

impl Plugin for NotificationDrawerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_count)
            .add_systems(Update, process_notifications)
            .add_systems(Update, stack_button_system)
            .add_systems(Update, clear_button_system)
            .add_systems(Update, card_close_button_system)
            .add_systems(Update, card_unstack_on_click_system)
            .add_systems(Update, action_button_system)
            .add_systems(Startup, notification_stacks_init)
            .add_systems(Startup, setup)
            .add_systems(Update, exit_on_esc)
            .add_systems(Update, clear_all_button_system)
            .add_systems(Update, update_notification_timestamps)
            .add_systems(Startup, init_timestamp_timer)
            .init_resource::<AppStackingState>();
    }
}
