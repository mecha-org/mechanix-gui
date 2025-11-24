use gpui::*;
use tokio::sync::mpsc;

use crate::prelude::app_manager::AppManagerMessage;

pub struct RunningApps {
    pub scroll_offset: Pixels,
    pub is_dragging: bool,
    pub drag_start_x: Pixels,
    pub target_scroll_offset: Pixels,
    pub drag_start_y: Pixels,
    pub drag_start_offset: Pixels,
    pub apps: Vec<AppCard>,
    pub dragging_card: Option<usize>,
    pub drag_direction: Option<DragDirection>,
    pub is_animating: bool,
    pub is_removing: bool,
    pub removing_card_id: Option<usize>,
    pub current_center_index: usize,
    pub is_cleaning_up: bool,
    pub position: f32,
    pub bar_drag_offset: f32,
    pub bar_drag_start_y: Option<f32>,
    pub show_apps: bool,
    pub message_tx: mpsc::Sender<AppManagerMessage>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DragDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct AppCard {
    pub id: usize,
    pub app_id: String,
    pub offset_y: Pixels,
    pub target_offset_y: Pixels,
    pub app_name: Option<String>,
    pub app_icon_path: Option<String>,
}
