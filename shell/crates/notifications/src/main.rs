use desktop_dbus::MechanixNotificationService;
use futures::{channel::mpsc, select, SinkExt, StreamExt};
use gpui::*;
use notifications::prelude::{AppEvents, StatusBar};

fn main() {
    let application = gpui::Application::new();
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
            |_window, cx| {
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

                cx.new(|cx| {
                    cx.spawn(async move |app, cx| {
                        while let Some(event) = app_channel_rx.next().await {
                            match event {
                                AppEvents::NotificationReceived { notification } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.notification_list = "".to_string();
                                        cx.notify();
                                    });
                                }
                            }
                        }
                    })
                    .detach();
                    StatusBar::new()
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
