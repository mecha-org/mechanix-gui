use std::collections::HashMap;

use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    widgets::{Div, Svg},
    Node,
};

use crate::{gui::BluetoothStatus, settings::BluetoothIconPaths};

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
                // .bg(0xFF00FFFF)
                ,
                [
                    size: [24, 24],
                ],
            )
            .push(node!(
                Svg::new(self.status.to_string()),
                lay![
                    size: [24, 24],
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
