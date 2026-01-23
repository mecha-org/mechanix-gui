use crate::config::constants::*;
use crate::models::models::{AppCardAnimation, GestureType};
use crate::ui::RunningApps;
use commons::widgets::wing;
use gpui::prelude::*;
use gpui::*;
use theme::ActiveTheme;
use theme::prelude::AlphaExt;

impl RunningApps {
    fn handle_card_mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) -> bool {
        if self.dragged_card_index.is_none() {
            return false;
        }

        if let Some((start_pos, _start_offset)) = self.drag_start {
            if self.dragged_card_index.is_some() {
                let delta_x = event.position.x - start_pos.x;
                let delta_y = event.position.y - start_pos.y;
                let abs_x = delta_x.abs().to_f64() as f32;
                let abs_y = delta_y.abs().to_f64() as f32;
                const GESTURE_THRESHOLD: f32 = 10.0;

                if self.gesture_locked.is_none()
                    && (abs_x > GESTURE_THRESHOLD || abs_y > GESTURE_THRESHOLD)
                {
                    if abs_x > abs_y {
                        self.gesture_locked = Some(GestureType::Horizontal);
                        // Log drag start
                        // if let Some(card_idx) = self.dragged_card_index {
                        //     if let Some((top_level, app)) = self.apps.get(card_idx) {
                        //         println!(
                        //             " DRAG STARTED: Card {:?} - '{:?}'",
                        //             top_level.app_id(),
                        //             app.name
                        //         );
                        //     }
                        // }
                        self.has_dragged = true;
                    } else {
                        self.gesture_locked = Some(GestureType::Vertical);
                        self.dragged_parent = true;
                    }
                }

                match self.gesture_locked {
                    Some(GestureType::Horizontal) => {
                        self.horizontal_offset = delta_x.to_f64() as f32;
                        cx.notify();
                        return true;
                    }
                    Some(GestureType::Vertical) => {
                        self.dragged_card_index = None;
                        return false;
                    }
                    None => {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn handle_card_mouse_down(
        &mut self,
        card_index: usize,
        event: &MouseDownEvent,
        cx: &mut Context<Self>,
    ) {
        if self.is_animating() {
            return;
        }

        self.dragged_card_index = Some(card_index);
        self.drag_start = Some((event.position, self.scroll_offset));
        self.horizontal_offset = 0.0;
        self.gesture_locked = None;
        self.has_dragged = false;
        cx.stop_propagation();
    }

    fn handle_card_mouse_up(
        &mut self,
        event: &MouseUpEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        // Check if this was a click (minimal movement)
        if let Some((start_pos, _)) = self.drag_start {
            let delta_x = event.position.x - start_pos.x;
            let delta_y = event.position.y - start_pos.y;
            let distance =
                ((delta_x.to_f64() as f32).powi(2) + (delta_y.to_f64() as f32).powi(2)).sqrt();

            if distance < CLICK_THRESHOLD && !self.has_dragged {
                // This is a click!
                if let Some(card_idx) = self.dragged_card_index {
                    if let Some((top_level, app)) = self.apps.get(card_idx) {
                        // println!("CLICK: Card {:?} - '{:?}'", top_level.app_id(), app.name);
                        self.show_apps = false;
                        self.update_input_regions(window, false, cx);
                        top_level.set_maximized();
                        top_level.activate();
                        cx.notify();
                    }
                }
            }
        }

        if self.gesture_locked == Some(GestureType::Horizontal) {
            self.drag_start = None;

            if self.horizontal_offset.abs() > SWIPE_THRESHOLD
                && self.dragged_card_index.is_some()
                && !self.apps.is_empty()
            {
                let direction = if self.horizontal_offset > 0.0 {
                    1.0
                } else {
                    -1.0
                };

                // Log drag completion with swipe
                if let Some(card_idx) = self.dragged_card_index {
                    if let Some((top_level, _app)) = self.apps.get(card_idx) {
                        // let dir_str = if direction > 0.0 { "RIGHT" } else { "LEFT" };
                        // println!(
                        //     "DRAG COMPLETE (SWIPED {}): Card {:?} - '{:?}'",
                        //     dir_str,
                        //     top_level.app_id(),
                        //     app.name
                        // );
                        top_level.close();
                    }
                }

                self.animation_state = AppCardAnimation::SwipingOut {
                    direction,
                    progress: 0.0,
                    start_offset: self.horizontal_offset,
                };
                self.animation_start_time = Some(std::time::Instant::now());
                self.schedule_animation_frame(cx);
            } else {
                // Drag didn't meet threshold
                // if self.has_dragged {
                //     if let Some(card_idx) = self.dragged_card_index {
                //         if let Some((_, app)) = self.apps.get(card_idx) {
                //             println!("DRAG CANCELLED: Card '{:?}'", app.name);
                //         }
                //     }
                // }
                self.horizontal_offset = 0.0;
                self.dragged_card_index = None;
            }

            self.gesture_locked = None;
            self.has_dragged = false;
            cx.notify();
            return true;
        }

        self.gesture_locked = None;
        self.dragged_card_index = None;
        self.horizontal_offset = 0.0;
        self.has_dragged = false;
        false
    }

    pub fn render_card(&self, cx: &mut Context<'_, Self>, i: usize) -> impl IntoElement {
        let colors = cx.theme().colors.clone();

        let (_top_level, app) = &self.apps[i];
        let app_icon_path = app.icon.clone();
        // println!("app_icon_path: {:?}", app_icon_path);
        let app_name: String = app.name.clone();
        let scroll_pos = -self.scroll_offset / CARD_STEP;
        let apps_count = self.apps.len();
        let local_pos = (apps_count - 1 - i) as f32 - scroll_pos;

        let mut offset_y = 0.0;
        let mut offset_x = 0.0;
        let mut scale_factor = 1.0;
        let mut opacity = 1.0;

        if let AppCardAnimation::ClearingAll { start_time } = self.animation_state {
            let elapsed = start_time.elapsed().as_secs_f32();

            // Uses constants for render speed
            let my_duration = CLEAR_ANIMATION_BASE + (i as f32 * CLEAR_ANIMATION_STEP);

            let progress = (elapsed / my_duration).min(1.0);
            let eased = 1.0 - (1.0 - progress).powf(3.0);

            offset_x = 1000.0 * eased;
            opacity = 1.0 - progress;
        } else if Some(i) == self.dragged_card_index {
            if let AppCardAnimation::SwipingOut {
                direction,
                progress,
                start_offset,
            } = self.animation_state
            {
                let eased = 1.0 - (1.0 - progress).powf(3.0);
                let target_offset = direction * 1000.0;
                offset_x = start_offset + (target_offset - start_offset) * eased;
                opacity = 1.0 - eased;
            } else {
                offset_x = self.horizontal_offset;
            }
        }

        if local_pos < 0.0 {
            offset_y = -1000.0 * local_pos;
            if !matches!(self.animation_state, AppCardAnimation::ClearingAll { .. }) {
                opacity = (1.0 + local_pos).max(0.0);
            }
        } else {
            offset_y = -130.0 * (1.0 - 0.7_f32.powf(local_pos));
            scale_factor = 0.92_f32.powf(local_pos);
        }

        let w = CARD_SIZE.width * scale_factor;
        let h = CARD_SIZE.height * scale_factor;
        let left_pos = (CARD_SIZE.width - w) / 2.0 + px(offset_x);
        let upper_wing_width = w * 0.6;
        let upper_wing_height = h * 0.06;

        deferred(
            div()
                .id(("card-", i))
                .absolute()
                .top(px(offset_y))
                .left(left_pos)
                .w(w)
                .h(h)
                .opacity(opacity)
                .rounded_xl()
                .shadow_xl()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, event, window, cx| {
                        this.handle_card_mouse_down(i, event, cx);
                        cx.stop_propagation();
                    }),
                )
                .on_mouse_move(cx.listener(|this, event, window, cx| {
                    let should_stop = this.handle_card_mouse_move(event, cx);
                    if should_stop {
                        cx.stop_propagation();
                    }
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(move |this, event, window, cx| {
                        let should_stop = this.handle_card_mouse_up(event, window, cx);
                        if should_stop {
                            cx.stop_propagation();
                        }
                    }),
                )
                .child({
                    let mut outer_wing = wing();
                    outer_wing.upper_wing_size(size(upper_wing_width, upper_wing_height));
                    outer_wing.border_radius(px(12.));
                    outer_wing.border_width(px(2.0));
                    outer_wing
                        .border_color(colors.accent_200.with_alpha(0.6))
                        .size_full()
                        .flex()
                        .absolute()
                        .bg(colors.background_900)
                        .child(
                             div()
                                .id("inner-wing")   
                                .flex_1()  
                                .child({
                                    let mut inner_wing = wing()
                                        .size_full()
                                        .bg(colors.accent_200.with_alpha(0.1))
                                        .child(
                                            div()
                                                .absolute()
                                                .top(px(0.))
                                                .left(px(0.))
                                                .h(upper_wing_height)
                                                .w_1_2()
                                                .pt(px(15. * scale_factor))
                                                .pl(px(20. * scale_factor))
                                                .flex()
                                                .flex_row()
                                                .gap_2()
                                                .items_center()
                                                .child(
                                                    div()
                                                    .flex_1()
                                                        .text_color(colors.foreground_200)                                
                                                        .text_size(px(20.0 * scale_factor))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_ellipsis()
                                                        .w(upper_wing_width - px(40.0 * scale_factor))
                                                        .child(app_name)
                                                )
                                        )
                                        .child(
                                            div()
                                                .absolute()
                                                .size_full()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .p_4()
                                                .top(upper_wing_height)
                                                .child(
                                                    div()
                                                        .size_full()
                                                        .rounded_2xl()
                                                        .bg(colors.background_1000)
                                                        .flex()
                                                        .p_8()
                                                        .relative()
                                                        .child(
                                                            div()
                                                                .rounded(px(20.0))
                                                                // .bg(colors.background_700)
                                                                .flex()
                                                                .w(w * 0.45)
                                                                .h(h * 0.35)
                                                                .top(upper_wing_height * 0.30)
                                                                .left(w * 0.17)
                                                                .p_6()
                                                                .items_center()
                                                                .justify_center()
                                                                .when_some(app_icon_path, |this, icon| {
                                                                    this.child(
                                                                        img(icon)
                                                                            .w(w * 0.40)
                                                                            .h(h * 0.35)
                                                                    )
                                                                })
                                                        )
                                                )
                                        );
                                        inner_wing.upper_wing_size(size(upper_wing_width, upper_wing_height));
                                        inner_wing.border_radius(px(12.));

                                        inner_wing
                                })
                        )
                }),
        )
        .with_priority(i + 1)
    }
}
