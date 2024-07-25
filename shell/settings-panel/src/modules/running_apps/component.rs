use std::{collections::HashMap, fmt};

use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    widgets::{Div, Image},
    Node,
};

use crate::{
    settings::RunningAppsIconPaths,
    widgets::clickable_setting::{ClickableSetting, SettingText},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RunningAppsStatus {
    Low,
    Medium,
    High,
}

impl fmt::Display for RunningAppsStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunningAppsStatus::Low => write!(f, "RunningAppsStatusLow"),
            RunningAppsStatus::Medium => write!(f, "RunningAppsStatusMedium"),
            RunningAppsStatus::High => write!(f, "RunningAppsStatusHigh"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RotationMessage {
    RunningAppsCountUpdate(i32),
}
#[derive(Debug)]
pub struct RunningAppsComponent {
    pub count: i32,
}

impl Component for RunningAppsComponent {
    fn view(&self) -> Option<Node> {
        let icon = if self.count < 10 {
            RunningAppsStatus::Low.to_string()
        } else {
            RunningAppsStatus::Medium.to_string()
        };

        Some(node!(ClickableSetting::new(
            icon,
            "Running Apps".to_string(),
            SettingText::Bold(self.count.to_string()),
        )
        .click_disabled(true)))
    }
}

pub fn get_running_apps_icons_map(icon_paths: RunningAppsIconPaths) -> HashMap<String, String> {
    let mut assets = HashMap::new();

    if let value = &icon_paths.low {
        assets.insert(RunningAppsStatus::Low.to_string(), value.clone());
    }

    if let value = &icon_paths.medium {
        assets.insert(RunningAppsStatus::Medium.to_string(), value.clone());
    }

    if let value = &icon_paths.high {
        assets.insert(RunningAppsStatus::High.to_string(), value.clone());
    }

    assets
}
