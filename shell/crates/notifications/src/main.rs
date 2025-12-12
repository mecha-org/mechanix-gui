use commons::assets::Assets;
use desktop_dbus::MechanixNotificationService;
use futures::{channel::mpsc, select, SinkExt, StreamExt};
use gpui::*;
use notifications::notification_widget::{Notification, NotificationList};
use notifications::prelude::{AppEvents, NotificationStory};

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(0.0), px(310.0)), cx));
        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                kind: WindowKind::LayerShell(layer_shell::LayerShellOptions {
                    namespace: "mechanix.notifications".to_string(),
                    layer: layer_shell::Layer::Top,
                    anchor: layer_shell::Anchor::RIGHT
                        | layer_shell::Anchor::LEFT
                        | layer_shell::Anchor::TOP,
                    exclusive_zone: Some(px(36.0)),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let (mut app_channel_tx, mut app_channel_rx) = mpsc::channel::<AppEvents>(120);
                let executor = cx.background_executor();

                executor
                    .spawn(async move {
                        let notification_service = MechanixNotificationService::new().await.unwrap();
                        let mut notification_received_stream = notification_service.stream_receive_notification().await;
                        let mut notification_closed_stream = notification_service.stream_close_notification().await;

                        loop {
                            select! {
                                new_notification = notification_received_stream.next() => {
                                    if let Some((id, notification)) = new_notification {
                                        println!("Notification received from: {:?}", notification.app_name);
                                        let _ = app_channel_tx.send(AppEvents::NotificationReceived { id, notification }).await;
                                    }
                                },
                                closed_notification = notification_closed_stream.next() => {
                                    println!("Notification closed: {:?}", closed_notification);
                                    if let Some(notification_id) = closed_notification {
                                        let _ = app_channel_tx.send(AppEvents::CloseNotification { id: notification_id }).await;
                                    }
                                },
                            }
                        }
                    })
                    .detach();


                // inside the window builder closure
                let notification_list = cx.new(|cx| NotificationList::new(window, cx));

                // Start a UI task on the window context that receives events from the background channel
                let list_for_events = notification_list.clone();
                notification_list.update(cx, |_, cx| {
                    cx.spawn_in(window, async move |_, cx| {
                        while let Some(event) = app_channel_rx.next().await {
                            match event {
                                AppEvents::NotificationReceived { id, notification } => {
                                    let title = format!("{}:{}", notification.app_name, notification.summary); // adapt fields to your type
                                    let body = notification.body.clone();
                                    let key_ss: SharedString = id.to_string().into();
                                    let key_id = ElementId::Name(key_ss);

                                    let _ = list_for_events.update_in(cx, |list, window, cx| {
                                        list.push(
                                            Notification::new()
                                                .id1::<Notification>(key_id)
                                                .title(title.clone())
                                                .message(body.clone())
                                                .autohide(false),
                                            window,
                                            cx,
                                        );
                                    });
                                }
                                AppEvents::CloseNotification { id } => {
                                    let _ = list_for_events.update_in(cx, |list, window, cx| {
                                        let key_ss: SharedString = id.to_string().into();
                                        let key_id = ElementId::Name(key_ss);
                                        let _ = list.close_by_key(key_id, window, cx);
                                    });
                                }
                            }
                        }
                    }).detach();
                });

                cx.new(|_| NotificationStory::new(notification_list))
            },
        )
            .unwrap();
        cx.activate(true);
    });
}
