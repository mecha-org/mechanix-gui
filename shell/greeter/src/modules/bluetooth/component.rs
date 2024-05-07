use std::collections::HashMap;

use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    node, rect, size, size_pct,
    widgets::{Div, Svg},
    Color, Node,
};

use crate::{settings::BluetoothIconPaths, types::BluetoothStatus};

#[derive(Debug, Clone)]
pub enum BluetoothMessage {
    BluetoothStatusUpdate(BluetoothStatus),
}
#[derive(Debug)]
pub struct BluetoothComponent {
    pub status: BluetoothStatus,
}

impl Component for BluetoothComponent {
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
                Svg::new(self.status.to_string()),
                lay![
                    size: [20, 20],
                ],
            )),
        )
    }
}

pub fn get_bluetooth_icons_map(icon_paths: BluetoothIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let Some(value) = &icon_paths.not_found {
        assets.insert(BluetoothStatus::NotFound.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.on {
        assets.insert(BluetoothStatus::On.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.off {
        assets.insert(BluetoothStatus::Off.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.connected {
        assets.insert(BluetoothStatus::Connected.to_string(), value.clone());
    }

    assets
}
