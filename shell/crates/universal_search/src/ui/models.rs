use gpui::{Entity, Pixels};

use crate::{TextInput, ui::icon::IconName};

pub struct UniversalSearch {
    pub scroll_offset: Pixels,
    pub is_dragging: bool,
    pub drag_start_x: Pixels,
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
}
