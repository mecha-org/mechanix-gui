mod icon;
use gpui::prelude::*;
mod components;
use gpui::*;
use crate::prelude::app_manager::AppManagerMessage;
use crate::prelude::constants::*;
use crate::prelude::models::*;
use tokio::sync::mpsc;
const NAVBAR_SIZE: (f32, f32) = (120.0, 29.0);
const APP_SIZE: (f32, f32) = (540.0, 620.0);

impl Render for RunningApps {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bar_fixed_pos = APP_SIZE.1 - NAVBAR_SIZE.1;
        let current_bar_y = bar_fixed_pos + self.bar_drag_offset;

        div()
            .w_full()
            .h_full()
            .when(self.show_apps, |this| {
                this.child(
                    div()
                        .w_full()
                        .h_full()
                        .absolute()
                        .top(px(self.position))
                        .child(self.running_apps(cx))
                )
            })
            .on_mouse_move(
                cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                    if let Some(start_y) = this.bar_drag_start_y {
                        let current_y = event.position.y.to_f64() as f32;
                        let offset = current_y - start_y;
                        this.bar_drag_offset = offset.min(0.0);
                        cx.notify();
                    }
                })
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.bar_drag_start_y.is_some() {
                        if this.bar_drag_offset < -80.0 {
                            //long swipe
                            //Show running apps
                            this.update_input_regions(window, !this.show_apps);
                            this.show_apps = !this.show_apps;
                        } else {
                            //short swipe
                            //Mimize all apps
                        }
                        this.bar_drag_start_y = None;
                        this.snap_bar_to(0.0, cx);
                        cx.notify();
                    }
                })
            )
            .child(
                div()
                    .id("running-apps-navbar")
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .absolute()
                    .top(px(current_bar_y))
                    .h(px(NAVBAR_SIZE.1))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, _, cx| {
                            cx.stop_propagation();
                            this.bar_drag_start_y = Some(event.position.y.to_f64() as f32);
                            cx.notify();
                        })
                    )
                    .child(
                        div().bg(rgb(0x4d4d4d)).w(px(NAVBAR_SIZE.0)).h(px(4.0)) // img(IconName::Navbar.resolve()).id("running-apps-navbar")
                    )
            )
    }
}

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
        Empty
    }
}

impl RunningApps {
    pub fn new(message_tx: mpsc::Sender<AppManagerMessage>) -> Self {
        let apps = Vec::new();
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
            message_tx,
            position: 0.0,
            bar_drag_offset: 0.0,
            bar_drag_start_y: None,
            show_apps: false,
        }
    }

    fn calculate_center_offset_for_index_static(index: usize) -> Pixels {
        let card_position = (index as f32) * (CARD_WIDTH + CARD_GAP);
        let center_point = (CONTAINER_WIDTH - CARD_WIDTH) / 2.0;
        px(center_point - card_position)
    }

    pub fn calculate_center_offset(&self, index: usize) -> Pixels {
        Self::calculate_center_offset_for_index_static(index)
    }

    fn find_app_id(&self, card_id: usize) -> Option<String> {
        self.apps
            .iter()
            .find(|a| a.id == card_id)
            .map(|app| app.app_id.clone())
    }

    fn determine_drag_direction(&mut self, delta_x: Pixels, delta_y: Pixels) {
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
    }

    fn handle_horizontal_drag(&mut self, delta_x: Pixels, cx: &mut Context<Self>) {
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

    fn handle_vertical_drag(&mut self, delta_y: Pixels, cx: &mut Context<Self>) {
        if
            let Some(card_id) = self.dragging_card &&
            let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id)
        {
            app.offset_y = if delta_y <= px(0.0) { delta_y } else { px(0.0) };
            cx.notify();
        }
    }
}

impl RunningApps {
    // fn closed_pos() -> f32 {
    //     APP_SIZE.1 - NAVBAR_SIZE.1
    // }

    // fn snap_to(&mut self, target: f32, cx: &mut Context<Self>) {
    //     let start = self.position;
    //     let change = target - start;
    //     let duration_ms = 250.0; // Animation speed
    //     let start_time = std::time::Instant::now();

    //     cx.spawn(async move |this: WeakEntity<RunningApps>, cx: &mut AsyncApp| {
    //         loop {
    //             let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

    //             // Check if animation is done
    //             if elapsed >= duration_ms {
    //                 this.update(cx, |this, cx| {
    //                     this.position = target;
    //                     cx.notify();
    //                 }).ok();
    //                 break;
    //             }

    //             let t = (elapsed / duration_ms).clamp(0.0, 1.0);
    //             let ease = 1.0 - (1.0 - t).powi(3);
    //             let current = start + change * ease;

    //             this.update(cx, |this, cx| {
    //                 this.position = current;
    //                 cx.notify();
    //             }).ok();

    //             cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
    //         }
    //     }).detach();
    // }

    fn snap_bar_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.bar_drag_offset;
        let change = target - start;
        let duration_ms = 250.0; // Animation speed
        let start_time = std::time::Instant::now();

        cx.spawn(async move |this: WeakEntity<RunningApps>, cx: &mut AsyncApp| {
            loop {
                let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                // Check if animation is done
                if elapsed >= duration_ms {
                    this.update(cx, |this, cx| {
                        this.bar_drag_offset = target;
                        cx.notify();
                    }).ok();
                    break;
                }

                let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                let ease = 1.0 - (1.0 - t).powi(3);
                let current = start + change * ease;

                this.update(cx, |this, cx| {
                    this.bar_drag_offset = current;
                    cx.notify();
                }).ok();

                cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
            }
        }).detach();
    }

    fn update_input_regions(&self, window: &mut Window, open: bool) {
        let mut regions = Vec::new();

        if open {
            regions.push(Bounds {
                origin: point(px(0.0), px(0.0)),
                size: size(px(APP_SIZE.0), px(APP_SIZE.1)),
            });
        } else {
            regions.push(Bounds {
                origin: point(
                    px((APP_SIZE.0 - NAVBAR_SIZE.0) / 2.0),
                    px(APP_SIZE.1 - NAVBAR_SIZE.1)
                ),
                size: size(px(NAVBAR_SIZE.0), px(NAVBAR_SIZE.1)),
            });
        }
        window.set_input_regions(Some(regions));
    }

    fn running_apps(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
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
            .when(has_apps, |this| { this.child(self.scroller_container(cx, should_center)) })
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
            .child(self.render_footer(cx))
    }
}