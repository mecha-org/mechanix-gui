use commons::prelude::*;
use futures::StreamExt;
use futures::channel::mpsc;

use gpui::*;
use settings_drawer::prelude::*;
use settings_drawer::services::*;

fn main() {
    let application = gpui::Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(540.0), px(620.0)), cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                ..Default::default()
            },
            |_window, cx| {
                let (app_channel_tx, mut app_channel_rx) = mpsc::channel::<AppEvents>(120);
                let (nm_tx, nm_rx) = mpsc::channel::<NmEvents>(128);
                let (bt_tx, bt_rx) = mpsc::channel::<BtEvents>(128);
                let (volume_tx, volume_rx) = mpsc::channel::<VolumeEvents>(128);
                let (brightness_tx, brightness_rx) = mpsc::channel::<BrightnessEvents>(128);
                let executor = cx.background_executor();

                executor
                    .spawn(sync_battery_state(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(sync_battery_level(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(sync_battery_percentage(app_channel_tx.clone()))
                    .detach();

                executor
                    .spawn(sync_network_status(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(sync_network_strength(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(stream_network_device_events(app_channel_tx.clone()))
                    .detach();
                executor.spawn(handle_wireless_toggle(nm_rx)).detach();

                executor
                    .spawn(sync_bluetooth_status(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(stream_bluetooth_device_status(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(sync_bluetooth_connected_status(app_channel_tx.clone()))
                    .detach();
                executor.spawn(handle_bluetooth_toggle(bt_rx)).detach();

                executor
                    .spawn(sound_device_events(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(handle_volume_change(volume_rx, app_channel_tx.clone()))
                    .detach();

                executor
                    .spawn(get_brightness_value(app_channel_tx.clone()))
                    .detach();
                executor
                    .spawn(handle_brightness_change(app_channel_tx.clone(), brightness_rx))
                    .detach();

                cx.new(|cx| {
                    cx.spawn(async move |app, cx| {
                        while let Some(event) = app_channel_rx.next().await {
                            match event {
                                AppEvents::BatteryStateChanged { state } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.battery_state = state;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BatteryLevelChanged { level } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.battery_level = level;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BatteryPercentageChanged { value } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.battery_percent = value;
                                        cx.notify();
                                    });
                                }
                                AppEvents::WirelessStatusChanged { enabled } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.wireless_details.enabled = enabled;
                                        cx.notify();
                                    });
                                }
                                AppEvents::WirelessStrength { strength } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.wireless_details.strength = strength;
                                        cx.notify();
                                    });
                                }
                                AppEvents::ConnectedNetwork { network } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.wireless_details.connected_network = network;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BluetoothEnabled { enabled } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.bluetooth_details.enabled = enabled;
                                        cx.notify();
                                    });
                                }
                                AppEvents::BluetoothDevices { count } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.bluetooth_details.devices = count;
                                        cx.notify();
                                    });
                                }
                                AppEvents::OutputSoundDevice { device_info } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.sound_device = Some(device_info);
                                        cx.notify();
                                    });
                                }
                                AppEvents::Brightness { value } => {
                                    let _ = app.update(cx, |this: &mut SettingsDrawer, cx| {
                                        this.brightness_dbus_value = value;
                                        cx.notify();
                                    });
                                }
                            }
                        }
                    })
                    .detach();

                    SettingsDrawer::new(cx, nm_tx.clone(), bt_tx.clone(), volume_tx.clone(), brightness_tx.clone())
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
