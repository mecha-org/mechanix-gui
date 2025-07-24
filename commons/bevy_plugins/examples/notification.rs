use bevy::color::palettes::basic::RED;
use bevy::{prelude::*, winit::WinitSettings};
use bevy_plugins::notification::{NotificationPlugin, NotificationEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NotificationPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, print_notifications)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn print_notifications(mut events: EventReader<NotificationEvent>) {
    for event in events.read() {
        match event {
            NotificationEvent::Recieved(id, notification) => {
                info!("Notification received: id={:?}, notification={:?}", id, notification);
            }
            NotificationEvent::Closed(id) => {
                info!("Notification closed: id={}", id);
            }
        }
    }
}
