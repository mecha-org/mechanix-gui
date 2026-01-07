use commons::prelude::*;
use gpui::{foreign_toplevel_management::ForeignToplevelHandle, *};

pub enum AppCardAnimation {
    None,
    SwipingOut {
        direction: f32,
        progress: f32,
        start_offset: f32,
    },
    ClearingAll {
        start_time: std::time::Instant,
    },
    Initial {
        start_time: std::time::Instant,
        start_offset: f32,
    },
}

#[derive(Clone, Copy, PartialEq)]
pub enum GestureType {
    Horizontal,
    Vertical,
}

pub struct RunningApps {
    pub scroll_offset: f32,
    pub animation_state: AppCardAnimation,
    pub dragged_card_index: Option<usize>,
    pub dragged_parent: bool,
    pub horizontal_offset: f32,
    pub drag_start: Option<(Point<Pixels>, f32)>,
    pub gesture_locked: Option<GestureType>,
    pub animation_start_time: Option<std::time::Instant>,
    pub has_dragged: bool, // Track if user has dragged
    pub apps: Vec<(ForeignToplevelHandle, commons::prelude::App)>,
    pub bar_drag_offset: f32,
    pub bar_drag_start_y: Option<f32>,
    pub show_apps: bool,
    pub installed_apps: Entity<InstalledApps>,
}
