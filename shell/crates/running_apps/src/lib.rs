use crate::{
    models::models::{AppCard, RunningApps},
    prelude::app_manager::{AppManagerMessage, AppManagerService, AppMessage},
};
use dispatcher::Dispatcher;
use gpui::{
    layer_shell::{KeyboardInteractivity, LayerShellOptions},
    *,
};

use settings::prelude::{LayerShellSettings, RunningAppsSettings, Settings};
use tokio::sync::mpsc;

pub mod config;
pub mod models;
pub mod services;
mod ui;
pub mod prelude {
    // pub use crate::ui::*;
    pub use crate::config::*;
    pub use crate::models::*;
    pub use crate::services::*;
}

fn window_options(cx: &mut App) -> WindowOptions {
    let LayerShellSettings {
        size,
        namespace,
        layer,
        anchor,
        exclusive_zone,
        ..
    } = cx.global::<Settings>().running_apps.layer_shell.clone();

    let window_bounds = WindowBounds::Windowed(Bounds::new(point(px(0.), px(0.)), size));
    WindowOptions {
        window_bounds: Some(window_bounds),
        window_background: WindowBackgroundAppearance::Transparent,
        kind: WindowKind::LayerShell(LayerShellOptions {
            namespace,
            layer,
            anchor,
            keyboard_interactivity: KeyboardInteractivity::None,
            exclusive_zone: Some(exclusive_zone),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn root_view(
    message_tx: mpsc::Sender<AppManagerMessage>,
    app_channel_rx: mpsc::Receiver<AppMessage>,
) -> impl FnOnce(&mut Window, &mut App) -> Entity<RunningApps> {
    |window: &mut Window, cx: &mut App| {
        let settings = Settings::global(cx).running_apps.clone();

        let RunningAppsSettings {
            navbar_size,
            layer_shell,
            ..
        } = settings;

        let LayerShellSettings {
            size: window_size, ..
        } = layer_shell;

        let mut regions = Vec::new();

        regions.push(Bounds {
            origin: point(
                (window_size.width - navbar_size.width) / 2.,
                window_size.height - navbar_size.height,
            ),
            size: navbar_size,
        });
        window.set_input_regions(Some(regions));

        let running_apps = cx.new(|cx| {
            cx.spawn(
                async move |app: WeakEntity<RunningApps>, cx: &mut AsyncApp| {
                    sync_app_channel(app, cx, app_channel_rx).await
                },
            )
            .detach();
            RunningApps::new(message_tx)
        });
        running_apps
    }
}

pub fn run_app(app: &mut App) -> EntityId {
    let (message_tx, message_rx) = mpsc::channel(120);
    let (app_channel_tx, mut app_channel_rx) = mpsc::channel(120);
    let dispatcher_rx = Dispatcher::global(app).0.clone();

    let executor = app.background_executor();

    executor
        .spawn(run_app_manager(message_rx, app_channel_tx.clone()))
        .detach();

    executor
        .spawn(run_dispatcher_listener(dispatcher_rx, app_channel_tx))
        .detach();

    let window_options = window_options(app);

    let window = app
        .open_window(window_options, root_view(message_tx, app_channel_rx))
        .unwrap();
    let entity_id = window.entity(app).unwrap().entity_id();
    entity_id
}

fn run_app_manager(
    message_rx: tokio::sync::mpsc::Receiver<AppManagerMessage>,
    app_channel_tx: tokio::sync::mpsc::Sender<AppMessage>,
) -> impl Future<Output = ()> {
    async move {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                AppManagerService::new()
                    .run(message_rx, app_channel_tx)
                    .await;
            });
    }
}

fn run_dispatcher_listener(
    mut dispatcher_rx: dispatcher::prelude::Receiver<dispatcher::Message>,
    _app_channel_tx: tokio::sync::mpsc::Sender<AppMessage>,
) -> impl Future<Output = ()> {
    async move {
        while let Ok(message) = dispatcher_rx.try_recv() {
            match message {
                _ => {}
            }
        }
    }
}

async fn sync_app_channel(
    this: WeakEntity<RunningApps>,
    cx: &mut AsyncApp,
    mut app_channel_rx: mpsc::Receiver<AppMessage>,
) {
    while let Some(message) = app_channel_rx.recv().await {
        match message {
            AppMessage::AppsUpdated { apps, .. } => {
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

                let _ = this.update(
                    cx,
                    |this: &mut RunningApps, cx: &mut Context<'_, RunningApps>| {
                        let old_apps_count = this.apps.len();

                        this.apps = latest_apps;

                        // Only recalculate scroll if the number of apps changed
                        if old_apps_count != new_apps_count {
                            let last_index = if this.apps.is_empty() {
                                0
                            } else {
                                this.apps.len() - 1
                            };
                            let initial_offset = this.calculate_center_offset(last_index);

                            this.scroll_offset = initial_offset;
                            this.target_scroll_offset = initial_offset;
                            this.current_center_index = last_index;

                            // Reset animation/drag states
                            this.is_animating = false;
                            this.is_dragging = false;
                        }
                        // If count is the same, preserve current scroll position
                        // (scroll_offset, target_scroll_offset, current_center_index remain unchanged)

                        cx.notify();
                    },
                );
            }
            _ => (),
        }
    }
}
