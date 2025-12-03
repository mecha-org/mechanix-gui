use std::time::Instant;

use gpui::*;

use crate::{config::HomescreenConfig, state::HomescreenState, widgets::WidgetId};

pub struct InputManagerState {
    pub(crate) is_mouse_pressed: bool,
    pub(crate) mouse_press_position: Option<Point<Pixels>>,
    pub(crate) drag_start_position: Option<Point<Pixels>>,
}

impl InputManagerState {
    pub fn new(_config: HomescreenConfig) -> Self {
        Self {
            is_mouse_pressed: false,
            mouse_press_position: None,
            drag_start_position: None,
        }
    }
}

pub struct InputManager;
impl InputManager {
    pub fn mouse_down(mouse_down_event: &MouseDownEvent, state: &mut HomescreenState) {
        state.input_manager_state.is_mouse_pressed = true;
        state.input_manager_state.mouse_press_position = Some(mouse_down_event.position);
        state.input_manager_state.drag_start_position = None;
        state.is_page_dragging = false;
    }

    pub fn mouse_move(mouse_move_event: &MouseMoveEvent, state: &mut HomescreenState) -> bool {
        if !state.input_manager_state.is_mouse_pressed {
            return false;
        }

        let Some(press_position) = state.input_manager_state.mouse_press_position else {
            return false;
        };

        let current_position = mouse_move_event.position;

        let drag_config = &state.config.drag;

        if !state.is_page_dragging {
            let dx: f32 = (current_position.x - press_position.x).into();
            if dx.abs() >= drag_config.drag_initiation_threshold {
                state.is_page_dragging = true;
                state.input_manager_state.drag_start_position = Some(press_position);
            } else {
                return false;
            }
        }

        if state.is_page_dragging {
            let drag_start = state
                .input_manager_state
                .drag_start_position
                .unwrap_or(press_position);
            let horizontal_offset: f32 = (current_position.x - drag_start.x).into();
            state.page_offset = horizontal_offset;
            return true;
        }

        false
    }

    pub fn mouse_up(mouse_up_event: &MouseUpEvent, state: &mut HomescreenState) {
        // Handle page drag end
        if state.is_page_dragging {
            let Some(drag_start) = state.input_manager_state.drag_start_position else {
                state.input_manager_state.is_mouse_pressed = false;
                state.input_manager_state.mouse_press_position = None;
                state.is_page_dragging = false;
                return;
            };

            let release_position = mouse_up_event.position;
            let total_drag_distance: f32 = (release_position.x - drag_start.x).into();
            let drag_config = &state.config.drag;

            if total_drag_distance.abs() >= drag_config.page_switch_threshold {
                if total_drag_distance < 0.0 {
                    if state.active_page < state.pages.len() - 1 {
                        state.set_active_page(state.active_page + 1);
                    }
                } else if state.active_page > 0 {
                    state.set_active_page(state.active_page - 1);
                }
            }
        }

        state.input_manager_state.is_mouse_pressed = false;
        state.input_manager_state.mouse_press_position = None;
        state.input_manager_state.drag_start_position = None;
        state.is_page_dragging = false;
    }
}
