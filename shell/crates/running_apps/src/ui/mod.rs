// ui/mod.rs
use gpui::prelude::*;
use gpui::*;

/// Running apps UI for a 540x620 embedded screen.
/// Optimized to avoid per-frame Vec clones and reduce allocations.
pub struct RunningApps {
    scroll_offset: Pixels,
    is_dragging: bool,
    drag_start_x: Pixels,
    drag_start_y: Pixels,
    drag_start_offset: Pixels,
    apps: Vec<AppCard>,
    dragging_card: Option<usize>, // stores app.id of the card being dragged
    drag_direction: Option<DragDirection>,
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
    card_color: Rgba,
}

impl RunningApps {
    pub fn new() -> Self {
        // Pre-allocate capacity if you expect more apps in future
        let mut apps = Vec::with_capacity(8);
        apps.push(AppCard { id: 0, offset_y: px(0.0), card_color: rgb(0x5a7c91) });
        apps.push(AppCard { id: 1, offset_y: px(0.0), card_color: rgb(0x6a491c) });
        apps.push(AppCard { id: 2, offset_y: px(0.0), card_color: rgb(0x9bcd9b) });
        apps.push(AppCard { id: 3, offset_y: px(0.0), card_color: rgb(0x654f60) });
        apps.push(AppCard { id: 4, offset_y: px(0.0), card_color: rgb(0x9024d1) });

        Self {
            scroll_offset: px(0.0),
            is_dragging: false,
            drag_start_x: px(0.0),
            drag_start_y: px(0.0),
            drag_start_offset: px(0.0),
            apps,
            dragging_card: None,
            drag_direction: None,
        }
    }

    /// Calculate the scroll offset needed to center a card at the given index
    fn calculate_center_offset_for_index(&self, index: usize) -> Pixels {
        const CARD_W: f32 = 250.0;
        const GAP: f32 = 8.0;
        const CONTAINER_W: f32 = 540.0;

        // Position of the card (left edge)
        let card_position = (index as f32) * (CARD_W + GAP);

        // Offset needed to center this card
        let center_point = (CONTAINER_W - CARD_W) / 2.0;

        px(center_point - card_position)
    }

    /// Find and snap to the nearest card based on current scroll position
    fn snap_to_nearest_card(&mut self) {
        if self.apps.is_empty() {
            return;
        }

        const CARD_W: f32 = 250.0;
        const GAP: f32 = 8.0;
        const CONTAINER_W: f32 = 540.0;

        let center_point = (CONTAINER_W - CARD_W) / 2.0;
        let card_step = CARD_W + GAP;

        // Convert scroll offset into f32
        let scroll_distance = self.scroll_offset.to_f64() as f32; // Assuming px(f32) type structure

        // Determine which card is closest to center
        // If scroll_offset is 0, first card should be centered.
        let nearest_index = ((-scroll_distance + center_point) / card_step).round() as isize;

        // Clamp the index
        let target_index = nearest_index.clamp(0, (self.apps.len() - 1) as isize) as usize;

        // Snap to calculated card
        self.scroll_offset = self.calculate_center_offset_for_index(target_index);
    }

    fn handle_clean_up(&mut self, cx: &mut Context<Self>) {
        // UI-only: clear the list of apps (no backend action).
        self.apps.clear();
        self.scroll_offset = px(0.0);
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

        // Lazily decide direction once movement > 5px
        if self.drag_direction.is_none() {
            let abs_delta_x = if delta_x >= px(0.0) { delta_x } else { px(0.0) - delta_x };
            let abs_delta_y = if delta_y >= px(0.0) { delta_y } else { px(0.0) - delta_y };

            if abs_delta_x > px(5.0) || abs_delta_y > px(5.0) {
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

                // Compute bounds such that first/last card can be centered.
                const CARD_W: f32 = 250.0;
                const GAP: f32 = 8.0;
                const PADDING: f32 = 0.0;
                const CONTAINER_W: f32 = 540.0;

                let center_offset = (CONTAINER_W - CARD_W) / 2.0;
                let max_scroll = px(center_offset - PADDING);

                // last card position (x) = (n-1) * (card_width + gap) + padding
                let last_index = if self.apps.is_empty() { 0 } else { self.apps.len() - 1 };
                let last_pos = (last_index as f32) * (CARD_W + GAP) + PADDING;
                let min_scroll = px(center_offset - last_pos);

                // clamp
                if self.scroll_offset > max_scroll {
                    self.scroll_offset = max_scroll;
                }
                if self.scroll_offset < min_scroll {
                    self.scroll_offset = min_scroll;
                }

                cx.notify();
            }
            Some(DragDirection::Vertical) => {
                // Vertical drag applied to the currently dragging card
                // Only allow upward dragging (negative delta_y)
                if let Some(card_id) = self.dragging_card {
                    if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                        // Only update if dragging upward (delta_y is negative)
                        if delta_y <= px(0.0) {
                            app.offset_y = delta_y;
                        } else {
                            // Dragging down - keep it at 0
                            app.offset_y = px(0.0);
                        }
                        cx.notify();
                    }
                }
            }
            None => {
                // not determined yet
            }
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
                        let threshold = px(-100.0);

                        if offset_y < threshold {
                            // remove the app
                            self.apps.retain(|a| a.id != card_id);

                            // Recalculate bounds and clamp scroll_offset if needed
                            if !self.apps.is_empty() {
                                const CARD_W: f32 = 250.0;
                                const GAP: f32 = 8.0;
                                const PADDING: f32 = 0.0;
                                const CONTAINER_W: f32 = 540.0;

                                let center_offset = (CONTAINER_W - CARD_W) / 2.0;
                                let last_index = self.apps.len() - 1;
                                let last_pos = (last_index as f32) * (CARD_W + GAP) + PADDING;
                                let min_scroll = px(center_offset - last_pos);

                                if self.scroll_offset < min_scroll {
                                    self.scroll_offset = min_scroll;
                                }
                            } else {
                                // no apps left -> reset scroll
                                self.scroll_offset = px(0.0);
                            }
                        } else {
                            // snap back
                            if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                                app.offset_y = px(0.0);
                            }
                        }
                    }
                }
            }
            Some(DragDirection::Horizontal) => {
                // Horizontal drag - snap to nearest card
                self.snap_to_nearest_card();
            }
            None => {
                // Direction was never determined (very small drag)
            }
        }

        // reset drag state
        self.is_dragging = false;
        self.dragging_card = None;
        self.drag_direction = None;
        cx.notify();
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
            // listen globally on the container for move/up events
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .when(has_apps, |this| {
                this.child(
                    // div()
                    //     .flex()
                    //     .items_center()
                    //     .justify_center()
                    //     .w_full()
                    //     .h_full()
                    //     .child(
                    // viewport container
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
                            let mut container = div().flex().flex_row().gap(px(8.0));

                            // Apply relative offset only when not centering
                            if !should_center {
                                container = container.relative().left(self.scroll_offset);
                            }

                            // Iterate by index to avoid cloning
                            for i in 0..self.apps.len() {
                                // local borrow to extract fields
                                let app_id = self.apps[i].id;
                                let offset_y = self.apps[i].offset_y;
                                let card_color = self.apps[i].card_color;

                                // Each card has its own on_mouse_down that references app_id
                                container = container.child(
                                    div()
                                        .flex()
                                        // .flex_shrink_0()
                                        .w(px(250.0))
                                        .h(px(290.0))
                                        .bg(card_color)
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
                                );
                            }

                            container
                        })
                )
                // )
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
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .px(px(32.0))
                            .py(px(12.0))
                            .bg(rgb(0x2d2d2d))
                            .rounded(px(8.0))
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|view, _event, _window, cx| {
                                    view.handle_clean_up(cx);
                                })
                            )
                            // .child(div().text_size(px(16.0)).child("🚀"))
                            .child(
                                div()
                                    .text_color(rgb(0x888888))
                                    .text_size(px(14.0))
                                    .child("Clean up")
                            )
                    )
            )
    }
}
