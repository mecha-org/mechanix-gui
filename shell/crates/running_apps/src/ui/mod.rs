use std::time::Duration;
mod icon;
use gpui::prelude::*;
use gpui::*;

use crate::ui::icon::{ Icon, IconName };

// Layout constants
const CARD_WIDTH: f32 = 250.0;
const CARD_HEIGHT: f32 = 290.0;
const CARD_GAP: f32 = 8.0;
const CONTAINER_WIDTH: f32 = 540.0;
const PADDING: f32 = 0.0;

// Drag thresholds
const DRAG_DETECTION_THRESHOLD: f32 = 5.0;
const VERTICAL_DISMISS_THRESHOLD: f32 = -100.0;

pub struct RunningApps {
    scroll_offset: Pixels,
    is_dragging: bool,
    drag_start_x: Pixels,
    target_scroll_offset: Pixels,
    drag_start_y: Pixels,
    drag_start_offset: Pixels,
    apps: Vec<AppCard>,
    dragging_card: Option<usize>, // stores app.id of the card being dragged
    drag_direction: Option<DragDirection>,
    is_animating: bool,
    is_removing: bool, // Flag to indicate a card is being removed with animation
    removing_card_id: Option<usize>, // ID of the card being removed
}

#[derive(Clone, Copy, PartialEq)]
enum DragDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
struct AppCard {
    id: usize,
    offset_y: Pixels, // used for vertical drag animation
    target_offset_y: Pixels, // Target for vertical animation
    icon_color: Rgba,
    app_name: String,
    app_icon_path: String,
}

impl RunningApps {
    pub fn new() -> Self {
        // Pre-allocate capacity if you expect more apps in future
        let mut apps = Vec::with_capacity(8);
        apps.push(AppCard {
            id: 0,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            icon_color: rgb(0x5a7c91),
            app_name: "Notes".to_string(),
            app_icon_path: IconName::Files.resolve().to_string(),
        });
        apps.push(AppCard {
            id: 1,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            icon_color: rgb(0x6a491c),
            app_name: "Files".to_string(),
            app_icon_path: IconName::Files.resolve().to_string(),
        });
        apps.push(AppCard {
            id: 2,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            icon_color: rgb(0x9bcd9b),
            app_name: "Settings".to_string(),
            app_icon_path: IconName::Files.resolve().to_string(),
        });
        apps.push(AppCard {
            id: 3,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            icon_color: rgb(0x654f60),
            app_name: "Music".to_string(),
            app_icon_path: IconName::Files.resolve().to_string(),
        });
        apps.push(AppCard {
            id: 4,
            offset_y: px(0.0),
            target_offset_y: px(0.0),
            icon_color: rgb(0x9024d1),
            app_name: "Camera".to_string(),
            app_icon_path: IconName::Files.resolve().to_string(),
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
        }
    }

    /// Static version of calculate_center_offset_for_index for use in new()
    fn calculate_center_offset_for_index_static(index: usize) -> Pixels {
        let card_position = (index as f32) * (CARD_WIDTH + CARD_GAP);
        let center_point = (CONTAINER_WIDTH - CARD_WIDTH) / 2.0;

        px(center_point - card_position)
    }

    /// Calculate the scroll offset needed to center a card at the given index
    fn calculate_center_offset_for_index(&self, index: usize) -> Pixels {
        Self::calculate_center_offset_for_index_static(index)
    }

    /// Find and snap to the nearest card based on current scroll position
    fn snap_to_nearest_card(&mut self) {
        if self.apps.is_empty() {
            return;
        }

        let center_point = (CONTAINER_WIDTH - CARD_WIDTH) / 2.0;
        let card_step = CARD_WIDTH + CARD_GAP;

        // Convert scroll offset into f32
        let scroll_distance = self.scroll_offset.to_f64() as f32;

        // Determine which card is closest to center
        let nearest_index = ((-scroll_distance + center_point) / card_step).round() as isize;

        // Clamp the index
        let target_index = nearest_index.clamp(0, (self.apps.len() - 1) as isize) as usize;
        // Snap to calculated card
        self.target_scroll_offset = self.calculate_center_offset_for_index(target_index);
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
            // Smooth lerp with 20% interpolation factor
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
                self.apps.retain(|a| a.id != card_id);
                self.is_removing = false;
                self.removing_card_id = None;

                // Recalculate bounds and clamp scroll_offset if needed
                if !self.apps.is_empty() {
                    // let center_offset = (CONTAINER_WIDTH - CARD_WIDTH) / 2.0;
                    let last_index = self.apps.len() - 1;
                    let min_scroll = Self::calculate_center_offset_for_index_static(last_index);

                    // let last_pos = (last_index as f32) * (CARD_WIDTH + CARD_GAP) + PADDING;
                    // let min_scroll = px(center_offset - last_pos);

                    if self.scroll_offset < min_scroll {
                        self.scroll_offset = min_scroll;
                        self.target_scroll_offset = min_scroll;
                    }
                } else {
                    self.scroll_offset = px(0.0);
                    self.target_scroll_offset = px(0.0);
                }
                cx.notify();
            } else {
                // Continue animation with smooth lerp
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

    fn handle_clean_up(&mut self, cx: &mut Context<Self>) {
        // UI-only: clear the list of apps (no backend action).
        self.apps.clear();
        self.scroll_offset = px(0.0);
        self.target_scroll_offset = px(0.0);
        self.is_animating = false;
        self.is_removing = false;
        self.removing_card_id = None;
        cx.notify();
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
        // stop propagation so parent doesn't treat this as viewport drag
        cx.stop_propagation();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>
    ) {
        if !self.is_dragging {
            return;
        }

        let delta_x = event.position.x - self.drag_start_x;
        let delta_y = event.position.y - self.drag_start_y;

        // Lazily decide direction once movement > threshold
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
                // let last_pos = (last_index as f32) * (CARD_WIDTH + CARD_GAP) + PADDING;
                // let min_scroll = px(center_offset - last_pos);

                self.scroll_offset = self.scroll_offset.clamp(min_scroll, max_scroll);
                cx.notify();
            }
            Some(DragDirection::Vertical) => {
                // Vertical drag applied to the currently dragging card
                // Only allow upward dragging (negative delta_y)
                if let Some(card_id) = self.dragging_card {
                    if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                        app.offset_y = if delta_y <= px(0.0) { delta_y } else { px(0.0) };
                        cx.notify();
                    }
                }
            }
            None => {}
        }
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>
    ) {
        if !self.is_dragging {
            return;
        }

        // Handle based on drag direction
        match self.drag_direction {
            Some(DragDirection::Vertical) => {
                // Vertical drag - check threshold and remove
                if let Some(card_id) = self.dragging_card {
                    if let Some(pos) = self.apps.iter().position(|a| a.id == card_id) {
                        let offset_y = self.apps[pos].offset_y;

                        if offset_y < px(VERTICAL_DISMISS_THRESHOLD) {
                            // Start removal animation
                            if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                                // Set target to animate card off screen (beyond top)
                                app.target_offset_y = px(-400.0);
                                self.is_removing = true;
                                self.removing_card_id = Some(card_id);
                                self.animate_card_removal(card_id, cx);
                            }
                        } else {
                            // Snap back to original position with animation
                            if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                                app.target_offset_y = px(0.0);
                                // Animate snap back
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
                self.snap_to_nearest_card();
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
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))

            .when(has_apps, |this| {
                this.child(
                    div()
                        .flex()
                        .w_full()
                        .h_full()
                        .overflow_x_hidden()
                        .items_center()
                        .when(should_center, |d| d.justify_center())
                        .child({
                            // container that holds the cards in a horizontal row
                            // Avoid cloning the apps vec; index directly.
                            let mut container = div().flex().flex_row().gap(px(CARD_GAP));

                            // Apply relative offset only when not centering
                            if !should_center {
                                container = container.relative().left(self.scroll_offset);
                            }

                            // Iterate by index to avoid cloning
                            for i in 0..self.apps.len() {
                                let app_id = self.apps[i].id;
                                let offset_y = self.apps[i].offset_y;
                                let app_icon_path = self.apps[i].app_icon_path.clone();
                                let app_name = self.apps[i].app_name.clone();
                                let icon_color = self.apps[i].icon_color;

                                // Each card has its own on_mouse_down that references app_id
                                container = container.child(
                                    div()
                                        .flex()
                                        .w(px(CARD_WIDTH))
                                        .h(px(CARD_HEIGHT))
                                        .child(
                                            Icon::from(IconName::BgApp).size((
                                                px(CARD_WIDTH),
                                                px(CARD_HEIGHT),
                                            ))
                                        )
                                        .rounded(px(16.0))
                                        .relative()
                                        .top(offset_y)
                                        .cursor_pointer()
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
                                                                .bg(icon_color)
                                                                .flex()
                                                                .justify_center()
                                                                .items_center()
                                                                .child(img(app_icon_path))
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
