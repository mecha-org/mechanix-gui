use gpui::*;

mod events;
pub mod services;
mod ui;
use settings::prelude::*;
use shell_state::ShellState;
use upower::interfaces::device::{BatteryLevel, BatteryState};

use crate::ui::StatusBar;

pub mod prelude {
    pub use crate::events::AppEvents;
    pub use crate::run_app;
    pub use crate::ui::{StatusBar, status_bar_components};
}

pub fn run_app(cx: &mut App) {
    let settings = Settings::global(cx).status_bar.clone();
    let LayerShellSettings {
        size,
        layer,
        anchor,
        namespace,
        exclusive_zone,
    } = settings.layer_shell.clone();
    let window_bounds = WindowBounds::Windowed(Bounds::centered(
        None,
        Size {
            width: px(0.),
            height: size.height,
        },
        cx,
    ));
    let window = cx
        .open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                window_background: WindowBackgroundAppearance::Transparent,
                kind: WindowKind::LayerShell(layer_shell::LayerShellOptions {
                    namespace,
                    layer,
                    anchor,
                    exclusive_zone: Some(exclusive_zone),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let mut regions = Vec::new();
                regions.push(Bounds {
                    origin: point(px(0.), px(0.)),
                    size: gpui::size(size.width, px(5.)),
                });
                window.set_input_regions(Some(regions));

                cx.new(|cx| {
                    cx.observe_global::<ShellState>(|this: &mut StatusBar, cx| {
                        let ShellState {
                            wireless_details,
                            bluetooth_details,
                            battery_state,
                            battery_level,
                            ..
                        } = ShellState::global(cx).clone();
                        let bluetooth_connected = bluetooth_details.connected_devices > 0;
                        let wireless_connected = wireless_details.connected_network.is_some();

                        if (this.bluetooth_connected != bluetooth_connected)
                            || (this.wireless_connected != wireless_connected)
                            || (this.battery_state != battery_state
                                && (battery_state == BatteryState::Unknown
                                    || battery_state == BatteryState::Charging
                                    || battery_state == BatteryState::Discharging
                                    || battery_state == BatteryState::Empty))
                            || (this.battery_level != battery_level
                                && (battery_level == BatteryLevel::Low
                                    || battery_level == BatteryLevel::Critical))
                            || (this.battery_state != battery_state)
                        {
                            this.bluetooth_connected = bluetooth_connected;
                            this.wireless_connected = wireless_connected;
                            this.battery_state = battery_state;
                            this.battery_level = battery_level;
                            this.show_status_bar(cx);
                        }
                    })
                    .detach();

                    StatusBar::new()
                })
            },
        )
        .unwrap();
    let entity_id = window.entity(cx).unwrap().entity_id();
    ShellState::global_mut(cx).status_bar_entity = Some(entity_id);
}
