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
    types::{WirelessConnectedState, WirelessInfo, WirelessStatus},
    widgets::clickable_setting::{ClickableSetting, SettingText},
};

#[derive(Debug, Clone)]
pub enum WirelessMessage {
    WirelessStatusUpdate(WirelessStatus),
}

#[derive(Debug)]
pub struct WirelessComponent {
    pub status: WirelessStatus,
    pub loading: bool,
}

impl Component for WirelessComponent {
    fn view(&self) -> Option<Node> {
        let mut ssid = "".to_string();
        let mut frequency = "".to_string();
        let icon = self.status.to_string();
        match self.status.clone() {
            WirelessStatus::Connected(_, wireless_info) => {
                ssid = wireless_info.ssid;
                let freq = wireless_info.frequency.parse::<i32>().unwrap();
                frequency = format!("{:.1} {}", freq as f32 / 1000., "GHz");
            }
            _ => (),
        }
        ssid = if ssid.len() > 11 {
            ssid[..11].to_owned()
        } else {
            ssid
        };

        Some(node!(ClickableSetting::new(
            icon,
            frequency,
            SettingText::Normal(ssid),
        )
        .on_click(Box::new(|| msg!(Message::SettingClicked(
            SettingNames::Wireless
        ))))
        .loading(self.loading)))
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
            WirelessStatus::Connected(WirelessConnectedState::Weak, WirelessInfo::default())
                .to_string(),
            value.clone(),
        );
    }

    if let value = &icon_paths.low {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Low, WirelessInfo::default())
                .to_string(),
            value.clone(),
        );
    }

    if let value = &icon_paths.good {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Good, WirelessInfo::default())
                .to_string(),
            value.clone(),
        );
    }
    if let value = &icon_paths.strong {
        assets.insert(
            WirelessStatus::Connected(WirelessConnectedState::Strong, WirelessInfo::default())
                .to_string(),
            value.clone(),
        );
    }

    assets
}
