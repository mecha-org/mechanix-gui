use crate::prelude::IconName;
use crate::ui::utils::prelude::{DesktopApp, DesktopApps};
use gpui::*;

#[derive(Default, Clone)]
pub struct AppDrawerState {
    pub search: String,
    pub apps: DesktopApps,
    pub recent: Vec<String>,
}

impl Global for AppDrawerState {}
