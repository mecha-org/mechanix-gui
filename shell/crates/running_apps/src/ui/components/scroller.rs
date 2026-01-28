use crate::config::constants::*;
use crate::models::models::AppCardAnimation;
use crate::ui::RunningApps;
use gpui::prelude::*;
use gpui::*;

impl RunningApps {
    fn on_mouse_down(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        if self.is_animating() {
            return;
        }
        self.drag_start = Some((event.position, self.scroll_offset));
        self.horizontal_offset = 0.0;
        self.dragged_card_index = None;
        self.has_dragged = false;
        self.dragged_parent = true;
    }
    pub fn on_mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        if !self.dragged_parent {
            return;
        }

        if let Some((start_pos, start_offset)) = self.drag_start {
            let delta_x = event.position.x - start_pos.x;
            let delta_y = event.position.y - start_pos.y;

            if self.dragged_card_index.is_some() {
                self.horizontal_offset = delta_x.to_f64() as f32;
            } else {
                if delta_x.abs() < px(30.0) {
                    self.scroll_offset = start_offset - delta_y.to_f64() as f32;
                    let max_scroll = (self.apps.len() as f32 - 1.0) * CARD_STEP;
                    self.scroll_offset = self.scroll_offset.clamp(-max_scroll, 0.0);
                }
            }
            cx.notify();
        }
    }

    pub fn on_mouse_up(&mut self, _event: &MouseUpEvent, cx: &mut Context<Self>) {
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
            self.animation_state = AppCardAnimation::SwipingOut {
                direction,
                progress: 0.0,
                start_offset: self.horizontal_offset,
            };
            self.animation_start_time = Some(std::time::Instant::now());
            self.schedule_animation_frame(cx);
        } else {
            let card_step = CARD_STEP;
            let current_card = (-self.scroll_offset / card_step).round();
            self.scroll_offset = -current_card * card_step;

            let max_scroll = (self.apps.len() as f32 - 1.0) * card_step;
            self.scroll_offset = self.scroll_offset.clamp(-max_scroll, 0.0);

            self.horizontal_offset = 0.0;
            self.dragged_card_index = None;
        }
        self.has_dragged = false;
        self.dragged_parent = false;
        cx.notify();
    }
    pub fn schedule_animation_frame(&self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, mut cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(16))
                .await;
            _ = this.update(cx, |this, cx| {
                this.update_animation(cx);
            });
        })
        .detach();
    }

    pub fn update_animation(&mut self, cx: &mut Context<Self>) {
        match self.animation_state {
            AppCardAnimation::Initial {
                start_time,
                start_offset,
            } => {
                let elapsed = start_time.elapsed().as_secs_f32();
                let progress = (elapsed / INITIAL_ANIMATION_DURATION).min(1.0);

                // Ease out cubic for smooth deceleration
                let eased = 1.0 - (1.0 - progress).powf(3.0);

                self.scroll_offset = start_offset + (0.0 - start_offset) * eased;

                if progress >= 1.0 {
                    self.scroll_offset = 0.0;
                    self.animation_state = AppCardAnimation::None;
                    self.animation_start_time = None;
                } else {
                    self.schedule_animation_frame(cx);
                }
                cx.notify();
            }
            AppCardAnimation::SwipingOut {
                direction,
                start_offset,
                ..
            } => {
                if let Some(start_time) = self.animation_start_time {
                    let elapsed = start_time.elapsed().as_secs_f32();
                    // Uses constant for single card swipe
                    let progress = (elapsed / ANIMATION_DURATION).min(1.0);

                    self.animation_state = AppCardAnimation::SwipingOut {
                        direction,
                        progress,
                        start_offset,
                    };

                    if progress >= 1.0 {
                        if let Some(card_index) = self.dragged_card_index {
                            println!("card_index: {} apps.len(): {}", card_index, self.apps.len());
                            if card_index < self.apps.len() {
                                self.apps.remove(card_index);
                            }
                        }
                        self.animation_state = AppCardAnimation::None;
                        self.animation_start_time = None;
                        self.horizontal_offset = 0.0;
                        self.dragged_card_index = None;
                        println!("apps len: {}", self.apps.len());
                        let max_scroll = (self.apps.len() as f32 - 1.0) * CARD_STEP;
                        if self.scroll_offset < -max_scroll {
                            self.scroll_offset = -max_scroll.max(0.0);
                        }
                    } else {
                        self.schedule_animation_frame(cx);
                    }
                    cx.notify();
                }
            }
            AppCardAnimation::ClearingAll { start_time } => {
                let elapsed = start_time.elapsed().as_secs_f32();
                let card_count = self.apps.len();

                // Calculate duration for the slowest card using constants
                let max_duration =
                    CLEAR_ANIMATION_BASE + ((card_count as f32 - 1.0) * CLEAR_ANIMATION_STEP);

                if elapsed >= max_duration {
                    self.apps.clear();
                    self.animation_state = AppCardAnimation::None;
                    self.scroll_offset = 0.0;
                    self.horizontal_offset = 0.0;
                    self.dragged_card_index = None;
                } else {
                    self.schedule_animation_frame(cx);
                }
                cx.notify();
            }
            AppCardAnimation::None => {}
        }
    }

    pub fn scroller_container(
        &self,
        window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) -> impl IntoElement {
        if self.drag_start.is_some() || self.is_animating() {
            window.request_animation_frame();
        }

        println!("scroller_container()");
        div()
            .size_full()
            .bg(gpui::black())
            .flex()
            .flex_col()
            .child(self.render_clear_all(cx))
            .child(
                div()
                    .w_full()
                    .h_3_4()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event, window, cx| {
                            this.on_mouse_down(event, cx);
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, event, window, cx| {
                        this.on_mouse_move(event, cx);
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, event, window, cx| {
                            this.on_mouse_up(event, cx);
                        }),
                    )
                    .child({
                        let mut container = div().absolute().left(px(30.)).bottom(px(320.));
                        for i in 0..self.apps.len() {
                            container = container.child(self.render_card(cx, i));
                        }
                        div()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(container)
                    }),
            )
    }
}
