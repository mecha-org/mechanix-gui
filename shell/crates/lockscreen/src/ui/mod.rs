use gpui::{prelude::FluentBuilder, *};
mod wallpaper;
mod wedges;
use wallpaper::wallpaper;
use wedges::{left_wedge, right_wedge};

// Threshold: if user swipes up more than this many pixels, hide the lockscreen
const UNLOCK_THRESHOLD: f32 = 80.0;

// The wedges area height
const WEDGES_AREA_HEIGHT: f32 = 67.0;

// Gap between the slider panel and the wedges (adjust to control spacing)
const PANEL_WEDGE_GAP: f32 = 10.0;

pub struct Lockscreen {
    drag_offset: Option<f32>,
    drag_start_mouse_y: f32,
    // position_y: 0 means panel at bottom (fully visible), negative means moved up (hidden)
    position_y: f32,
    window_height: f32,
    pub show: bool,
}

impl Lockscreen {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            drag_offset: None,
            drag_start_mouse_y: 0.0,
            position_y: 0.0,
            window_height: 0.0,
            show: true,
        }
    }

    fn update_input_regions(&self, window: &mut Window, show: bool, cx: &mut Context<Self>) {
        let size = window.bounds().size;
        let regions = if show {
            vec![Bounds {
                origin: point(px(0.0), px(0.0)),
                size,
            }]
        } else {
            vec![]
        };

        window.set_input_regions(Some(regions));
        cx.notify();
    }

    fn handle_unlock(&mut self, cx: &mut Context<Self>) {
        self.position_y = -self.window_height;
        self.show = false;
        cx.notify();
    }

    fn snap_back(&mut self, cx: &mut Context<Self>) {
        self.position_y = 0.0;
        cx.notify();
    }
}

impl Render for Lockscreen {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let size = window.bounds().size;
        let window_height = f32::from(size.height);
        let show = self.show;

        if self.window_height == 0.0 {
            self.window_height = window_height;
        }

        self.update_input_regions(window, show, cx);

        let overlay_color = hsla(0.0, 0.0, 0.0, 0.75);
        let text_color = hsla(0.0, 0.0, 1.0, 1.0);
        let handle_color = hsla(0.0, 0.0, 1.0, 0.9);

        // Panel top position: starts at 0 (top of screen), moves up (negative) when swiped
        let panel_top = self.position_y.min(0.0);
        // Panel height excludes the wedges area and gap at the bottom
        let panel_height = window_height - WEDGES_AREA_HEIGHT - PANEL_WEDGE_GAP;

        div()
            .size_full()
            .when(show, |this| {
                this.bg(overlay_color)
                    // Slider panel - stops above the wedges
                    .child(
                        div()
                            .absolute()
                            .top(px(panel_top))
                            .left_0()
                            .w(size.width)
                            .h(px(panel_height))
                            .overflow_hidden()
                            // Wallpaper background
                            .child(
                                div()
                                    .absolute()
                                    .inset_0()
                                    .child(wallpaper(size.width, px(panel_height))),
                            )
                            // Content overlay
                            .child(
                                div()
                                    .absolute()
                                    .inset_0()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_end()
                                    .pb(px(60.0))
                                    .gap_4()
                                    .child(
                                        div()
                                            .text_xl()
                                            .text_color(text_color)
                                            .child("Swipe up to unlock"),
                                    )
                                    .child(
                                        div()
                                            .w(px(64.0))
                                            .h(px(6.0))
                                            .rounded(px(3.0))
                                            .bg(handle_color)
                                            .mb(px(20.0)),
                                    ),
                            )
                            .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                                if let Some(offset) = this.drag_offset {
                                    let new_y = event.position.y.to_f64() as f32 - offset;
                                    // Only allow dragging upward (negative values)
                                    this.position_y = new_y.min(0.0);
                                    cx.notify();
                                }
                            }))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                    cx.stop_propagation();
                                    this.drag_start_mouse_y = event.position.y.to_f64() as f32;
                                    this.drag_offset =
                                        Some(event.position.y.to_f64() as f32 - this.position_y);
                                    cx.notify();
                                }),
                            )
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, event: &MouseUpEvent, _, cx| {
                                    if this.drag_offset.is_some() {
                                        this.drag_offset = None;

                                        let current_mouse_y = event.position.y.to_f64() as f32;
                                        let mouse_delta = current_mouse_y - this.drag_start_mouse_y;

                                        // If swiped up enough (negative delta), unlock
                                        if mouse_delta <= -UNLOCK_THRESHOLD {
                                            this.handle_unlock(cx);
                                        } else {
                                            this.snap_back(cx);
                                        }
                                    }
                                }),
                            ),
                    )
                    // Wedges container - fixed at bottom, both wedges overlap
                    .child(
                        div()
                            .absolute()
                            .bottom_0()
                            .left_0()
                            .w(size.width)
                            .h(px(WEDGES_AREA_HEIGHT))
                            // Left wedge (below right wedge in z-order)
                            .child(left_wedge(div()))
                            // Right wedge (overlaps left wedge, rendered on top)
                            .child(right_wedge(div())),
                    )
            })
    }
}
