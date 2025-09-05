use bevy::prelude::*;
use extensions::plugin::{ ExtensionPlugin, DeviceInteruptEvent };
use extensions::device::Device;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ExtensionPlugin)
        .add_systems(Update, handle_extension_events)
        .run();
}

fn handle_extension_events(mut device_interupt_events: EventReader<DeviceInteruptEvent>) {
    for event in device_interupt_events.read() {
        match event {
            DeviceInteruptEvent::Added(device) => {
                
                    info!("╭─────────────────────────────────────╮");
                    info!("│  🔌 EXTENSION DEVICE CONNECTED      │");
                    info!("╰─────────────────────────────────────╯");
                    info!("Name:        {}", device.name());
                    info!("Path:        {:?}", device.path());
                    info!("Type:        {:?}", device.device_type());
                    info!("Connection:  {:?}", device.connection_type());
                    info!("Vendor ID:   0x{:04X}", device.vendor_id());
                    info!("Unique ID:   {}", device.unique_id());
                    info!("─────────────────────────────────────");
                
            },
            DeviceInteruptEvent::Removed(device) => {
                
                    info!("╭─────────────────────────────────────╮");
                    info!("│  🔌 EXTENSION DEVICE DISCONNECTED  │");
                    info!("╰─────────────────────────────────────╯");
                    info!(" Name:        {}", device.name());
                    info!(" Path:        {:?}", device.path());
                    info!(" Unique ID:   {}", device.unique_id());
                    info!("─────────────────────────────────────");
            }
        }
    }
}
