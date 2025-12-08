use std::time::Instant;

use gpui::*;

use crate::{config::HomescreenConfig, state::HomescreenState};

pub struct AnimationManager;
impl AnimationManager {
    pub fn animate(state: &mut HomescreenState) -> bool {
        // Don't animate while user is dragging
        if state.is_page_dragging {
            state.animation_manager_state.last_frame_time = Some(Instant::now());
            return false;
        }

        let Some(last_frame_time) = state.animation_manager_state.last_frame_time else {
            state.animation_manager_state.last_frame_time = Some(Instant::now());
            return false;
        };

        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time).as_secs_f32().max(0.1);
        state.animation_manager_state.last_frame_time = Some(now);

        let mut needs_animation = false;

        // Animate page offset
        let target = 0.0;
        let current = state.page_offset;

        // If we're close enough to target, snap to it
        if current.abs() >= 0.1 {
            // Animate towards target at configured velocity
            let velocity = state.config.animation.page_snap_velocity;
            let direction = if current > target { -1.0 } else { 1.0 };
            let step = direction * velocity * delta_time;
            let new_offset = current + step;

            state.page_offset = new_offset;
            if new_offset * direction > 0.0 {
                state.page_offset = target;
            } else {
                needs_animation = true;
            }
        }

        // Animate widgets to their intended positions
        let widget_snap_velocity = state.config.animation.widget_snap_velocity;
        let dragging_widget_id = state.dragging_widget;
        let screen_width = state.config.window.width;

        for (widget_id, widget_data) in state.widgets.iter_mut() {
            // Skip widgets that are being dragged
            if Some(*widget_id) == dragging_widget_id {
                continue;
            }

            // If widget has a dragged_page, adjust bounds and clear it
            if let Some(dragged_page) = widget_data.dragged_page() {
                let target_page = widget_data.page_number();
                if dragged_page != target_page {
                    // Calculate the page offset
                    let page_offset = -(dragged_page as f32 - target_page as f32) * screen_width;
                    let mut adjusted_bounds = widget_data.bounds();
                    adjusted_bounds.origin.x += px(page_offset);
                    widget_data.widget_mut().set_bounds(adjusted_bounds);
                }
                widget_data.set_dragged_page(None);
            }

            let current_bounds = widget_data.widget().get_bounds();
            let target_bounds = widget_data.bounds();

            // Calculate distance between current and target position
            let dx: f32 = (target_bounds.origin.x - current_bounds.origin.x).into();
            let dy: f32 = (target_bounds.origin.y - current_bounds.origin.y).into();
            let distance = (dx * dx + dy * dy).sqrt();

            // If we're close enough, snap to target
            if distance < 0.5 {
                widget_data.widget_mut().set_bounds(target_bounds);
                continue;
            }

            // Animate towards target
            needs_animation = true;
            let step = widget_snap_velocity * delta_time;
            let t = (step / distance).min(1.0);

            let new_x = current_bounds.origin.x + px(dx * t);
            let new_y = current_bounds.origin.y + px(dy * t);

            let mut new_bounds = current_bounds;
            new_bounds.origin.x = new_x;
            new_bounds.origin.y = new_y;

            widget_data.widget_mut().set_bounds(new_bounds);
        }

        needs_animation
    }
}

pub struct AnimationManagerState {
    pub(crate) last_frame_time: Option<Instant>,
}

impl AnimationManagerState {
    pub fn new(_config: HomescreenConfig) -> Self {
        Self {
            last_frame_time: None,
        }
    }
}
