use std::os::linux::raw::stat;
use std::time::Instant;

use gpui::*;

use crate::{config::HomescreenConfig, state::HomescreenState, widgets::WidgetId};

pub struct InputManagerState {
    pub(crate) is_mouse_pressed: bool,
    pub(crate) mouse_press_position: Option<Point<Pixels>>,
    pub(crate) drag_start_position: Option<Point<Pixels>>,
    pub(crate) mouse_press_time: Option<Instant>,
    pub(crate) widget_under_cursor: Option<WidgetId>,
    pub(crate) edge_hover_start_time: Option<Instant>,
    pub(crate) edge_hover_target_page: Option<usize>,
}

impl InputManagerState {
    pub fn new(_config: HomescreenConfig) -> Self {
        Self {
            is_mouse_pressed: false,
            mouse_press_position: None,
            drag_start_position: None,
            mouse_press_time: None,
            widget_under_cursor: None,
            edge_hover_start_time: None,
            edge_hover_target_page: None,
        }
    }
}

pub struct InputManager;
impl InputManager {
    pub fn mouse_down(mouse_down_event: &MouseDownEvent, state: &mut HomescreenState) {
        state.input_manager_state.is_mouse_pressed = true;
        state.input_manager_state.mouse_press_position = Some(mouse_down_event.position);
        state.input_manager_state.drag_start_position = None;
        state.input_manager_state.mouse_press_time = Some(Instant::now());
        state.input_manager_state.widget_under_cursor =
            state.find_widget_under_point(mouse_down_event.position);
        state.is_page_dragging = false;
        state.dragging_widget = None;
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

        // Calculate movement from initial press position
        let dx: f32 = (current_position.x - press_position.x).into();
        let dy: f32 = (current_position.y - press_position.y).into();
        let movement_distance = (dx * dx + dy * dy).sqrt();

        // Check if we should start widget dragging and prevent dragging of fixed widgets
        if state.dragging_widget.is_none() && !state.is_page_dragging {
            if let Some(press_time) = state.input_manager_state.mouse_press_time {
                let elapsed = press_time.elapsed();

                if movement_distance < drag_config.drag_initiation_threshold
                    && elapsed >= drag_config.widget_drag_time_threshold
                {
                    if let Some(widget_id) = state.input_manager_state.widget_under_cursor {
                        // Check if the widget is on the first or last page
                        if let Some(widget_data) = state.widgets.get(&widget_id) {
                            let page = widget_data.page_number();
                            // Page 0 and Pages.len()-1 are our fixed widgets
                            if page == 0 || page == state.pages.len() - 1 {
                                return false; // Prevent dragging
                            }
                        }

                        state.pick_widget(widget_id);
                        state.input_manager_state.drag_start_position =
                            Some(mouse_move_event.position);
                        return true;
                    }
                }
            }
        }

        if let Some(dragging_widget_id) = state.dragging_widget.as_ref() {
            let dragging_widget = state.widgets.get_mut(dragging_widget_id).unwrap();
            let position = dragging_widget.bounds().origin
                - state.input_manager_state.drag_start_position.unwrap()
                + mouse_move_event.position;
            let mut bounds = dragging_widget.bounds();
            bounds.origin = position;
            dragging_widget.widget_mut().set_bounds(bounds);

            // Edge hover detection for page switching
            let widget_bounds = bounds;
            let screen_width = state.config.window.width;
            let edge_threshold = state.config.drag.edge_hover_threshold;

            // Check if widget is near the left or right edge
            let widget_center_x: f32 = widget_bounds.origin.x.into();
            let widget_center_x =
                widget_center_x + Into::<f32>::into(widget_bounds.size.width) / 2.0;

            let near_left_edge = widget_center_x < edge_threshold && state.active_page > 0;
            let near_right_edge = widget_center_x > (screen_width - edge_threshold)
                && state.active_page < state.pages.len() - 1;

            if near_left_edge || near_right_edge {
                let target_page = if near_left_edge {
                    state.active_page - 1
                } else {
                    state.active_page + 1
                };

                // Check if we're hovering on a different edge than before
                if state.input_manager_state.edge_hover_target_page != Some(target_page) {
                    // Reset timer for new edge
                    state.input_manager_state.edge_hover_start_time = Some(Instant::now());
                    state.input_manager_state.edge_hover_target_page = Some(target_page);
                } else if let Some(hover_start) = state.input_manager_state.edge_hover_start_time {
                    // Check if we've been hovering long enough
                    if hover_start.elapsed() >= state.config.drag.edge_hover_wait_interval {
                        state.set_active_page(target_page);
                        // Update the dragged widget's page
                        if let Some(widget_id) = state.dragging_widget {
                            if let Some(widget_data) = state.widgets.get_mut(&widget_id) {
                                widget_data.set_dragged_page(Some(target_page));
                            }
                        }
                        // Reset the timer so we don't keep switching
                        state.input_manager_state.edge_hover_start_time = None;
                        state.input_manager_state.edge_hover_target_page = None;
                    }
                }
            } else {
                // Not near any edge, reset edge hover state
                state.input_manager_state.edge_hover_start_time = None;
                state.input_manager_state.edge_hover_target_page = None;
            }

            return true;
        }

        // Otherwise, handle page dragging
        if !state.is_page_dragging {
            let horizontal_movement = dx.abs();
            if horizontal_movement >= drag_config.drag_initiation_threshold {
                state.is_page_dragging = true;
                state.input_manager_state.drag_start_position = Some(press_position);
            } else {
                return true;
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
                state.input_manager_state.mouse_press_time = None;
                state.input_manager_state.widget_under_cursor = None;
                state.is_page_dragging = false;
                state.dragging_widget = None;
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

        if state.dragging_widget.is_some() {
            state.drop_widget();
        }

        state.input_manager_state.is_mouse_pressed = false;
        state.input_manager_state.mouse_press_position = None;
        state.input_manager_state.drag_start_position = None;
        state.input_manager_state.mouse_press_time = None;
        state.input_manager_state.widget_under_cursor = None;
        state.input_manager_state.edge_hover_start_time = None;
        state.input_manager_state.edge_hover_target_page = None;
        state.is_page_dragging = false;
    }
}
