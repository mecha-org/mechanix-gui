use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Debug, Clone)]
pub struct DesktopApp {
    pub app_id: String,
    pub name: String,
    pub icon: Handle<Image>,
    pub categories: Vec<String>,
    pub exec: String,
    pub on_click: SystemId,
}

#[derive(Debug, Clone)]
pub enum SearchResultType {
    App,
    File,
    Action,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub icon: Handle<Image>,
    pub on_click: Option<SystemId>,
    pub _type: SearchResultType,
}
