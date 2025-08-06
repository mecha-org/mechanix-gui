use bevy::prelude::*;
use bevy::log::{ error, info };
use std::collections::HashMap;
use freedesktop_notifications_server::proxies::mechanix::{ MechanixNotificationProxy };
use freedesktop_notifications_server::notification::Notification;
use freedesktop_notifications_server::database::get_all_notifications_from_db;
use zbus::Connection;
use std::sync::{ Arc, Mutex };
use tokio::select;
use std::sync::mpsc::{ channel, Receiver, Sender };
use futures_util::stream::StreamExt;
use bevy::tasks::{ AsyncComputeTaskPool, IoTaskPool };

#[derive(Resource)]
pub struct AllNotificationsResource(HashMap<u32, Notification>);

#[derive(Event, Debug, Clone)]
pub enum NotificationEvent {
    Recieved(u32, Notification),
    Closed(u32),
    ActionInvoked(u32, String), // id, action_id
}

#[derive(Resource)]
pub struct NotificationEventReceiver(pub Arc<Mutex<Receiver<NotificationEvent>>>);

#[derive(Resource)]
pub struct NotificationEventSender(pub Sender<NotificationEvent>);

// fn load_notifications_from_database(mut commands: Commands,  mut event_writer: EventWriter<NotificationEvent>) {
//     let (tx, rx) = std::sync::mpsc::channel();
//     IoTaskPool::get()
//         .spawn(async move {
//             let notifications = get_all_notifications_from_db().await.expect(
//                 "Notification Fetching from database failed"
//             );
//             let _ = tx.send(notifications);
//         })
//         .detach();

//     if let Ok(notifications) = rx.try_recv() {
//         for (id,notification) in notifications{
//             event_writer.send(NotificationEvent::Recieved(id, notification.clone()));
//         }
//     }
// }

fn apply_loaded_notifications(
    mut notification_resource: ResMut<AllNotificationsResource>,
    receiver: Res<NotificationEventReceiver>,
    mut event_writer: EventWriter<NotificationEvent> // Added
) {
    let receiver = receiver.0.lock().unwrap();
    while let Ok(event) = receiver.try_recv() {
        match event {
            NotificationEvent::Recieved(id, notification) => {
                notification_resource.0.insert(id.clone(), notification.clone());
                println!("Notification Recieved");
                // You can emit a new event here if needed:
                event_writer.send(NotificationEvent::Recieved(id, notification.clone()));
            }
            NotificationEvent::Closed(id) => {
                notification_resource.0.remove(&id);
                println!("Notification Closed");
                // You can emit a new event here if needed:
                event_writer.send(NotificationEvent::Closed(id));
            }
            _ => {}
        }
    }
}

fn spawn_notification_poller(mut commands: Commands) {
    let (tx, rx) = channel();
    let rx = Arc::new(Mutex::new(rx));

    // Spawn the async poller in a background thread
    std::thread::spawn({
        let tx = tx.clone();
        move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                use freedesktop_notifications_server::proxies::mechanix::MechanixNotificationProxy;
                use zbus::Connection;
                use futures_util::stream::StreamExt;
                use tokio::select;

                let connection = Connection::session().await.unwrap();
                let proxy = MechanixNotificationProxy::new(&connection).await.unwrap();

                let mut received_stream = proxy.receive_notification_received().await.unwrap();
                let mut closed_stream = proxy.receive_notification_closed().await.unwrap();

                loop {
                    select! {
                        msg = received_stream.next() => {
                            if let Some(signal) = msg {
                                match signal.args() {
                                    Ok(args) => {
                                        let _ = tx.send(NotificationEvent::Recieved(args.id, args.notification.clone()));
                                    }
                                    Err(e) => eprintln!("Error parsing notification_received: {}", e),
                                }
                            } else {
                                break;
                            }
                        }
                        msg = closed_stream.next() => {
                            if let Some(signal) = msg {
                                match signal.args() {
                                    Ok(args) => {
                                        let _ = tx.send(NotificationEvent::Closed(args.id));
                                    }
                                    Err(e) => eprintln!("Error parsing notification_closed: {}", e),
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            });
        }
    });

    commands.insert_resource(NotificationEventReceiver(rx));
    commands.insert_resource(NotificationEventSender(tx));
}

fn handle_action_invoked(mut event_reader: EventReader<NotificationEvent>) {
    for event in event_reader.read() {
        if let NotificationEvent::ActionInvoked(id, action_id) = event {
            let id = *id;
            let action_id = action_id.clone();

            // Use AsyncComputeTaskPool instead of IoTaskPool for async operations
            AsyncComputeTaskPool::get()
                .spawn(async move {
                    // Create a new tokio runtime for this task
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async move {
                        match Connection::session().await {
                            Ok(connection) => {
                                match MechanixNotificationProxy::new(&connection).await {
                                    Ok(proxy) => {
                                        if let Err(e) = proxy.invoke_action(id, &action_id).await {
                                            error!("Failed to invoke action {} for notification {}: {}", action_id, id, e);
                                        } else {
                                            info!("Successfully invoked action {} for notification {}", action_id, id); 
                                        }
                                    }
                                    Err(e) => error!("Failed to create proxy: {}", e),
                                }
                            }
                            Err(e) => error!("Failed to connect to session bus: {}", e),
                        }
                    });
                })
                .detach();
        }
    }
}

pub struct NotificationPlugin;

impl Plugin for NotificationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AllNotificationsResource(HashMap::new()))
            .add_event::<NotificationEvent>()
            .add_systems(Startup, spawn_notification_poller)
            // .add_systems(Startup, load_notifications_from_database)
            .add_systems(Update, (apply_loaded_notifications, handle_action_invoked));
    }
}
