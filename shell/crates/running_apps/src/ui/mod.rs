use std::time::Duration;
mod icon;
use gpui::prelude::*;
use gpui::*;
pub mod models;
pub mod constants;
pub use constants::*;
pub use models::{ RunningApps, AppCard, DragDirection };

use crate::ui::icon::{ Icon, IconName };

// Drag data structure
#[derive(Clone, Copy)]
struct CardDragData {
    start_position: Point<Pixels>,
}

impl CardDragData {
    fn new() -> Self {
        Self {
            start_position: Point::default(),
        }
    }
    fn position(mut self, pos: Point<Pixels>) -> Self {
        self.start_position = pos;
        self
    }
}

impl Render for CardDragData {
    fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
        // Empty render - we don't show a drag preview
        Empty
    }
}

impl RunningApps {
    pub fn new() -> Self {
        let mut apps = Vec::with_capacity(8);
        apps.push(AppCard {
            id: 0,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            app_name: "Firefox".to_string(),
            app_icon_path: IconName::Firefox,
        });
        apps.push(AppCard {
            id: 1,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            app_name: "Chromium".to_string(),
            app_icon_path: IconName::Chromium,
        });
        apps.push(AppCard {
            id: 2,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            app_name: "Kitty".to_string(),
            app_icon_path: IconName::Kitty,
        });
        apps.push(AppCard {
            id: 3,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            app_name: "Mecha".to_string(),
            app_icon_path: IconName::Mecha,
        });
        apps.push(AppCard {
            id: 4,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            app_name: "Files".to_string(),
            app_icon_path: IconName::Files,
        });

        let last_index = if apps.is_empty() { 0 } else { apps.len() - 1 };
        let initial_scroll_offset = Self::calculate_center_offset_for_index_static(last_index);

        Self {
            scroll_offset: initial_scroll_offset,
            target_scroll_offset: initial_scroll_offset,
            is_dragging: false,
            drag_start_x: px(0.0),
            drag_start_y: px(0.0),
            drag_start_offset: px(0.0),
            apps,
            dragging_card: None,
            drag_direction: None,
            is_animating: false,
            is_removing: false,
            removing_card_id: None,
            current_center_index: last_index,
            is_cleaning_up: false,
        }
    }

    fn calculate_center_offset_for_index_static(index: usize) -> Pixels {
        let card_position = (index as f32) * (CARD_WIDTH + CARD_GAP);
        let center_point = (CONTAINER_WIDTH - CARD_WIDTH) / 2.0;
        px(center_point - card_position)
    }

    fn snap_to_nearest_card_with_threshold(&mut self) {
        if self.apps.is_empty() {
            return;
        }

        // Calculate drag distance from start
        let drag_delta = self.scroll_offset - self.drag_start_offset;
        let drag_distance = drag_delta.to_f64() as f32;
        // Calculate threshold: 20% of card width + gap
        let card_step = CARD_WIDTH + CARD_GAP;
        let switch_threshold = card_step * HORIZONTAL_SWITCH_THRESHOLD;

        let mut target_index = self.current_center_index;

        if drag_distance > switch_threshold {
            // Dragged right (showing previous card) - move to left
            if target_index > 0 {
                target_index -= 1;
            }
        } else if drag_distance < -switch_threshold {
            // Dragged left (showing next card) - move to right
            if target_index < self.apps.len() - 1 {
                target_index += 1;
            }
        }
        // else: stay on current card (didn't drag enough)

        self.current_center_index = target_index;
        self.target_scroll_offset = Self::calculate_center_offset_for_index_static(target_index);
        self.is_animating = true;
        self.is_dragging = false;
    }

    fn animate_scroll(&mut self, cx: &mut Context<Self>) {
        if !self.is_animating || self.is_dragging {
            return;
        }

        let diff = self.target_scroll_offset - self.scroll_offset;
        let threshold = px(0.5);

        if diff.abs() < threshold {
            self.scroll_offset = self.target_scroll_offset;
            self.is_animating = false;
        } else {
            let lerp_factor = 0.2;
            self.scroll_offset = self.scroll_offset + diff * lerp_factor;
            cx.spawn(async move |this, cx| {
                cx.background_executor().timer(Duration::from_millis(16)).await;
                let _ = this.update(cx, |this, cx| {
                    this.animate_scroll(cx);
                    cx.notify();
                });
            }).detach();
        }
    }

    fn animate_card_removal(&mut self, card_id: usize, cx: &mut Context<Self>) {
        if !self.is_removing {
            return;
        }

        if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
            let diff: Pixels = app.target_offset_y - app.offset_y;
            let threshold = px(0.5);

            if diff.abs() < threshold {
                // Animation complete - remove the card
                app.offset_y = app.target_offset_y;

                // Find which index was removed
                let removed_index = self.apps.iter().position(|a| a.id == card_id);

                self.apps.retain(|a| a.id != card_id);
                self.is_removing = false;
                self.removing_card_id = None;

                // Adjust current_center_index after removal
                if let Some(removed_idx) = removed_index {
                    // If we removed a card to the left of center, adjust index
                    if removed_idx < self.current_center_index {
                        self.current_center_index = self.current_center_index.saturating_sub(1);
                    } else if
                        // If we removed the centered card or one to the right, keep same index
                        // (the next card will slide into that position)
                        self.current_center_index >= self.apps.len() &&
                        !self.apps.is_empty()
                    {
                        self.current_center_index = self.apps.len() - 1;
                    }
                }

                // After removing, smoothly animate scroll position
                if !self.apps.is_empty() {
                    self.target_scroll_offset = Self::calculate_center_offset_for_index_static(
                        self.current_center_index
                    );
                    self.is_animating = true;
                    self.animate_scroll(cx);
                } else {
                    self.scroll_offset = px(0.0);
                    self.target_scroll_offset = px(0.0);
                    self.current_center_index = 0;
                }
                cx.notify();
            } else {
                let lerp_factor = 0.2;
                app.offset_y = app.offset_y + diff * lerp_factor;

                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(Duration::from_millis(16)).await;
                    let _ = this.update(cx, |this, cx| {
                        this.animate_card_removal(card_id, cx);
                        cx.notify();
                    });
                }).detach();
            }
        }
    }

    fn animate_clean_up(&mut self, cx: &mut Context<Self>) {
        if !self.is_cleaning_up {
            return;
        }

        // Check if all cards have reached their target
        let all_done = self.apps.iter().all(|app| {
            let diff = app.target_offset_y - app.offset_y;
            diff.abs() < px(1.0)
        });

        if all_done {
            // All animations complete - clear all cards
            self.apps.clear();
            self.scroll_offset = px(0.0);
            self.target_scroll_offset = px(0.0);
            self.is_cleaning_up = false;
            self.current_center_index = 0;
            cx.notify();
        } else {
            // Continue animating all cards upward
            let lerp_factor = 0.2;
            for app in self.apps.iter_mut() {
                let diff = app.target_offset_y - app.offset_y;
                app.offset_y = app.offset_y + diff * lerp_factor;
            }

            cx.spawn(async move |this, cx| {
                cx.background_executor().timer(Duration::from_millis(16)).await;
                let _ = this.update(cx, |this, cx| {
                    this.animate_clean_up(cx);
                    cx.notify();
                });
            }).detach();
        }
    }

    fn handle_clean_up(&mut self, cx: &mut Context<Self>) {
        if self.apps.is_empty() {
            return;
        }

        // Set all cards to animate upward with staggered delays
        for (i, app) in self.apps.iter_mut().enumerate() {
            // Stagger the animation by adding delay based on index
            let delay_offset = (i as f32) * 30.0; // 30px stagger between cards
            app.target_offset_y = px(VERTICAL_TARGET_THRESHOLD - delay_offset);
        }

        self.is_cleaning_up = true;
        self.is_animating = false;
        self.is_removing = false;

        // Start the animation
        self.animate_clean_up(cx);
    }

    fn handle_card_mouse_down(
        &mut self,
        app_id: usize,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>
    ) {
        self.is_dragging = true;
        self.is_animating = false;
        self.dragging_card = Some(app_id);
        self.drag_start_x = event.position.x;
        self.drag_start_y = event.position.y;
        self.drag_start_offset = self.scroll_offset;
        self.drag_direction = None;
        cx.stop_propagation();
    }

    fn handle_mouse_move(
        &mut self,
        event: &DragMoveEvent<CardDragData>,
        _window: &mut Window,
        cx: &mut Context<Self>
    ) {
        if !self.is_dragging {
            return;
        }

        // Use the stored start position from drag data
        let delta_x = event.event.position.x - self.drag_start_x;
        let delta_y = event.event.position.y - self.drag_start_y;
        if self.drag_direction.is_none() {
            let abs_delta_x = delta_x.abs();
            let abs_delta_y = delta_y.abs();

            if
                abs_delta_x > px(DRAG_DETECTION_THRESHOLD) ||
                abs_delta_y > px(DRAG_DETECTION_THRESHOLD)
            {
                self.drag_direction = if abs_delta_x > abs_delta_y {
                    Some(DragDirection::Horizontal)
                } else {
                    Some(DragDirection::Vertical)
                };
            }
        }

        match self.drag_direction {
            Some(DragDirection::Horizontal) => {
                // Horizontal scrolling (swipe) - free scroll during drag
                self.scroll_offset = self.drag_start_offset + delta_x;

                // Compute bounds such that first/last card can be centered
                let center_offset = (CONTAINER_WIDTH - CARD_WIDTH) / 2.0;
                let max_scroll = px(center_offset - PADDING);

                // last card position (x) = (n-1) * (card_width + gap) + padding
                let last_index = if self.apps.is_empty() { 0 } else { self.apps.len() - 1 };
                let min_scroll = Self::calculate_center_offset_for_index_static(last_index);

                self.scroll_offset = self.scroll_offset.clamp(min_scroll, max_scroll);
                cx.notify();
            }
            Some(DragDirection::Vertical) => {
                if let Some(card_id) = self.dragging_card {
                    if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                        app.offset_y = if delta_y <= px(0.0) { delta_y } else { px(0.0) };
                        cx.notify();
                    }
                }
            }
            // No direction determined yet
            None => {}
        }
    }

    fn handle_mouse_up(&mut self, _: &CardDragData, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.is_dragging {
            return;
        }

        match self.drag_direction {
            Some(DragDirection::Vertical) => {
                if let Some(card_id) = self.dragging_card {
                    if let Some(pos) = self.apps.iter().position(|a| a.id == card_id) {
                        let offset_y = self.apps[pos].offset_y;

                        if offset_y < px(VERTICAL_DISMISS_THRESHOLD) {
                            if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                                app.target_offset_y = px(VERTICAL_TARGET_THRESHOLD);
                                self.is_removing = true;
                                self.removing_card_id = Some(card_id);
                                self.animate_card_removal(card_id, cx);
                            }
                        } else {
                            if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                                app.target_offset_y = px(0.0);
                                let card_id_copy = card_id;
                                cx.spawn(async move |this, cx| {
                                    let _ = this.update(cx, |this, cx| {
                                        this.animate_snap_back(card_id_copy, cx);
                                    });
                                }).detach();
                            }
                        }
                    }
                }
            }
            Some(DragDirection::Horizontal) => {
                // Horizontal drag - snap to nearest card
                self.snap_to_nearest_card_with_threshold();
                self.animate_scroll(cx);
            }
            None => {}
        }

        self.is_dragging = false;
        self.dragging_card = None;
        self.drag_direction = None;
        cx.notify();
    }

    fn animate_snap_back(&mut self, card_id: usize, cx: &mut Context<Self>) {
        if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
            let diff = app.target_offset_y - app.offset_y;
            let threshold = px(0.5);

            if diff.abs() < threshold {
                app.offset_y = px(0.0);
                app.target_offset_y = px(0.0);
            } else {
                let lerp_factor = 0.25;
                app.offset_y = app.offset_y + diff * lerp_factor;

                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(Duration::from_millis(16)).await;
                    let _ = this.update(cx, |this, cx| {
                        this.animate_snap_back(card_id, cx);
                        cx.notify();
                    });
                }).detach();
            }
        }
    }
}

impl Render for RunningApps {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_apps = !self.apps.is_empty();
        let should_center = self.apps.len() == 1;

        div()
            .flex()
            .w_full()
            .h_full()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(rgb(0x000000))
            .when(has_apps, |this| {
                this.child(
                    div()
                        .id("running_apps")
                        .flex()
                        .w_full()
                        .h_full()
                        .overflow_x_hidden()
                        .on_drop(cx.listener(RunningApps::handle_mouse_up))
                        .items_center()
                        .when(should_center, |d| d.justify_center())
                        .child({
                            let mut container = div().flex().flex_row().gap(px(CARD_GAP));

                            if !should_center {
                                container = container.relative().left(self.scroll_offset);
                            }

                            for i in 0..self.apps.len() {
                                let app_id = self.apps[i].id;
                                let offset_y = self.apps[i].offset_y;
                                let app_icon_path = self.apps[i].app_icon_path.clone();
                                let app_name = self.apps[i].app_name.clone();

                                container = container.child(
                                    div()
                                        .id("card")
                                        .relative()
                                        .flex()
                                        .w(px(CARD_WIDTH))
                                        .h(px(CARD_HEIGHT))
                                        .justify_center()
                                        .items_center()

                                        .child(
                                            div()
                                                .absolute()
                                                .left_0()
                                                .top_0()
                                                .child(
                                                    Icon::from(IconName::BgApp).size((
                                                        px(CARD_WIDTH),
                                                        px(CARD_HEIGHT),
                                                    ))
                                                )
                                        )
                                        .child(
                                            Icon::from(app_icon_path.clone())
                                                .size((px(40.0), px(40.0)))
                                                .text_color(rgb(0xf4f4f4))
                                        )
                                        .rounded(px(16.0))
                                        .relative()
                                        .top(offset_y)
                                        .cursor_pointer()
                                        .on_drag_move(cx.listener(Self::handle_mouse_move))

                                        .on_drag(
                                            CardDragData::new(),
                                            move |_: &CardDragData, pos, _, cx| {
                                                let data = CardDragData::new().position(pos);
                                                cx.new(|_| data)
                                            }
                                        )
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |view, event, window, cx| {
                                                view.handle_card_mouse_down(
                                                    app_id,
                                                    event,
                                                    window,
                                                    cx
                                                );
                                            })
                                        )
                                        .child(
                                            div()
                                                .absolute()
                                                .top(px(8.0))
                                                .left(px(8.0))
                                                .w(px(100.0))
                                                .h(px(16.0))
                                                .justify_start()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_2()
                                                        .h(px(16.0))
                                                        .w(px(106.0))
                                                        .child(
                                                            div()
                                                                .rounded(px(3.2))
                                                                .w(px(16.0))
                                                                .h(px(16.0))
                                                                .flex()
                                                                .justify_center()
                                                                .items_center()
                                                                .child(
                                                                    Icon::from(
                                                                        app_icon_path.clone()
                                                                    )
                                                                        .size((px(16.0), px(16.0)))
                                                                        .text_color(rgb(0xf4f4f4))
                                                                )
                                                        )
                                                        .child(
                                                            div()
                                                                .font_weight(FontWeight(400.0))
                                                                .text_size(px(16.0))
                                                                .text_color(rgb(0xf4f4f4))
                                                                .child(app_name)
                                                        )
                                                )
                                        )
                                );
                            }

                            container
                        })
                )
            })
            .when(!has_apps, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_16()
                        .child(
                            div()
                                .text_color(rgb(0x666666))
                                .text_size(px(16.0))
                                .line_height(px(24.0))
                                .text_center()
                                .font_weight(FontWeight(400.0))
                                .max_w(px(300.0))
                                .child("There are no apps or droids")
                                .child(div().child("you are looking for_"))
                        )
                )
            })
            // fixed positioned footer button
            .child(
                div()
                    .absolute()
                    .bottom_16()
                    .child({
                        let is_enabled = !self.apps.is_empty();
                        let mut btn = div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .w(px(109.0))
                            .h(px(36.0))
                            .px(px(8.0))
                            .py(px(12.0))
                            .rounded(px(8.0))
                            .bg(if is_enabled { rgb(0x363636) } else { rgb(0x202020) })
                            .cursor(
                                if is_enabled {
                                    CursorStyle::PointingHand
                                } else {
                                    CursorStyle::default()
                                }
                            )
                            .child(
                                Icon::from(IconName::CleanUp)
                                    .size((px(20.0), px(20.0)))
                                    .text_color(
                                        if is_enabled {
                                            rgb(0xf4f4f4)
                                        } else {
                                            rgb(0x4d4d4d)
                                        }
                                    )
                            )
                            .child(
                                div()
                                    .text_color(
                                        if is_enabled {
                                            rgb(0xf4f4f4)
                                        } else {
                                            rgb(0x4d4d4d)
                                        }
                                    )
                                    .opacity(if is_enabled { 1.0 } else { 0.4 })
                                    .text_size(px(16.0))
                                    .child("Clean up")
                            );

                        if is_enabled {
                            btn = btn.on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|view, _event, _window, cx| {
                                    view.handle_clean_up(cx);
                                })
                            );
                        }

                        btn
                    })
            )
    }
}
