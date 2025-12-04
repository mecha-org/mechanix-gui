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

    fn value_to_pixels(&self, width: f32) -> f32 {
        if self.max == self.min || width == 0.0 {
            return 0.0;
        }

        let normalized = (self.value - self.min) / (self.max - self.min);
        let pixels = normalized * width;
        pixels
    }

    fn pixels_to_value(&self, pixels: f32, width: f32) -> f32 {
        if self.max == self.min || width == 0.0 {
            return self.min;
        }

        let clamped = pixels.clamp(0.0, width);
        let normalized = clamped / width;
        self.min + (normalized * (self.max - self.min))
    }

    fn update_value_by_position(
        &mut self,
        position: Point<Pixels>,
        width: f32,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let bounds = self.bounds;
        let inner_pos = position.x - bounds.left();
        let new_value = self.pixels_to_value(inner_pos.into(), width);

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

        let width = self.width.unwrap_or(172.0);
        let height = self.height.unwrap_or(56.0);
        let active_width = state.value_to_pixels(width);
        let pattern = state.pattern;

        let entity_id = self.state.entity_id();
        let slider_width_copy = width;
        let slider_width_drag_copy = width;

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

                    let dot_grid = (0..rows).map(|_| {
                        let row_dots = (0..columns).map(|_| {
                            div()
                                .w(px(DOT_SIZE))
                                .h(px(DOT_SIZE))
                                .bg(rgb(INACTIVE_DOT_COLOR))
                        });
                        div().flex().flex_row().gap(px(DOT_GAP))
                        .children(row_dots)
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
                                .flex_col()
                                .gap(px(DOT_GAP))
                                .bg(rgb(BG_COLOR))
                                // .p(px(DOT_GAP))
                                .py(px(DOT_GAP))
                                .children(dot_grid)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    window.listener_for(
                                        &self.state,
                                        move |state, e: &MouseDownEvent, window, cx| {
                                            state.update_value_by_position(
                                                e.position,
                                                slider_width_copy,
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
                                                    slider_width_drag_copy,
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
                                .top_0()
                                .h_full()
                                .w(px(active_width.max(0.0)))
                                .rounded(px(2.))
                                .bg(rgb(ACTIVE_FILL_COLOR)),
                        )
                }
                SliderPattern::Bars => {
                    let unit_width = BAR_SEGMENT_WIDTH + BAR_GAP_WIDTH;
                    let no_of_segments = (width / unit_width).floor() as usize;

                    let track_segments =
                        (0..no_of_segments).map(|idx| {
                            let segment_end_pos = (idx as f32 * unit_width) + BAR_SEGMENT_WIDTH;
                            let is_active = segment_end_pos <= active_width;

                            div()
                                .flex()
                                .flex_row()
                                .gap_0()
                                .child(div().w(px(BAR_SEGMENT_WIDTH)).h(px(height)).bg(
                                    if is_active {
                                        rgb(ACTIVE_BAR_COLOR)
                                    } else {
                                        rgb(INACTIVE_BAR_COLOR)
                                    },
                                ))
                                .child(div().w(px(BAR_GAP_WIDTH)).h(px(height)).bg(rgb(BG_COLOR)))
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
                                                slider_width_copy,
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
                                                    slider_width_drag_copy,
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
