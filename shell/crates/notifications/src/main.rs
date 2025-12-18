use commons::assets::Assets;
use desktop_dbus::MechanixNotificationService;
use futures::{channel::mpsc, select, FutureExt, SinkExt, StreamExt};
use gpui::*;
use notifications::notification_widget::{Notification, NotificationList, NotificationCenter, DbNotification};
use notifications::prelude::icon::{Icon, IconName};
use notifications::prelude::AppEvents;

// Root view to compose NotificationCenter (background) and the toast NotificationList (overlay)
struct Root {
    center: Entity<NotificationCenter>,
    list: Entity<NotificationList>,
}

impl Root {
    fn new(center: Entity<NotificationCenter>, list: Entity<NotificationList>) -> Self {
        Self { center, list }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Put the toast list last so it sits on top (its own render positions it absolutely)
        div()
            .child(self.center.clone())
            .child(self.list.clone())
    }
}

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(0.0), px(0.0)), cx));
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
                // Channel for UI -> backend events (e.g., action invoked)
                let (ui_channel_tx, mut ui_channel_rx) = mpsc::channel::<AppEvents>(120);

                // Load unread notifications
                let notification_list = cx.new(|cx| NotificationList::new(window, cx));
                // Create the NotificationCenter entity here so we can update it later
                let center = cx.new(|cx| NotificationCenter::new(window, cx));

                // Keep a clone to update center from the background task
                let center_for_fetch = center.clone();
                let ui_tx_for_ui_task = ui_channel_tx.clone();

                // Fetch unread notifications by spawning a UI-bound task using the center's Context
                let _ = center.update(cx, |_, cx: &mut Context<NotificationCenter>| {
                    cx.spawn_in(window, async move |_, cx| {
                        if let Ok(notification_service) = MechanixNotificationService::new().await {
                            if let Ok(all) = notification_service.fetch_all().await {
                                // Map HashMap<u32, Notification> -> Vec<DbNotification>
                                let mut vec_items: Vec<DbNotification> = Vec::new();
                                for (id, n) in all {
                                    let mut hints: std::collections::HashMap<String, String> =
                                        std::collections::HashMap::new();
                                    if let Some(image) = n.get_image() {
                                        if let Ok(opt_path) = image.resolve_path() {
                                            if let Some(path) = opt_path {
                                                hints.insert(
                                                    "image-path".to_string(),
                                                    path.to_string_lossy().to_string(),
                                                );
                                            }
                                        }
                                    }
                                    vec_items.push(DbNotification {
                                        id,
                                        app_name: n.app_name.clone(),
                                        app_icon: n.app_icon.clone(),
                                        summary: n.summary.clone(),
                                        body: n.body.clone(),
                                        actions: n.actions.clone(),
                                        hints,
                                    });
                                }

                                let _ = center_for_fetch.update(cx, |center: &mut NotificationCenter, cx: &mut Context<NotificationCenter>| {
                                    center.load_from_database(vec_items, cx);
                                });
                            }
                        }
                    })
                    .detach();
                });

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
                                        println!("Notification received from: {:?}", notification.app_icon);
                                        let _ = app_channel_tx.send(AppEvents::NotificationReceived { id, notification }).await;
                                    }
                                },
                                closed_notification = notification_closed_stream.next() => {
                                    println!("Notification closed: {:?}", closed_notification);
                                    if let Some(notification_id) = closed_notification {
                                        let _ = app_channel_tx.send(AppEvents::CloseNotification { id: notification_id }).await;
                                    }
                                },
                                ui_event = ui_channel_rx.next() => {
                                    if let Some(AppEvents::ActionInvoked { id, action_id }) = ui_event {
                                        // Forward the action invoke to the DBus notification service
                                        if let Err(err) = notification_service.send_action_invoke(id, &action_id).await {
                                            eprintln!("Failed to send action invoke for {} ({}): {:?}", id, action_id, err);
                                        }
                                    }
                                }
                            }
                        }
                    })
                    .detach();


                // Start a UI task on the window context that receives events from the background channel
                let list_for_events = notification_list.clone();
                // Clone the UI->backend sender into the UI task
                let ui_tx_for_ui_task = ui_channel_tx.clone();
                notification_list.update(cx, |_, cx| {
                    cx.spawn_in(window, async move |_, cx| {
                        while let Some(event) = app_channel_rx.next().await {
                            match event {
                                AppEvents::NotificationReceived { id, notification } => {
                                    let title = format!("{}:{}", notification.app_name, notification.summary); // adapt fields to your type
                                    let body = notification.body.clone();
                                    let key_ss: SharedString = id.to_string().into();
                                    let key_id = ElementId::Name(key_ss);
                                    let actions = notification.actions.clone();
                                    let parsed_actions = parse_actions(actions);
                                    let mut icon_path: Option<std::path::PathBuf> = None;
                                    if let Some(image) = notification.get_image() {
                                        if let Ok(opt_path) = image.resolve_path() {
                                            if let Some(path) = opt_path {
                                                icon_path = Some(path);
                                            }
                                        }
                                    }
                                    println!("ICON PATH: {:?}",icon_path);
                                    // Clone into a separate variable that we can move into the UI closure
                                    let actions_pairs = parsed_actions.clone();
                                    // Capture id and a sender clone for click handlers
                                    let notif_id = id;
                                    let ui_tx_buttons = ui_tx_for_ui_task.clone();
                                    let _ = list_for_events.update_in(cx, |list, window, cx| {
                                        list.push(
                                            {
                                                let base = Notification::new()
                                                    .id1::<Notification>(key_id)
                                                    .title(title.clone())
                                                    .on_click(move |event, window, cx| {
                                                        println!("Notification clicked: {}", id);
                                                    })
                                                    .message(body.clone())
                                                    .autohide(false);

                                                // Only set icon when we have a path
                                                let base = if let Some(ref path) = icon_path {
                                                    base.icon_img(path.clone())
                                                } else {
                                                    base
                                                };

                                                base
                                            }
                                                // .icon(icon_el)
                                                // .with_type(NotificationType::Application)
                                                .action(move |_, _, cx| {
                                                    // Render an actions bar similar to the mock:
                                                    // a top border and evenly-spaced action cells with optional icons.
                                                    let mut row = div()
                                                        .id("actions-row")
                                                        .mt_2()
                                                        .pt_2()
                                                        .border_t_1()
                                                        .border_color(rgb(0xff9500))
                                                        .flex()
                                                        .flex_row()
                                                        .items_center();

                                                    let items: Vec<(String, String)> = actions_pairs.clone();
                                                    let total = items.len();
                                                    for (idx, (action_id, action_label)) in items.into_iter().enumerate() {
                                                        let is_last = idx + 1 == total;
                                                        let btn_id = format!("action-{}", action_id);
                                                        let action_id_move = action_id.clone();
                                                        let btn_eid = ElementId::Name(SharedString::from(btn_id));
                                                        let ui_tx_click = ui_tx_buttons.clone();
                                                        let clicked_notif_id = notif_id;

                                                        // Each cell is flex_1 and centered
                                                        row = row.child(
                                                            div()
                                                                .id(btn_eid)
                                                                .flex_1()
                                                                .flex()
                                                                .flex_row()
                                                                .justify_center()
                                                                .items_center()
                                                                .gap_2()
                                                                .py_2()
                                                                .text_color(if is_last { rgb(0xff9500) } else { rgb(0xe9e9e9) })
                                                                .on_click(cx.listener(move |this, _, window, cx| {
                                                                    println!(
                                                                        "Action '{}' clicked for notification {}",
                                                                        action_id_move, clicked_notif_id
                                                                    );
                                                                    let mut tx = ui_tx_click.clone();
                                                                    let action_id_to_send = action_id_move.clone();
                                                                    cx.spawn_in(window, async move |_, _| {
                                                                        let _ = tx.send(AppEvents::ActionInvoked { id: clicked_notif_id, action_id: action_id_to_send }).await;
                                                                    }).detach();
                                                                    this.dismiss(window, cx);
                                                                }))
                                                                // Placeholder icon, will be replaced by designer
                                                                .child(Icon::new(IconName::Application).size((px(18.), px(18.))).text_color(if is_last { rgb(0xff9500) } else { rgb(0xe9e9e9) }))
                                                                .child(action_label.clone())
                                                        );

                                                        // Add a vertical divider between cells (not after the last)
                                                        if !is_last {
                                                            row = row.child(
                                                                div()
                                                                    .w(px(1.0))
                                                                    .h(px(24.0))
                                                                    .bg(rgb(0x3a3a3a))
                                                            );
                                                        }
                                                    }
                                                    row
                                                }),
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
                                // UI side should not receive ActionInvoked, but handle gracefully
                                AppEvents::ActionInvoked { .. } => {}
                            }
                        }
                    }).detach();
                });
                // Mount both by returning an Entity root that composes them.
                // This avoids returning a raw Div (which isn't an Entity) from the window builder.
                {
                    cx.new(|_| Root::new(center, notification_list))
                }
            },
        )
            .unwrap();
        cx.activate(true);
    });
}

fn parse_actions(actions: Vec<String>) -> Vec<(String, String)> {
    // Parse into pairs
    let parsed_actions: Vec<(String, String)> = actions
        .chunks(2)
        .filter_map(|c| {
            if c.len() == 2 {
                Some((c[0].clone(), c[1].clone()))
            } else {
                None
            }
        })
        .collect();
    parsed_actions
}
