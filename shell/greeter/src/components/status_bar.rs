use mctk_core::{
    component::Component, lay, layout::Alignment, node, rect, size, size_pct, widgets::Div, Color,
    Node,
};

use crate::{
    modules::{
        battery::component::BatteryComponent, bluetooth::component::BluetoothComponent,
        clock::component::ClockComponent, wireless::component::WirelessComponent,
    },
    types::{BatteryLevel, BluetoothStatus, WirelessStatus},
};

#[derive(Debug)]
pub struct StatusBar {
    pub battery_level: BatteryLevel,
    pub wireless_status: WirelessStatus,
    pub bluetooth_status: BluetoothStatus,
    pub current_time: String,
}

impl Component for StatusBar {
    fn view(&self) -> Option<Node> {
        let bg_color = if true {
            Color::rgba(5., 7., 10., 1.)
        } else {
            Color::TRANSPARENT
        };

        Some(
            node!(
                Div::new().bg(bg_color),
                lay![
                    padding: [6, 14, 5, 0],
                    size_pct: [100],
                    axis_alignment: Alignment::Start,
                    // cross_alignment: Alignment::SpaceBetween
                ]
            )
            .push(node!(
                ClockComponent {
                    current_time: self.current_time.clone(),
                },
                lay![
                    margin: [0, 0, 0, 263],
                ]
            ))
            .push(node!(
                WirelessComponent {
                    status: self.wireless_status,
                },
                lay![margin: [0, 0]]
            ))
            .push(node!(
                BluetoothComponent {
                    status: self.bluetooth_status,
                },
                lay![margin: [0, 14]]
            ))
            .push(node!(
                BatteryComponent {
                    level: self.battery_level,
                },
                lay![margin: [0, 0]]
            )),
        )
    }
}
