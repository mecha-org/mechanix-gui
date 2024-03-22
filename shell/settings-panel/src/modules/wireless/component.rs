use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    msg, node, rect, size, size_pct,
    widgets::{Div, Image, Svg},
    Node,
};

use crate::{
    gui::{Message, SettingNames},
    settings::WirelessIconPaths,
    widgets::clickable_setting::ClickableSetting,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WirelessConnectedState {
    Low,
    Weak,
    Good,
    Strong,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WirelessStatus {
    On,
    #[default]
    Off,
    Connected(WirelessConnectedState),
    NotFound,
}

impl fmt::Display for WirelessStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WirelessStatus::On => write!(f, "WirelessOn"),
            WirelessStatus::Off => write!(f, "WirelessOff"),
            WirelessStatus::Connected(state) => write!(f, "WirelessConnected({:?})", state),
            WirelessStatus::NotFound => write!(f, "WirelessNotFound"),
        }
    }
}

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
        Some(node!(ClickableSetting::new(
            WirelessStatus::On.to_string(),
            "Actonate Wi-fi".to_string(),
            "".to_string(),
            "SpaceGrotesk-Medium".to_string()
        )
        .on_click(Box::new(|| msg!(Message::SettingClicked(
            SettingNames::Wireless
        ))))))
    }
}

pub fn get_wireless_icons_map(icon_paths: WirelessIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let Some(value) = &icon_paths.not_found {
        assets.insert(WirelessStatus::NotFound.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.on {
        assets.insert(WirelessStatus::On.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.off {
        assets.insert(WirelessStatus::Off.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.weak {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Weak).to_string(),
            value.clone(),
        );
    }

    if let Some(value) = &icon_paths.low {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Low).to_string(),
            value.clone(),
        );
    }

    if let Some(value) = &icon_paths.good {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Good).to_string(),
            value.clone(),
        );
    }
    if let Some(value) = &icon_paths.strong {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Strong).to_string(),
            value.clone(),
        );
    }

    assets
}
