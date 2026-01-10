use std::ops::Range;

use crate::ui::icon::IconName;
use commons::input::TextInput;
use gpui::{Bounds, Context, Entity, FocusHandle, Pixels, Point, ShapedLine, SharedString};
use mxsearch::{prelude::*, service::MxSearchService};

pub struct UniversalSearch {
    pub scroll_offset: Pixels,
    pub is_dragging: bool,
    pub drag_start_y: Pixels,
    pub last_scroll_offset: Pixels,
    pub app_count: usize,
    pub file_count: usize,
    pub ardour_icon: IconName,
    pub arrow_up_right_icon: IconName,
    pub chromium_icon: IconName,
    pub firefox_icon: IconName,
    pub github_icon: IconName,
    pub folder_icon: IconName,
    pub search_icon: IconName,
    pub x_icon: IconName,
    pub text_input: Entity<TextInput>,
    pub last_search_query: String,
    pub is_searching: bool,
    pub position: f32,
    pub drag_offset: Option<f32>,
    pub drag_start_pos: f32,
    pub search_service: Option<MxSearchService>,
    pub file_search_results: Vec<SearchResult>,
    pub app_search_results: Vec<AppInfo>,
}

pub struct DragInfo {
    pub position: Point<Pixels>,
}

pub struct SearchResults {
    pub name: String,
    pub path: Option<String>,
    pub file_type: FileType,
    pub extension: String,
    pub possible_app_id: String,
    pub exec: String,
}

pub struct RecentApps {
    pub name: String,
    pub icon_path: IconName,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    App,
    File,
}
