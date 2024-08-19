use mctk_core::component;
use mctk_core::layout::Alignment;
use mctk_core::widgets::Svg;
use mctk_core::{component::Component, lay, node, size, size_pct, widgets::Div, Node};
use std::hash::Hash;

use crate::types::BluetoothStatus;

#[derive(Debug)]
pub struct Bluetooth {
    pub bluetooth_status: BluetoothStatus,
}

impl Component for Bluetooth {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.bluetooth_status.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                ,
                [
                    size_pct: [100],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ],
            )
            .push(node!(
                Svg::new(self.bluetooth_status.to_string()),
                lay![
                    size: [28, 28],
                ],
            )),
        )
    }
}
