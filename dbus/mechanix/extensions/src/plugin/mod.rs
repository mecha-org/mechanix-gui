use bevy::prelude::*;
use bevy::log::{ error, info };
use crate::proxy::reciever::ExtensionServiceReceiver;
use crate::events::ExtensionServiceEvent;
use crate::device::{ Device, types::DeviceType };
use bevy::tasks::{ AsyncComputeTaskPool, IoTaskPool };
use std::sync::mpsc::{ channel, Receiver, Sender };
use std::sync::{ Arc, Mutex };

#[derive(Event, Debug, Clone)]
pub enum DeviceInteruptEvent {
    Added(Device),
    Removed(Device),
}

#[derive(Resource)]
struct DeviceInteruptEventReceiver(pub Arc<Mutex<Receiver<DeviceInteruptEvent>>>);

#[derive(Resource)]
struct DeviceInteruptEventSender(pub Sender<DeviceInteruptEvent>);

fn write_device_interupt_events(
    receiver: Res<DeviceInteruptEventReceiver>,
    mut event_writer: EventWriter<DeviceInteruptEvent>
) {
    let receiver = receiver.0.lock().unwrap();
    while let Ok(event) = receiver.try_recv() {
        match event {
            DeviceInteruptEvent::Added(device) => {
                event_writer.write(DeviceInteruptEvent::Added(device));
            }
            DeviceInteruptEvent::Removed(device) => {
                event_writer.write(DeviceInteruptEvent::Removed(device));
            }
        }
    }
}

fn spawn_event_poller(mut commands: Commands) {
    let (tx, rx) = channel();
    let rx = Arc::new(Mutex::new(rx));

    // Spawn the async poller in a background thread
    std::thread::spawn({
        let tx = tx.clone();
        move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                match ExtensionServiceReceiver::new().await {
                    Ok((receiver, mut event_receiver)) => {
                        // Start the receiver service in a background task
                        let receiver_task = tokio::spawn(async move {
                            if let Err(e) = receiver.start_service().await {
                                error!("ExtensionService receiver failed: {}", e);
                            }
                        });

                        // Handle events from the receiver
                        let event_handler_task = tokio::spawn(async move {
                            info!("ExtensionService event handler started");

                            while let Some(event) = event_receiver.recv().await {
                                match event {
                                    ExtensionServiceEvent::Added(device) => {
                                        let _ = tx.send(DeviceInteruptEvent::Added(device));
                                    }
                                    ExtensionServiceEvent::Removed(device) => {
                                        let _ = tx.send(DeviceInteruptEvent::Removed(device));
                                    }
                                }
                            }

                            info!("ExtensionService event receiver stream ended");
                        });

                        // Wait for either task to complete
                        tokio::select! {
                            result = receiver_task => {
                                match result {
                                    Ok(_) => info!("ExtensionService receiver task completed"),
                                    Err(e) => error!("ExtensionService receiver task failed: {}", e),
                                }
                            }
                            result = event_handler_task => {
                                match result {
                                    Ok(_) => info!("ExtensionService event handler task completed"),
                                    Err(e) => error!("ExtensionService event handler task failed: {}", e),
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to create ExtensionServiceReceiver: {}", e);
                    }
                }
            });
        }
    });

    commands.insert_resource(DeviceInteruptEventReceiver(rx));
    commands.insert_resource(DeviceInteruptEventSender(tx));
}

pub struct ExtensionPlugin;

impl Plugin for ExtensionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeviceInteruptEvent>()
            .add_systems(Startup, spawn_event_poller)
            .add_systems(Update, (write_device_interupt_events));
    }
}
