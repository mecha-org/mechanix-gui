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
pub struct CommonStatusBar {
    pub battery_level: BatteryLevel,
    pub wireless_status: WirelessStatus,
    pub bluetooth_status: BluetoothStatus,
    pub current_time: String,
}

impl Component for CommonStatusBar {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    padding: [6, 14, 5, 10],
                    size_pct: [100],
                    axis_alignment: Alignment::Stretch,
                    //  cross_alignment: Alignment::SpaceBetween
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [50],
                        axis_alignment: Alignment::Start
                    ],
                )
                .push(node!(
                    ClockComponent {
                        current_time: self.current_time.clone(),
                    },
                    lay![
                        margin: [0,  0],
                    ]
                )),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [50],
                        axis_alignment: Alignment::End
                    ]
                )
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
            ),
        )
    }
}
