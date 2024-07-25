use std::collections::HashMap;

use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    node, rect, size, size_pct,
    widgets::{Div, Image, Svg},
    Color, Node,
};

use crate::{
    settings::WirelessIconPaths,
    types::{WirelessConnectedState, WirelessStatus},
};

#[derive(Debug, Clone)]
pub enum WirelessMessage {
    WirelessStatusUpdate(WirelessStatus),
}

#[derive(Debug)]
pub struct WirelessComponent {
    pub status: WirelessStatus,
}

impl Component for WirelessComponent {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                // .bg(Color::RED)
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

pub fn get_wireless_icons_map(icon_paths: WirelessIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.not_found {
        assets.insert(WirelessStatus::NotFound.to_string(), value.clone());
    }

    if let value = &icon_paths.on {
        assets.insert(WirelessStatus::On.to_string(), value.clone());
    }

    if let value = &icon_paths.off {
        assets.insert(WirelessStatus::Off.to_string(), value.clone());
    }

    if let value = &icon_paths.weak {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Weak).to_string(),
            value.clone(),
        );
    }

    if let value = &icon_paths.low {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Low).to_string(),
            value.clone(),
        );
    }

    if let value = &icon_paths.good {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Good).to_string(),
            value.clone(),
        );
    }
    if let value = &icon_paths.strong {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Strong).to_string(),
            value.clone(),
        );
    }

    assets
}
