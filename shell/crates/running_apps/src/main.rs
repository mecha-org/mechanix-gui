use commons::prelude::*;
use gpui::prelude::*;
use gpui::*;
use running_apps::prelude::*;
use tokio::sync::mpsc;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds = WindowBounds::Windowed(
            Bounds::centered(None, size(px(540.0), px(620.0)), cx)
        );
        
        let (message_tx, message_rx) = mpsc::channel(120);
        let (app_channel_tx, mut app_channel_rx) = mpsc::channel(120);

        let running_apps_handle = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(window_bounds),
                    ..Default::default()
                },
                |_window, cx| cx.new(|_cx| RunningApps::new(message_tx))
            )
            .unwrap();

        cx.background_executor()
            .spawn(async move {
                tokio::runtime::Builder
                    ::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let mut app_manager = AppManagerService::new();
                        app_manager.run(message_rx, app_channel_tx).await;
                    });
            })
            .detach();

        cx.spawn(async move |cx| {
            println!("📬 Message receiver task started");
            let rhandle = running_apps_handle.clone();

            loop {
                match app_channel_rx.recv().await {
                    Some(message) => {
                        match message {
                            AppMessage::AppsUpdated { apps, app_id, active_apps_count } => {
                                let apps_clone = apps.clone();
                                let mut latest_apps: Vec<AppCard> = Vec::new();

                                for (i, app) in apps_clone.iter().enumerate() {
                                    latest_apps.push(AppCard {
                                        id: i,
                                        app_id: app.app_id.clone(),
                                        offset_y: px(0.0),
                                        target_offset_y: px(0.0),
                                        app_name: app.name.clone(),
                                        app_icon_path: app.icon_path.clone(),
                                    });
                                }

                                let new_apps_count = latest_apps.len();

                                let _ = rhandle.update(
                                    cx,
                                    |a: &mut RunningApps, _, c: &mut Context<'_, RunningApps>| {
                                        let old_apps_count = a.apps.len();

                                        a.apps = latest_apps;

                                        // Only recalculate scroll if the number of apps changed
                                        if old_apps_count != new_apps_count {
                                            let last_index = if a.apps.is_empty() {
                                                0
                                            } else {
                                                a.apps.len() - 1
                                            };
                                            let initial_offset =
                                                a.calculate_center_offset(last_index);

                                            a.scroll_offset = initial_offset;
                                            a.target_scroll_offset = initial_offset;
                                            a.current_center_index = last_index;

                                            // Reset animation/drag states
                                            a.is_animating = false;
                                            a.is_dragging = false;
                                        }
                                        // If count is the same, preserve current scroll position
                                        // (scroll_offset, target_scroll_offset, current_center_index remain unchanged)

                                        c.notify();
                                    }
                                );
                            }
                            _ => {}
                        }
                    }
                    None => {
                        println!("❌ app_channel closed, exiting loop");
                        break;
                    }
                }
            }
            println!("📬 Message receiver loop ended");
        }).detach();

        cx.activate(true);
    });
}
