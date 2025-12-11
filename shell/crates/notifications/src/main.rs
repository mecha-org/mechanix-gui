use desktop_dbus::MechanixNotificationService;
use futures::{channel::mpsc, select, SinkExt, StreamExt};
use gpui::*;
use commons::assets::Assets;
use notifications::prelude::{AppEvents, NotificationStory};
use notifications::notification_widget::{Notification, NotificationList};

fn main() {
    let application = gpui::Application::new().with_assets(Assets{});
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
                        let mut notification_received_stream = notification_service.stream_notification().await;

                        loop {
                            select! {
                                new_notification = notification_received_stream.next() => {
                                    if let Some(notification) = new_notification {
                                        println!("Notification received from: {:?}", notification.app_name);
                                        let _ = app_channel_tx.send(AppEvents::NotificationReceived { notification }).await;
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
                                AppEvents::NotificationReceived { notification } => {
                                    let title = format!("{}:{}",notification.app_name, notification.summary); // adapt fields to your type
                                    let body = notification.body.clone();

                                    let _ = list_for_events.update_in(cx, |list, window, cx| {
                                        list.push(
                                            Notification::new()
                                                .title(title.clone())
                                                .message(body.clone())
                                                .autohide(true),
                                            window,
                                            cx,
                                        );
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
