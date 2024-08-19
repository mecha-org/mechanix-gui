use mctk_core::layout::{Alignment, Direction};
use mctk_core::{component::Component, lay, node, size, widgets::Div, Node};

use crate::types::{BatteryLevel, BluetoothStatus, WirelessStatus};

use super::battery::component::Battery;
use super::bluetooth::component::Bluetooth;
use super::wireless::component::Wireless;

#[derive(Debug)]
pub struct Controls {
    pub battery_level: BatteryLevel,
    pub wireless_status: WirelessStatus,
    pub bluetooth_status: BluetoothStatus,
}

impl Component for Controls {
    fn view(&self) -> Option<Node> {
        let mut controls_node = node!(
            Div::new(),
            lay![
                size: [120, 120],
                axis_alignment: Alignment::Start,
                direction: Direction::Row,
                wrap: true,
            ],
        );

        controls_node = controls_node.push(node!(
            Wireless {
                wireless_status: self.wireless_status
            },
            lay![size: [60, 60]]
        ));
        controls_node = controls_node.push(node!(
            Battery {
                battery_level: self.battery_level
            },
            lay![size: [60, 60]]
        ));
        controls_node = controls_node.push(node!(
            Bluetooth {
                bluetooth_status: self.bluetooth_status
            },
            lay![size: [60, 60]]
        ));

        Some(controls_node)
    }
}
