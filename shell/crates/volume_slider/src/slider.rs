use gpui::*;

const DOT_SIZE: f32 = 2.0;
const DOT_GAP: f32 = 7.0;
const BAR_SEGMENT_WIDTH: f32 = 2.0;
const BAR_GAP_WIDTH: f32 = 4.0;
const INACTIVE_DOT_COLOR: u32 = 0x797979;
const INACTIVE_BAR_COLOR: u32 = 0x4D4D4D;
const ACTIVE_FILL_COLOR: u32 = 0xE9E9E9;
const ACTIVE_BAR_COLOR: u32 = 0xFFFFFF;
const BG_COLOR: u32 = 0x202020;
pub const SLIDER_CONTROL_WIDTH: f32 = 20.0;
pub const SLIDER_CONTROL_HEIGHT: f32 = 233.0;

#[derive(Clone, Copy, PartialEq)]
pub enum SliderPattern {
    Dots,
    Bars,
}

pub struct SliderState {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub bounds: Bounds<Pixels>,
    pub pattern: SliderPattern,
}

pub enum SliderEvent {
    Change(f32),
}

impl SliderState {
    pub fn new() -> Self {
        Self {
            min: 0.,
            max: 100.,
            value: 0.,
            bounds: Bounds::default(),
            pattern: SliderPattern::Dots,
        }
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn default_value(mut self, value: f32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn pattern(mut self, pattern: SliderPattern) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn set_value(&mut self, value: f32, _: &mut Window, cx: &mut Context<Self>) {
        self.value = value.clamp(self.min, self.max);
        cx.emit(SliderEvent::Change(self.value));
        cx.notify();
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    fn value_to_pixels(&self, height: f32) -> f32 {
        if self.max == self.min || height == 0.0 {
            return 0.0;
        }

        let normalized = (self.value - self.min) / (self.max - self.min);
        let pixels = normalized * height;
        pixels
    }

    fn pixels_to_value(&self, pixels: f32, height: f32) -> f32 {
        if self.max == self.min || height == 0.0 {
            return self.min;
        }

        let clamped = pixels.clamp(0.0, height);
        let normalized = clamped / height;
        self.min + (normalized * (self.max - self.min))
    }

    fn update_value_by_position(
        &mut self,
        position: Point<Pixels>,
        height: f32,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let bounds = self.bounds;
        // Get Y position relative to the slider bounds
        let inner_pos_y = position.y - bounds.top();
        let inverted_y = height - f32::from(inner_pos_y);
        let new_value = self.pixels_to_value(inverted_y, height);

        self.value = new_value.clamp(self.min, self.max);

        cx.emit(SliderEvent::Change(self.value));
        cx.notify();
    }
}

impl EventEmitter<SliderEvent> for SliderState {}

impl Render for SliderState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    state: Entity<SliderState>,
    width: Option<f32>,
    height: Option<f32>,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>, state: &Entity<SliderState>) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            width: None,
            height: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);

        let width = self.width.unwrap_or(SLIDER_CONTROL_WIDTH);
        let height = self.height.unwrap_or(SLIDER_CONTROL_HEIGHT);
        let active_height = state.value_to_pixels(height);
        let pattern = state.pattern;

        let entity_id = self.state.entity_id();
        let slider_height_copy = height;
        let slider_height_drag_copy = height;

        div()
            .id(self.id.clone())
            .w(px(width))
            .h(px(height))
            .flex()
            .child(match pattern {
                SliderPattern::Dots => {
                    let unit_size = DOT_SIZE + DOT_GAP;
                    let columns = (width / unit_size).floor() as usize;
                    let rows = (height / unit_size).floor() as usize;

                    // For vertical slider, build columns of dots instead of rows
                    let dot_grid = (0..columns).map(|_| {
                        let col_dots = (0..rows).map(|_| {
                            div()
                                .w(px(DOT_SIZE))
                                .h(px(DOT_SIZE))
                                .bg(rgb(INACTIVE_DOT_COLOR))
                        });
                        div().flex().flex_col().gap(px(DOT_GAP))
                        .children(col_dots)
                    });

                    div()
                        .flex()
                        .relative()
                        .w_full()
                        .h_full()
                        .items_center()
                        .child(
                            div()
                                .id("container-track")
                                .absolute()
                                .w_full()
                                .h_full()
                                .flex()
                                .flex_row()
                                .gap(px(DOT_GAP))
                                .bg(rgb(BG_COLOR))
                                .px(px(DOT_GAP))
                                .children(dot_grid)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    window.listener_for(
                                        &self.state,
                                        move |state, e: &MouseDownEvent, window, cx| {
                                            state.update_value_by_position(
                                                e.position,
                                                slider_height_copy,
                                                window,
                                                cx,
                                            );
                                        },
                                    ),
                                )
                                .on_drag(DragThumb(entity_id), |drag, _, _, cx| {
                                    cx.stop_propagation();
                                    cx.new(|_| drag.clone())
                                })
                                .on_drag_move(window.listener_for(
                                    &self.state,
                                    move |state, event: &DragMoveEvent<DragThumb>, window, cx| {
                                        match event.drag(cx) {
                                            DragThumb(id) => {
                                                if *id != entity_id {
                                                    return;
                                                }
                                                state.update_value_by_position(
                                                    event.event.position,
                                                    slider_height_drag_copy,
                                                    window,
                                                    cx,
                                                );
                                            }
                                        }
                                    },
                                ))
                                .child({
                                    let state = self.state.clone();
                                    canvas(
                                        move |bounds, _, cx| {
                                            state.update(cx, |s, _| s.bounds = bounds);
                                        },
                                        |_, _, _, _| {},
                                    )
                                    .absolute()
                                    .size_full()
                                }),
                        )
                        .child(
                            div()
                                .id("active-fill")
                                .absolute()
                                .left_0()
                                .bottom_0()
                                .w_full()
                                .h(px(active_height.max(0.0)))
                                .rounded(px(2.))
                                .bg(rgb(ACTIVE_FILL_COLOR)),
                        )
                }
                SliderPattern::Bars => {
                    let unit_height = BAR_SEGMENT_WIDTH + BAR_GAP_WIDTH;
                    let no_of_segments = (height / unit_height).floor() as usize;

                    let track_segments =
                        (0..no_of_segments).map(|idx| {
                            let segment_bottom_pos = idx as f32 * unit_height;
                            let segment_top_pos = segment_bottom_pos + BAR_SEGMENT_WIDTH;
                            let is_active = segment_top_pos <= active_height;

                            div()
                                .flex()
                                .flex_col()
                                .gap_0()
                                // Gap segment (on top in visual stack, but rendered first in column-reverse)
                                .child(div().h(px(BAR_GAP_WIDTH)).w(px(width)).bg(rgb(BG_COLOR)))
                                // Bar segment
                                .child(div().h(px(BAR_SEGMENT_WIDTH)).w(px(width)).bg(
                                    if is_active {
                                        rgb(ACTIVE_BAR_COLOR)
                                    } else {
                                        rgb(INACTIVE_BAR_COLOR)
                                    },
                                ))
                        });

                    div()
                        .flex()
                        .relative()
                        .w_full()
                        .h_full()
                        .items_center()
                        .child(
                            div()
                                .id("container-track")
                                .absolute()
                                .w_full()
                                .h_full()
                                .flex()
                                .flex_col_reverse()  // Stack from bottom to top
                                .gap_0()
                                .bg(rgb(BG_COLOR))
                                .children(track_segments)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    window.listener_for(
                                        &self.state,
                                        move |state, e: &MouseDownEvent, window, cx| {
                                            state.update_value_by_position(
                                                e.position,
                                                slider_height_copy,
                                                window,
                                                cx,
                                            );
                                        },
                                    ),
                                )
                                .on_drag(DragThumb(entity_id), |drag, _, _, cx| {
                                    cx.stop_propagation();
                                    cx.new(|_| drag.clone())
                                })
                                .on_drag_move(window.listener_for(
                                    &self.state,
                                    move |state, event: &DragMoveEvent<DragThumb>, window, cx| {
                                        match event.drag(cx) {
                                            DragThumb(id) => {
                                                if *id != entity_id {
                                                    return;
                                                }
                                                state.update_value_by_position(
                                                    event.event.position,
                                                    slider_height_drag_copy,
                                                    window,
                                                    cx,
                                                );
                                            }
                                        }
                                    },
                                ))
                                .child({
                                    let state = self.state.clone();
                                    canvas(
                                        move |bounds, _, cx| {
                                            state.update(cx, |s, _| s.bounds = bounds);
                                        },
                                        |_, _, _, _| {},
                                    )
                                    .absolute()
                                    .size_full()
                                }),
                        )
                }
            })
    }
}

#[derive(Clone)]
struct DragThumb(EntityId);

impl Render for DragThumb {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}