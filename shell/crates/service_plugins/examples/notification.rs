use bevy::prelude::*;
use notification::notification::Notification;
use service_plugins::notification::{ NotificationEvent, NotificationPlugin };
use bevy::color::palettes::basic::RED;
use bevy::{ prelude::*, winit::WinitSettings };

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(NotificationPlugin)
        .add_systems(Update, print_notifications)
        .run();
}

fn print_notifications(mut event_reader: EventReader<NotificationEvent>) {
    for event in event_reader.read() {
        match event {
            NotificationEvent::Recieved(id, notification) => {
                info!("Received notification with ID: {} - {:?}", id, notification);
            }
            NotificationEvent::Closed(id) => {
                info!("Notification with ID: {} closed", id);
            }
            NotificationEvent::ActionInvoked(id, action_key) => {
                info!("Action invoked on notification ID: {} - Action Key: {}", id, action_key);
            }
        }
    }
}
