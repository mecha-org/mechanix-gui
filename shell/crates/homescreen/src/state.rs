use gpui::*;

use crate::{
    animation_manager::AnimationManagerState, config::HomescreenConfig,
    input_manager::InputManagerState, layout_manager::LayoutManagerState,
};

pub struct HomescreenState {
    pub config: HomescreenConfig,
    pub layout_manager_state: LayoutManagerState,
    pub animation_manager_state: AnimationManagerState,
    pub input_manager_state: InputManagerState,
}

impl HomescreenState {
    pub fn new(config: HomescreenConfig) -> Self {
        let layout_manager_state = LayoutManagerState::new(config);
        let animation_manager_state = AnimationManagerState::new(config);
        let input_manager_state = InputManagerState::new(config);
        Self {
            config,
            layout_manager_state,
            input_manager_state,
            animation_manager_state,
        }
    }

    pub fn render(&mut self) -> impl IntoElement {
        div().size_full().bg(rgba(0x000000FF))
    }
}
