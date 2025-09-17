use bevy::prelude::*;
use extensions::plugin::{ ExtensionPlugin, ExtensionEvent };
use extensions::device::Device;
use std::collections::HashMap;
use zbus::{zvariant::Value, Connection};
use tokio::runtime::Runtime;
use std::sync::Arc;

#[derive(Resource)]
struct TokioRuntime(Arc<Runtime>);

fn main() {
    // Create tokio runtime
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    let runtime_resource = TokioRuntime(Arc::new(rt));

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ExtensionPlugin)
        .insert_resource(runtime_resource)
        .add_systems(Update, handle_extension_events)
        .run();
}

fn handle_extension_events(
    mut device_interupt_events: EventReader<ExtensionEvent>,
    runtime: Res<TokioRuntime>,
) {
    for event in device_interupt_events.read() {
        match event {
            ExtensionEvent::Added(device) => {
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
                
                // Send desktop notification for device connection
                let device_clone = device.clone();
                let rt = runtime.0.clone();
                std::thread::spawn(move || {
                    rt.block_on(async {
                        if let Err(e) = send_connection_notification(&device_clone).await {
                            error!("Failed to send notification: {}", e);
                        }
                    });
                });
            },
            ExtensionEvent::Removed(device) => {
                info!("╭─────────────────────────────────────╮");
                info!("│  🔌 EXTENSION DEVICE DISCONNECTED  │");
                info!("╰─────────────────────────────────────╯");
                info!(" Name:        {}", device.name());
                info!(" Path:        {:?}", device.path());
                info!(" Unique ID:   {}", device.unique_id());
                info!("─────────────────────────────────────");
                
                // Send desktop notification for device disconnection
                let device_clone = device.clone();
                let rt = runtime.0.clone();
                std::thread::spawn(move || {
                    rt.block_on(async {
                        if let Err(e) = send_disconnection_notification(&device_clone).await {
                            error!("Failed to send notification: {}", e);
                        }
                    });
                });
            }
        }
    }
}

async fn send_connection_notification(device: &Device) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let connection = Connection::session().await?;
    
    let summary = "🔌 Extension Device Connected";
    let body = format!("Device: {}\nType: {:?}\nVendor: 0x{:04X}", 
                      device.name(), 
                      device.device_type(), 
                      device.vendor_id());
    
    let _reply = connection.call_method(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        Some("org.freedesktop.Notifications"),
        "Notify",
        &("mechanix-extensions", 0u32, "dialog-information", summary, &body,
          vec![""; 0], HashMap::<&str, &Value>::new(), 5000),
    ).await?;
    
    Ok(())
}

async fn send_disconnection_notification(device: &Device) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let connection = Connection::session().await?;
    
    let summary = "🔌 Extension Device Disconnected";
    let body = format!("Device: {}\nUnique ID: {}", 
                      device.name(), 
                      device.unique_id());
    
    let _reply = connection.call_method(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        Some("org.freedesktop.Notifications"),
        "Notify",
        &("mechanix-extensions", 0u32, "dialog-warning", summary, &body,
          vec![""; 0], HashMap::<&str, &Value>::new(), 4000),
    ).await?;
    
    Ok(())
}