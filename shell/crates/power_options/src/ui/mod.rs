use gpui::{prelude::FluentBuilder, *};

use crate::ui::icon::{Icon, IconName};
pub mod icon;

const INIT_ANIMATE_HEIGHT: f32 = 0.0; // Start from top

pub struct PowerOptions {
    pub power_off: bool,

    // Drag state
    drag_offset: Option<f32>,
    drag_start_pos: f32,
    position_y: f32, // Current Y position of the swipe card

    // Animation state
    initial_height: f32,
    is_initial_animation_done: bool,

    // Thresholds
    max_drag_distance: f32,

    // For upward swipe detection
    drag_start_y: f32,
    drag_start_mouse_y: f32, // Track actual mouse Y position at start
    is_dragging: bool,

    window_height: f32,
}

impl PowerOptions {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let this = Self {
            power_off: false,
            drag_offset: None,
            drag_start_pos: 0.0,
            position_y: 0.0,
            initial_height: INIT_ANIMATE_HEIGHT,
            is_initial_animation_done: false,
            max_drag_distance: 0.0,
            drag_start_y: 0.0,
            drag_start_mouse_y: 0.0,
            is_dragging: false,
            window_height: 0.0,
        };

        this
    }

    fn handle_upward_swipe(&mut self, cx: &mut Context<Self>) {
        println!("Go back - close power options");
        self.snap_to(0.0, cx);
    }

    fn animate_initial_reveal(&mut self, window_height: f32, cx: &mut Context<Self>) {
        let start_height = 0.0;
        let target_height = window_height / 2.0;
        let duration_ms = 1000.0;
        let start_time = std::time::Instant::now();

        cx.spawn(
            async move |this: WeakEntity<PowerOptions>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    if elapsed >= duration_ms {
                        this.update(cx, |this, cx| {
                            this.initial_height = target_height;
                            this.is_initial_animation_done = true;
                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3);
                    let current_height = start_height + (target_height - start_height) * ease;

                    this.update(cx, |this, cx| {
                        this.initial_height = current_height;
                        cx.notify();
                    })
                    .ok();

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(16))
                        .await;
                }
            },
        )
        .detach();
    }

    fn snap_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.position_y;
        let change = target - start;
        let duration_ms = 250.0;
        let start_time = std::time::Instant::now();
        let window_height = self.window_height;

        cx.spawn(
            async move |this: WeakEntity<PowerOptions>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    if elapsed >= duration_ms {
                        this.update(cx, |this, cx| {
                            this.position_y = target;

                            let total_height = this.initial_height + target;
                            if total_height >= window_height {
                                this.power_off = true;
                                println!("Power off triggered!");
                            }

                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3);
                    let current = start + (change * ease);

                    this.update(cx, |this, cx| {
                        this.position_y = current;
                        cx.notify();
                    })
                    .ok();

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(16))
                        .await;
                }
            },
        )
        .detach();
    }
}

impl Render for PowerOptions {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let size = window.bounds().size;
        let window_height = f32::from(size.height);

        if self.window_height == 0.0 {
            self.window_height = window_height;
            self.max_drag_distance = window_height;
            self.animate_initial_reveal(window_height, cx);
        }

        let initial_h = self.initial_height;

        let amber_card_height = if self.is_initial_animation_done {
            self.initial_height + self.position_y
        } else {
            self.initial_height
        };

        let arrow_height = if self.is_initial_animation_done {
            let remaining = window_height - amber_card_height;
            remaining.min(60.0).max(0.0)
        } else {
            0.0
        };

        div()
            .flex()
            .flex_col()
            .relative()
            .w(size.width)
            .h(size.height)
            .bg(rgb(0x1a1a1a))
            .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                if let Some(offset) = this.drag_offset {
                    let new_y = event.position.y.to_f64() as f32 - offset;
                    let max_position = window_height - initial_h;
                    this.position_y = new_y.clamp(0.0, max_position);
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, event: &MouseUpEvent, window, cx| {
                    if this.drag_offset.is_some() {
                        this.drag_offset = None;
                        this.is_dragging = false;

                        let current_mouse_y = event.position.y.to_f64() as f32;
                        let mouse_delta = current_mouse_y - this.drag_start_mouse_y;

                        if mouse_delta < -50.0 {
                            this.handle_upward_swipe(cx);
                            return;
                        }

                        let remaining_space = window_height - initial_h;
                        let half_remaining = remaining_space / 2.0;

                        let target = if this.position_y >= half_remaining {
                            remaining_space
                        } else {
                            0.0
                        };

                        this.snap_to(target, cx);
                        cx.notify();
                    }
                }),
            )
            .child(
                // Upper swipe area - AMBER CARD
                div()
                    .id("power-off-swipe-area")
                    .h(px(amber_card_height))
                    .w_full()
                    .bg(rgb(0x2d1f0f))
                    .rounded_b(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .when(self.is_initial_animation_done, |this| {
                        this.on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                cx.stop_propagation();

                                this.drag_start_y = this.position_y;
                                this.drag_start_pos = this.position_y;
                                this.drag_start_mouse_y = event.position.y.to_f64() as f32;
                                this.drag_offset =
                                    Some(event.position.y.to_f64() as f32 - this.position_y);
                                this.is_dragging = true;

                                cx.notify();
                            }),
                        )
                    })
                    .when(!self.power_off, |this| {
                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    Icon::new(IconName::PowerOff)
                                        .text_color(rgb(0xC67600))
                                        .size((px(32.), px(32.))),
                                )
                                .child(
                                    div()
                                        .text_xl()
                                        .text_color(rgb(0xd4a574))
                                        .child("Swipe to power off"),
                                ),
                        )
                    }),
            )
            .child(
                // Swipe indicator - arrow
                div()
                    .h(px(arrow_height))
                    .w_full()
                    .bg(rgb(0x000000))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .when(arrow_height > 20.0, |div| {
                        div.child(
                            Icon::new(IconName::DownArrow)
                                .text_color(rgb(0xC67600))
                                .size((px(26.), px(26.))),
                        )
                    }),
            )
            .child(
                // Lower area - black background
                div().flex_1().w_full().bg(rgb(0x000000)),
            )
    }
}
