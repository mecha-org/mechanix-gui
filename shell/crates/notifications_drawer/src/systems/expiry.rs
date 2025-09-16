use bevy::prelude::*;
use std::time::{SystemTime, Duration};

#[derive(Component)]
pub struct NotificationTimestamp {
    pub created_at: SystemTime,
}

#[derive(Resource)]
pub struct TimestampUpdateTimer(pub Timer);

pub fn init_timestamp_timer(mut commands: Commands) {
    commands.insert_resource(TimestampUpdateTimer(Timer::from_seconds(60.0, TimerMode::Repeating)));
}

pub fn format_time_elapsed(created_at: SystemTime) -> String {
    let now = SystemTime::now();
    if let Ok(elapsed) = now.duration_since(created_at) {
        let secs = elapsed.as_secs();
        if secs < 60 {
            "now".to_string()
        } else if secs < 3600 {
            let mins = secs / 60;
            format!("{}m", mins)
        } else if secs < 86400 {
            let hours = secs / 3600;
            format!("{}h", hours)
        } else {
            let days = secs / 86400;
            format!("{}d", days)
        }
    } else {
        "now".to_string()
    }
}

pub fn update_notification_timestamps(
    mut timestamp_query: Query<(&NotificationTimestamp, &Children)>,
    mut text_query: Query<&mut Text>,
    time: Res<Time>,
    mut timer: ResMut<TimestampUpdateTimer>
) {
    // Update every 30 seconds to avoid excessive updates
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for (timestamp, children) in timestamp_query.iter_mut() {
        // Find the text node among the children and update it
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                **text = format_time_elapsed(timestamp.created_at);
                break;
            }
        }
    }
}