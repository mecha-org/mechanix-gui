pub mod constants;
pub mod helper;
mod ui;

use gpui::{
    layer_shell::{KeyboardInteractivity, LayerShellOptions},
    *,
};
use settings::prelude::*;
use shell_state::ShellState;
use ui::*;

pub mod prelude {
    pub use crate::constants::*;
    pub use crate::run_app;
    pub use crate::ui::SettingsDrawer;
}

pub fn run_app(cx: &mut App) {
    let SettingsDrawerSettings {
        layer_shell,
        navbar_size,
    } = Settings::global(cx).settings_drawer.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
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
                keyboard_interactivity: KeyboardInteractivity::None,
                exclusive_zone: Some(exclusive_zone),
                ..Default::default()
            }),
            ..Default::default()
        },
        |window, cx| {
            let mut regions = Vec::new();
            regions.push(Bounds {
                origin: point(
                    size.width - navbar_size.width,
                    size.height - navbar_size.height,
                ),
                size: gpui::size(navbar_size.width, navbar_size.height),
            });
            window.set_input_regions(Some(regions));

            cx.new(|cx| {
                cx.observe_global::<ShellState>(|this: &mut SettingsDrawer, cx| {
                    let ShellState {
                        current_time_date,
                        wireless_details,
                        bluetooth_details,
                        battery_percent,
                        sound_device_info,
                        nm_tx,
                        bt_tx,
                        volume_tx,
                        brightness_tx,
                        ..
                    } = ShellState::global(cx).clone();

                    this.current_time_date = current_time_date;
                    this.battery_percent = battery_percent;
                    this.wireless_details.enabled = wireless_details.enabled;
                    this.wireless_details.connected_network = wireless_details.connected_network;

                    let mut sorted_list = wireless_details.networks.unwrap_or_else(|| vec![]);
                    sorted_list.sort_by_key(|n| (!n.is_active, !n.is_known));
                    sorted_list.retain(|n| !n.ssid.is_empty());
                    this.wireless_details.networks = Some(sorted_list);

                    this.bluetooth_details.enabled = bluetooth_details.enabled;
                    this.bluetooth_details.connected_devices = bluetooth_details.connected_devices;
                    this.bluetooth_details.available_devices =
                        bluetooth_details.available_devices.clone();

                    let sound_device = Some(sound_device_info).clone().unwrap();
                    this.volume_device_name = sound_device.name;
                    this.volume_mute = sound_device.mute;
                    this.volume_slider_value = if this.volume_mute { 0.0 } else { sound_device.volume as f32 };
                    this.volume_slider_state.update(cx, |state, _cx| {
                        state.value = this
                            .volume_slider_value
                            .clamp(state.min, state.max);
                    });

                    this.brightness_slider_value = ShellState::global(cx).brightness_value;
                    this.brightness_slider_state.update(cx, |state, _cx| {
                        state.value = this
                            .brightness_slider_value
                            .clone()
                            .clamp(state.min, state.max);
                    });

                    this.nm_tx = nm_tx;
                    this.bt_tx = bt_tx;
                    this.volume_tx = volume_tx;
                    this.brightness_tx = brightness_tx;
                    cx.notify();

                })
                .detach();

                SettingsDrawer::new(cx)
            })
        },
    )
    .unwrap();
}
