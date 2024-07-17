use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{
    settings::CpuIcons,
    widgets::clickable_setting::{ClickableSetting, SettingText},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CpuUsage {
    Low,
    Medium,
    High,
}

impl fmt::Display for CpuUsage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuUsage::Low => write!(f, "CpuUsageLow"),
            CpuUsage::Medium => write!(f, "CpuUsageMedium"),
            CpuUsage::High => write!(f, "CpuUsageHigh"),
        }
    }
}

#[derive(Debug)]
pub struct CpuComponent {
    pub usage: i32,
}

impl Component for CpuComponent {
    fn view(&self) -> Option<Node> {
        let icon = if self.usage < 10 {
            CpuUsage::Low.to_string()
        } else {
            CpuUsage::Medium.to_string()
        };

        Some(node!(ClickableSetting::new(
            icon,
            "CPU".to_string(),
            SettingText::Subscript(self.usage.to_string(), "%".to_string()),
        )
        .click_disabled(true)))
    }
}

pub fn get_cpu_icons_map(icon_paths: CpuIcons) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.low {
        assets.insert(CpuUsage::Low.to_string(), value.clone());
    }

    if let value = &icon_paths.medium {
        assets.insert(CpuUsage::Medium.to_string(), value.clone());
    }

    if let value = &icon_paths.high {
        assets.insert(CpuUsage::High.to_string(), value.clone());
    }

    assets
}
