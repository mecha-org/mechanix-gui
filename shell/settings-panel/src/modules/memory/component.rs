use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{settings::CommonLowMediumHighPaths, widgets::clickable_setting::ClickableSetting};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MemoryUsage {
    Low,
    Medium,
    High,
}

impl fmt::Display for MemoryUsage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemoryUsage::Low => write!(f, "MemoryUsageLow"),
            MemoryUsage::Medium => write!(f, "MemoryUsageMedium"),
            MemoryUsage::High => write!(f, "MemoryUsageHigh"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemoryMessage {
    MemoryUsageUpdate(i32),
}
#[derive(Debug)]
pub struct MemoryComponent {
    pub usage: i32,
}

impl Component for MemoryComponent {
    fn view(&self) -> Option<Node> {
        let icon = if self.usage < 10 {
            MemoryUsage::Low.to_string()
        } else {
            MemoryUsage::Medium.to_string()
        };

        Some(node!(ClickableSetting::new(
            icon,
            "Memory".to_string(),
            format!("{} %", self.usage),
            "SpaceGrotesk-Medium".to_string()
        )))
    }
}

pub fn get_memory_icons_map(icon_paths: CommonLowMediumHighPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let Some(value) = &icon_paths.low {
        assets.insert(MemoryUsage::Low.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.medium {
        assets.insert(MemoryUsage::Medium.to_string(), value.clone());
    }

    if let Some(value) = &icon_paths.high {
        assets.insert(MemoryUsage::High.to_string(), value.clone());
    }

    assets
}
