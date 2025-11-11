use gpui::prelude::*;
use gpui::Global;
use crate::prelude::IconName;

#[derive(Clone, PartialEq)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub icon_path: IconName,
}

#[derive(Default, Clone)]
pub struct AppDrawerState {
    pub expanded: bool,
    pub search: String,
    pub apps: Vec<AppInfo>,
    pub recent: Vec<String>,
    pub context_menu_app: Option<AppInfo>,
}

impl Global for AppDrawerState {}
