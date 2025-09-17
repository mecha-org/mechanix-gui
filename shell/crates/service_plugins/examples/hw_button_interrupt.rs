use bevy::prelude::*;
use bevy::winit::WinitSettings;
use service_plugins::desktop::{DesktopPlugin, PowerButtonEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DesktopPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(
            Update,
            handle_desktop_event.run_if(resource_exists::<PowerButtonEvent>),
        )
        .run();
}

fn handle_desktop_event(power_interrupt_event: Res<PowerButtonEvent>) {
    println!("Power button event: {:?}", power_interrupt_event.0);
}
