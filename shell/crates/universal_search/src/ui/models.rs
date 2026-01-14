use std::{ops::Range, path::PathBuf};

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
    pub ardour_icon: PathBuf,
    pub arrow_up_right_icon: PathBuf,
    pub chromium_icon: PathBuf,
    pub firefox_icon: PathBuf,
    pub github_icon: PathBuf,
    pub folder_icon: PathBuf,
    pub search_icon: PathBuf,
    pub x_icon: PathBuf,
    pub text_input: Entity<TextInput>,
    pub last_search_query: String,
    pub is_searching: bool,
    pub position: f32,
    pub drag_offset: Option<f32>,
    pub drag_start_pos: f32,
    pub search_service: Option<MxSearchService>,
    pub file_search_results: Vec<SearchResult>,
    pub app_search_results: Vec<AppInfo>,
    pub files_app: Option<AppInfo>,
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
    pub icon_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    App,
    File,
}
