use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{
    settings::MemoryIconPaths,
    widgets::clickable_setting::{ClickableSetting, SettingText},
};

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
    pub usage: u64,
}

impl Component for MemoryComponent {
    fn view(&self) -> Option<Node> {
        let icon = if self.usage < 10 {
            MemoryUsage::Low.to_string()
        } else {
            MemoryUsage::Medium.to_string()
        };

        let mut unit = "".to_string();
        let mut memory_used = "".to_string();
        let memory_b: u64 = self.usage;
        if memory_b < 1000000000 {
            // Convert to MB
            let memory_mb = memory_b as f64 / 1000000.0;
            memory_used = format!("{:.0}", memory_mb.clone());
            unit = "MB".to_string();
        } else {
            // Convert to GB
            let memory_gb = memory_b as f64 / 1000000000.0;
            memory_used = format!("{:.2}", memory_gb);
            unit = "GB".to_string();
        }

        Some(node!(ClickableSetting::new(
            icon,
            "Memory".to_string(),
            SettingText::Subscript(memory_used.to_string(), unit.to_string()),
        )
        .click_disabled(true)))
    }
}

pub fn get_memory_icons_map(icon_paths: MemoryIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.low {
        assets.insert(MemoryUsage::Low.to_string(), value.clone());
    }

    if let value = &icon_paths.medium {
        assets.insert(MemoryUsage::Medium.to_string(), value.clone());
    }

    if let value = &icon_paths.high {
        assets.insert(MemoryUsage::High.to_string(), value.clone());
    }

    assets
}
