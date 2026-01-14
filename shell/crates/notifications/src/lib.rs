use crate::events::AppEvents;
use crate::widgets::prelude::{
    DbNotification, NotificationCenter, NotificationList, NotificationUi, NotificationWidget,
    UserDismissedEvent,
};
use desktop_dbus::NotificationService;
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt, select};
use gpui::layer_shell::{KeyboardInteractivity, LayerShellOptions};
use gpui::{
    App, AppContext, Bounds, Context, ElementId, InteractiveElement, IntoElement, ParentElement,
    ReadGlobal, Render, SharedString, StatefulInteractiveElement, Styled,
    WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, div, point, px, rgb, svg,
};
use icons::prelude::Icons;
use settings::prelude::{LayerShellSettings, NotificationSettings, Settings};

mod events;
mod helper;
mod ui;
pub mod widgets;

pub mod prelude {
    pub use crate::events::AppEvents;
    pub use crate::ui::NotificationStory;
}

pub fn run_app(cx: &mut App) {
    let NotificationSettings {
        layer_shell,
        navbar_size,
        ..
    } = Settings::global(cx).notifications.clone();

    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
        ..
    } = layer_shell;

    let window_bounds = WindowBounds::Windowed(Bounds::centered(None, size, cx));

    cx.open_window(
        WindowOptions {
            window_bounds: Some(window_bounds),
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace,
                layer,
                anchor,
                keyboard_interactivity: KeyboardInteractivity::OnDemand,
                exclusive_zone: Some(exclusive_zone),
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
                    if let Ok(notification_service) = NotificationService::new().await {
                        if let Ok(all) = notification_service.fetch_all().await {
                            // Map HashMap<u32, Notification> -> Vec<DbNotification>
                            let mut vec_items: Vec<DbNotification> = Vec::new();
                            for (id, stored_notification) in all {
                                let notification  = stored_notification.notification;
                                let received_at = stored_notification.received_at;
                                let mut hints: std::collections::HashMap<String, String> =
                                    std::collections::HashMap::new();
                                if let Some(image) = notification.get_image() {
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
                                    app_name: notification.app_name.clone(),
                                    app_icon: notification.app_icon.clone(),
                                    summary: notification.summary.clone(),
                                    body: notification.body.clone(),
                                    actions: notification.actions.clone(),
                                    received_at: Some(received_at),
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
                    let notification_service = NotificationService::new().await.unwrap();

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
                                    match ui_event {
                                        Some(AppEvents::ActionInvoked { id, action_id }) => {
                                            // Forward the action invoke to the DBus notification service
                                            if let Err(err) = notification_service.send_action_invoke(id, &action_id).await {
                                                eprintln!("Failed to send action invoke for {} ({}): {:?}", id, action_id, err);
                                            }
                                        }
                                        Some(AppEvents::UserCloseNotification { id, reason }) => {
                                            // Forward the action invoke to the DBus notification service
                                            if let Err(err) = notification_service.close_notification(id, reason).await {
                                                eprintln!("Failed to send close notification for {} : {:?}", id, err);
                                            }
                                        }
                                        Some(other_event) => {
                                            // Optional: handle or ignore other AppEvents variants
                                            println!("Unhandled UI event: {:?}", other_event);
                                        }

                                        None => {
                                            // Channel closed
                                            println!("UI channel closed");
                                        }
                                    }

                                }
                            }
                    }
                })
                .detach();
            // Start a UI task on the window context that receives events from the background channel
            let list_for_events = notification_list.clone();

            cx.subscribe(&notification_list, move |_list_handle, event: &UserDismissedEvent, cx| {
                println!("USER DISMISSED EVENT RECEIVED: {}", event.id);
                let mut tx = ui_tx_for_ui_task.clone();
                let id = event.id;
                cx.background_executor()
                    .spawn(async move {
                        let _ = tx
                            .send(AppEvents::UserCloseNotification { id, reason: 2 })
                            .await;
                    })
                    .detach();
            }).detach();

            let ui_tx_for_center = ui_channel_tx.clone();
            cx.subscribe(&center, move |_center_handle, event: &UserDismissedEvent, cx| {
                println!("USER DISMISSED EVENT RECEIVED from a center: {}", event.id);
                let mut tx = ui_tx_for_center.clone();
                let id = event.id;
                cx.background_executor()
                    .spawn(async move {
                        let _ = tx
                            .send(AppEvents::UserCloseNotification { id, reason: 2 })
                            .await;
                    })
                    .detach();
            }).detach();

            // Clone the UI->backend sender into the UI task
            let ui_tx_for_ui_task = ui_channel_tx.clone();
            let center_for_visibility = center.clone();
            notification_list.update(cx, |_, cx| {
                cx.spawn_in(window, async move |_, cx| {
                    while let Some(event) = app_channel_rx.next().await {
                        match event {
                            AppEvents::NotificationReceived { id, notification } => {
                                let title = format!("{}: {}", notification.app_name, notification.summary); // adapt fields to your type
                                let body = notification.body.clone();
                                let key_ss: SharedString = id.to_string().into();
                                let key_id = ElementId::Name(key_ss);
                                let actions = notification.actions.clone();
                                let parsed_actions = helper::parse_actions(actions);
                                let mut icon_path: Option<std::path::PathBuf> = None;
                                if let Some(image) = notification.get_image() {
                                    if let Ok(opt_path) = image.resolve_path() {
                                        if let Some(path) = opt_path {
                                            icon_path = Some(path);
                                        }
                                    }
                                }
                                println!("ICON PATH: {:?}", icon_path);

                                let mut hints: std::collections::HashMap<String, String> =
                                    std::collections::HashMap::new();
                                if let Some(ref path) = icon_path {
                                    hints.insert(
                                        "image-path".to_string(),
                                        path.to_string_lossy().to_string(),
                                    );
                                }

                                let db_notif = DbNotification {
                                    id,
                                    app_name: notification.app_name.clone(),
                                    app_icon: notification.app_icon.clone(),
                                    summary: notification.summary.clone(),
                                    body: notification.body.clone(),
                                    actions: notification.actions.clone(),
                                    received_at: Some(helper::epoch_seconds()),
                                    hints,
                                };

                                let center_for_visibility = center_for_visibility.clone();
                                if center_for_visibility.read_with(cx, |center, _| center.is_visible()).unwrap_or(false) {
                                    let _ = center_for_visibility.update(cx, |center, cx| {
                                        center.add_db_notification(db_notif, cx);
                                    });
                                    continue;
                                }

                                // Clone into a separate variable that we can move into the UI closure
                                let actions_pairs = parsed_actions.clone();
                                // Capture id and a sender clone for click handlers
                                let notif_id = id;
                                let ui_tx_buttons = ui_tx_for_ui_task.clone();
                                let center_for_click = center_for_visibility.clone();
                                let _ = list_for_events.update_in(cx, |list, window, cx| {

                                    list.push(
                                        {
                                            let base = NotificationUi::new()
                                                .id1::<NotificationUi>(key_id)
                                                .db_id(notif_id)
                                                .db_notification(db_notif)
                                                .title(title.clone())
                                                .on_click(move |_event, _window, cx| {
                                                    println!("Notification clicked: {}", id);
                                                    // let _ = center_for_click.update(cx, |center, cx| {
                                                    //     // center.snap_to(0.0, cx);
                                                    // });
                                                })
                                                .message(body.clone())
                                                .autohide(helper::should_auto_hide(notification.expire_timeout))
                                                .expire_timeout(notification.get_expire_timeout());


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
                                            .action(move |_, _, notif_cx| {
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
                                                let icons = Icons::global(notif_cx).notifications.clone();
                                                let default_icon: SharedString = icons.application.to_string_lossy().to_string().into();
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
                                                            .on_click(notif_cx.listener(move |this, _, window, cx| {
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
                                                            .child(svg().external_path(&default_icon).w(px(18.)).h(px(18.)).text_color(if is_last { rgb(0xff9500) } else { rgb(0xe9e9e9) }))
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
                                println!("Close notification signal received: {}", id);

                                // Close in the toast list
                                let _ = list_for_events.update_in(cx, |list, window, cx| {
                                    let key_ss: SharedString = id.to_string().into();
                                    let key_id = ElementId::Name(key_ss);
                                    let _ = list.close_by_key(key_id, id, window, cx);
                                });

                                // Also close in the Notification Center
                                let _ = center_for_visibility.update(cx, |center, cx| {
                                    center.remove_db_notification(id, cx);
                                });
                            }
                            // UI side should not receive ActionInvoked, but handle gracefully
                            AppEvents::ActionInvoked { .. } => {}
                            AppEvents::UserCloseNotification { .. } => {}
                        }
                    }
                }).detach();
            });
            // Mount both by returning an Entity root that composes them.
            // This avoids returning a raw Div (which isn't an Entity) from the window builder.
            {
                let mut regions = Vec::new();
                regions.push(Bounds {
                    origin: point(px(0.), size.height - navbar_size.height),
                    size: gpui::size(navbar_size.width, navbar_size.height),
                });
                window.set_input_regions(Some(regions));
                cx.new(|cx| NotificationWidget::new(center, notification_list, cx))
            }

        },
    ).unwrap();
}
