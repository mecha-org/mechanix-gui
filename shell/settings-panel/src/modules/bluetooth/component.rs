use std::{collections::HashMap, fmt, hash::Hash};

use mctk_core::{
    component::{Component, ComponentHasher},
    lay, msg, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{
    gui::{Message, SettingNames},
    settings::BluetoothIconPaths,
    types::BluetoothStatus,
    widgets::clickable_setting::{ClickableSetting, SettingText},
};

#[derive(Debug, Clone)]
pub enum BluetoothMessage {
    BluetoothStatusUpdate(BluetoothStatus),
}
#[derive(Debug)]
pub struct BluetoothComponent {
    pub status: BluetoothStatus,
    pub loading: bool,
}

impl Component for BluetoothComponent {
    fn view(&self) -> Option<Node> {
        let bluetooth_off = match self.status {
            BluetoothStatus::Off => true,
            BluetoothStatus::NotFound => true,
            _ => false,
        };

        Some(node!(ClickableSetting::new(
            self.status.to_string(),
            "Bluetooth".to_string(),
            SettingText::Normal("".to_string()),
        )
        .on_click(Box::new(|| msg!(Message::SettingClicked(
            SettingNames::Bluetooth
        ))))
        .disabled(bluetooth_off)))
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
