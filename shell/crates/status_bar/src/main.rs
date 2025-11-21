use commons::prelude::*;
use futures::{StreamExt, channel::mpsc};
use gpui::*;
use status_bar::prelude::*;
use status_bar::services::*;
use std::time::Duration;
use upower::interfaces::device::BatteryState;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(540.0), px(36.0)), cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                ..Default::default()
            },
            |_window, cx| {
                let (app_channel_tx, mut app_channel_rx) = mpsc::channel::<AppEvents>(120);
                let executor = cx.background_executor();

                executor
                    .spawn(sync_network_status(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(sync_network_strength(app_channel_tx.clone()))
                    .detach();

                executor
                    .spawn(sync_bluetooth_status(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(sync_bluetooth_connected_status(app_channel_tx.clone()))
                    .detach();

                executor
                    .spawn(sync_battery_state(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(sync_battery_level(app_channel_tx.clone()))
                    .detach();

                cx.new(|cx| {
                    cx.spawn(async move |app, cx| {
                        while let Some(event) = app_channel_rx.next().await {
                            match event {
                                AppEvents::WirelessStatusChanged { enabled } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.wireless_enabled = enabled;
                                        cx.notify();
                                    });
                                }
                                AppEvents::WirelessStrength { strength } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.wireless_strength = strength;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BluetoothEnabled { enabled } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.bluetooth_enabled = enabled;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BluetoothConnectionStatus { connected } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.bluetooth_connected = connected;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BatteryStateChanged { state } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.battery_state = match state {
                                            BatteryState::Charging => BatteryState::Charging,
                                            BatteryState::Discharging => BatteryState::Discharging,
                                            _ => BatteryState::Unknown,
                                        };
                                        cx.notify();
                                    });
                                }
                                AppEvents::BatteryLevelChanged { level } => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.battery_level = level;
                                        cx.notify();
                                    });
                                }
                                AppEvents::TimeUpdated => {
                                    let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                        this.update_time(cx);
                                    });
                                }
                            }
                        }
                    })
                    .detach();

                    cx.spawn(async move |app, cx| {
                        loop {
                            cx.background_executor().timer(Duration::from_secs(1)).await;
                            let _ = app.update(cx, |this: &mut StatusBar, cx| {
                                this.update_time(cx);
                            });
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
