use mctk_core::layout::Alignment;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{Svg, Text};
use mctk_core::{component::Component, lay, node, rect, size, size_pct, widgets::Div, Node};
use mctk_core::{txt, Color};

use crate::types::{BatteryLevel, BluetoothStatus, WirelessStatus};

#[derive(Debug)]
pub struct StatusBar {
    pub battery_level: BatteryLevel,
    pub wireless_status: WirelessStatus,
    pub bluetooth_status: BluetoothStatus,
    pub current_time: String,
}

impl Component for StatusBar {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    padding: [13, 20, 1, 20],
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
                    Clock {
                        time: self.current_time.clone(),
                    },
                    lay![
                        margin: [0,  0],
                        size_pct: [100],
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
                    Wireless {
                        status: self.wireless_status,
                    },
                    lay![margin: [0, 0]]
                ))
                .push(node!(
                    Bluetooth {
                        status: self.bluetooth_status,
                    },
                    lay![margin: [0, 14]]
                ))
                .push(node!(
                    Battery {
                        level: self.battery_level,
                    },
                    lay![margin: [0, 0]]
                )),
            ),
        )
    }
}

#[derive(Debug)]
pub struct Clock {
    pub time: String,
}

impl Component for Clock {
    fn view(&self) -> Option<Node> {
        Some(node!(
            Text::new(txt!(self.time.clone()))
                .style("color", Color::WHITE)
                .style("size", 15.0)
                .style("font", "SpaceGrotesk-Bold")
                .style("font_weight", FontWeight::Bold),
            lay![
                size: [Auto, 20],
            ]
        ))
    }
}

#[derive(Debug)]
pub struct Wireless {
    pub status: WirelessStatus,
}

impl Component for Wireless {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                // .bg(Color::RED)
                ,
                [
                    size: [24, 24],
                    padding: [1, 2, 1, 2]
                ],
            )
            .push(node!(
                Svg::new(self.status.to_string()),
                lay![
                    size: [20, 20],
                ],
            )),
        )
    }
}

#[derive(Debug)]
pub struct Bluetooth {
    pub status: BluetoothStatus,
}

impl Component for Bluetooth {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                // .bg(Color::LIGHT_GREY)
                ,
                [
                    size: [24, 24],
                    padding: [1, 2, 1, 2]
                ],
            )
            .push(node!(
                Svg::new(self.status.to_string()),
                lay![
                    size: [20, 20],
                ],
            )),
        )
    }
}
#[derive(Debug)]
pub struct Battery {
    pub level: BatteryLevel,
}

impl Component for Battery {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                // .bg(Color::LIGHT_GREY)
                ,
                [
                    size: [24, 24],
                    // cross_alignment: Alignment::Center,
                    // axis_alignment: Alignment::Center,
                    padding: [1, 2, 1, 2]
                ],
            )
            .push(node!(
                Svg::new(self.level.to_string()),
                lay![
                    size: [20, 20],
                ],
            )),
        )
    }
}
