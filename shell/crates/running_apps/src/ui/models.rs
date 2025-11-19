use crate::ui::icon::IconName;
use gpui::*;
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
    pub is_cleaning_up: bool, // Flag for clean up animation
    pub position: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DragDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct AppCard {
    pub id: usize,
    pub offset_y: Pixels,
    pub target_offset_y: Pixels,
    pub app_name: String,
    pub app_icon_path: IconName,
}
