use std::hash::Hash;

use mctk_core::layout::Alignment;
use mctk_core::style::Styled;
use mctk_core::widgets::{Image, Text};
use mctk_core::{component::Component, lay, node, rect, size, size_pct, widgets::Div, Node};
use mctk_core::{txt, Color};
use networkmanager::WirelessModel;

use crate::modules::battery::model::BatteryModel;
use crate::modules::clock::model::ClockModel;
use crate::types::{BluetoothStatus, WirelessStatus};
use crate::utils::{get_formatted_battery_level, get_forttated_wireless_status};

#[derive(Debug)]
pub struct StatusBar {
    pub time_format: String,
    pub bluetooth_status: BluetoothStatus,
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
                    cross_alignment: Alignment::Center
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [50, Auto],
                        axis_alignment: Alignment::Start
                    ],
                )
                .push(node!(
                    Clock {
                        time_format: self.time_format.clone()
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
                        size_pct: [50, Auto],
                        axis_alignment: Alignment::End
                    ]
                )
                .push(node!(Wireless {}, lay![margin: [0, 0]]))
                .push(node!(
                    Bluetooth {
                        status: self.bluetooth_status,
                    },
                    lay![margin: [0, 20]]
                ))
                .push(node!(Battery {}, lay![margin: [0, 0]])),
            ),
        )
    }
}

#[derive(Debug)]
pub struct Clock {
    pub time_format: String,
}

impl Component for Clock {
    fn props_hash(&self, hasher: &mut mctk_core::prelude::ComponentHasher) {
        self.time_format.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let time = &ClockModel::time(&self.time_format);

        Some(node!(
            Text::new(txt!(time.clone()))
                .with_class("text-white font-space-grotesk font-bold")
                .style("size", 15.0),
            lay![
                size: [Auto, 20],
            ]
        ))
    }
}

#[derive(Debug)]
pub struct Wireless {}

impl Component for Wireless {
    fn view(&self) -> Option<Node> {
        let wireless_status = get_forttated_wireless_status(WirelessModel::get());
        Some(node!(
            Image::new(format!("sm{:?}", wireless_status.to_string())),
            lay![
                size: [22, 22],
            ]
        ))
    }
}

#[derive(Debug)]
pub struct Bluetooth {
    pub status: BluetoothStatus,
}

impl Component for Bluetooth {
    fn view(&self) -> Option<Node> {
        Some(node!(
            Image::new(format!("sm{:?}", self.status.to_string())),
            lay![
                size: [22, 22],
            ],
        ))
    }
}
#[derive(Debug)]
pub struct Battery {}

impl Component for Battery {
    fn view(&self) -> Option<Node> {
        let level = BatteryModel::level();
        let status = BatteryModel::status();
        let battery_level = get_formatted_battery_level(&level, &status);

        Some(node!(
            Image::new(battery_level.to_string()),
            lay![
                size: [22, 22],
            ],
        ))
    }
}
