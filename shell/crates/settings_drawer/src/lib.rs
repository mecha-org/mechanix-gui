pub mod constants;
mod events;
pub mod services;
mod ui;

use bluez::service::BluetoothService;
use events::*;
use futures::{SinkExt, StreamExt, channel::mpsc, select};
use gpui::{
    layer_shell::{KeyboardInteractivity, LayerShellOptions},
    *,
};
use networkmanager::{interfaces::wireless::NMState, service::NetworkManagerService};
use pulseaudio::service::PulseAudioService;
use services::*;
use settings::prelude::*;
use shell_state::{ShellState, ShellStateMessage};
use system_dbus::display_client;
use ui::*;
use upower::service::UPowerService;

use crate::ui::icon::IconName;

pub mod prelude {
    pub use crate::constants::*;
    pub use crate::events::{AppEvents, BrightnessEvents, VolumeEvents};
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
                        nm_tx,
                        bt_tx,
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

                    this.nm_tx = nm_tx;
                    this.bt_tx = bt_tx;
                })
                .detach();

                SettingsDrawer::new(
                    cx,
                    // bt_tx.clone(),
                    // volume_tx.clone(),
                    // brightness_tx.clone(),
                )
            })
        },
    )
    .unwrap();
}

pub fn get_wireless_strength_icon(enable: bool, signal_strength: u8, security: String) -> IconName {
    if enable {
        match security.as_str() {
            "Open" => match signal_strength {
                0 => IconName::ConnectedWifiOn,
                0..=30 => IconName::ConnectedWifiLow,
                31..=60 => IconName::ConnectedWifiMedium,
                61..=100 => IconName::ConnectedWifiHigh,
                _ => IconName::ConnectedWifiWarning,
            },
            "Protected" => match signal_strength {
                0..=30 => IconName::ConnectedWifiLowLocked,
                31..=60 => IconName::ConnectedWifiMediumLocked,
                61..=100 => IconName::ConnectedWifiHighLocked,
                _ => IconName::ConnectedWifiWarning,
            },
            _ => IconName::ConnectedWifiWarning,
        }
    } else {
        match security.as_str() {
            "Open" => match signal_strength {
                0 => IconName::WifiOn,
                0..=30 => IconName::WifiLow,
                31..=60 => IconName::WifiMedium,
                61..=100 => IconName::WifiHigh,
                _ => IconName::WifiWarning,
            },
            "Protected" => match signal_strength {
                0..=30 => IconName::WifiLowLocked,
                31..=60 => IconName::WifiMediumLocked,
                61..=100 => IconName::WifiHighLocked,
                _ => IconName::WifiWarning,
            },
            _ => IconName::WifiWarning,
        }
    }
}

pub fn get_bluetooth_icon(connected: bool) -> IconName {
    if connected {
        IconName::BluetoothConnected
    } else {
        IconName::BluetoothOff
    }
}
