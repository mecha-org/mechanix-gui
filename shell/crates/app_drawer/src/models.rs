use crate::ui::utils::prelude::DesktopApps;
use gpui::*;

#[derive(Default, Clone)]
pub struct AppDrawerState {
    pub apps: DesktopApps,
}

impl Global for AppDrawerState {}
