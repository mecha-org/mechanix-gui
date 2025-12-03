use std::time::Instant;

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

        let target = 0.0;
        let current = state.page_offset;

        // If we're close enough to target, snap to it and stop
        if current.abs() < 0.1 {
            state.page_offset = target;
            return false;
        }

        // Animate towards target at configured velocity
        let velocity = state.config.animation.page_snap_velocity;
        let direction = if current > target { -1.0 } else { 1.0 };
        let step = direction * velocity * delta_time;
        let new_offset = current + step;

        state.page_offset = new_offset;
        if new_offset * direction > 0.0 {
            state.page_offset = target;
        }
        true
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
