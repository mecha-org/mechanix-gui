use std::ops::Range;

use gpui::{Bounds, Context, Entity, FocusHandle, Pixels, Point, ShapedLine, SharedString};

use crate::ui::icon::IconName;
use mxsearch::{AppInfo, service::MxSearchService}; // Import your service

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
    pub folder_medium_icon: IconName,
    pub search_icon: IconName,
    pub folder_small_icon: IconName,
    pub x_icon: IconName,
    pub text_input: Entity<TextInput>,
    pub position: f32,
    pub drag_offset: Option<f32>,
    pub drag_start_pos: f32,
    pub search_service: Option<MxSearchService>,
    pub file_search_results: Vec<mxsearch::SearchResult>,
    pub app_search_results: Vec<AppInfo>,
}

pub struct DragInfo {
    pub position: Point<Pixels>,
}

pub struct TextInput {
    pub focus_handle: FocusHandle,
    pub content: SharedString,
    pub placeholder: SharedString,
    pub selected_range: Range<usize>,
    pub selection_reversed: bool,
    pub marked_range: Option<Range<usize>>,
    pub last_layout: Option<ShapedLine>,
    pub last_bounds: Option<Bounds<Pixels>>,
    pub is_selecting: bool,
    pub on_change: Option<Box<dyn Fn(&mut Self, &mut Context<Self>)>>,
}

pub struct SearchResults {
    pub name: String,
    pub path: String,
    pub file_type: FileType,
    pub extension: String,
}

pub struct RecentApps {
    pub name: String,
    pub icon_path: IconName,
}

pub enum FileType {
    App,
    File,
}
