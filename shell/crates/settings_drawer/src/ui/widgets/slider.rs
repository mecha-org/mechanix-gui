use gpui::*;
use icons::prelude::Icons;
use theme::prelude::Theme;

const DOT_SIZE: f32 = 3.0;
const DOT_GAP: f32 = 6.0;
const BAR_SEGMENT_WIDTH: f32 = 2.2;
const BAR_GAP_WIDTH: f32 = 5.0;

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
        let inner_pos = position.x - self.bounds.left();
        let new_value = self
            .pixels_to_value(inner_pos.into(), width)
            .clamp(self.min, self.max);

        let previous_value = self.value;

        let prev_u32 = previous_value as u32;
        let new_u32 = new_value as u32;
        if prev_u32 == new_u32 {
            return;
        }

        self.value = new_value;
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
        let colors = Theme::global(cx).colors.clone();

        let theme_inactive_bar_color = colors.background_600;
        let theme_active_bar_color = colors.accent_200;
        let theme_bg_color = colors.background_900;

        let state = self.state.read(cx);

        let width = self.width.unwrap_or(167.0);
        let height = self.height.unwrap_or(66.0);

        let active_width = state.value_to_pixels(width);

        let pattern = state.pattern;
        let entity_id = self.state.entity_id();
        let slider_width_copy = width;
        let slider_width_drag_copy = width;

        let accent_dots_column = Icons::global(cx)
            .settings_drawer
            .slider_accent_dots_column
            .clone();

        let slider_gray_dot_column = Icons::global(cx)
            .settings_drawer
            .slider_gray_dot_column
            .clone();

        div()
            .id(self.id.clone())
            .w(px(width))
            .h(px(height))
            .flex()
            .pl_2()
            .child(match pattern {
                SliderPattern::Dots => {
                    let unit_size = DOT_SIZE + DOT_GAP;
                    let total_columns = (width / unit_size).floor() as usize;

                    let active_dot_path =
                        SharedString::from(accent_dots_column.to_string_lossy().to_string());
                    let inactive_dot_path =
                        SharedString::from(slider_gray_dot_column.to_string_lossy().to_string());

                    let render_dot = |is_active: bool| {
                        svg()
                            .external_path(if is_active {
                                active_dot_path.clone()
                            } else {
                                inactive_dot_path.clone()
                            })
                            .text_color(if is_active {
                                colors.accent_200
                            } else {
                                colors.background_600
                            })
                            .w(px(DOT_SIZE))
                            .h(px(height))
                    };

                    let dots = (0..total_columns).map(|idx| {
                        let dot_end_pos = (idx as f32 * unit_size) + DOT_SIZE;
                        let is_active = dot_end_pos <= active_width;

                        div().w(px(DOT_SIZE)).h_full().child(render_dot(is_active))
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
                                .bg(theme_bg_color)
                                .children(dots)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    window.listener_for(
                                        &self.state,
                                        move |state, e: &MouseDownEvent, window, cx| {
                                            cx.stop_propagation();
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
                                        let DragThumb(id) = event.drag(cx);
                                        if *id != entity_id {
                                            return;
                                        }

                                        cx.stop_propagation();
                                        state.update_value_by_position(
                                            event.event.position,
                                            slider_width_drag_copy,
                                            window,
                                            cx,
                                        );
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
                                        theme_active_bar_color
                                    } else {
                                        theme_inactive_bar_color
                                    },
                                ))
                                .child(div().w(px(BAR_GAP_WIDTH)).h(px(height)).bg(theme_bg_color))
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
                                .bg(theme_bg_color)
                                .children(track_segments)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    window.listener_for(
                                        &self.state,
                                        move |state, e: &MouseDownEvent, window, cx| {
                                            cx.stop_propagation();
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
                                                cx.stop_propagation();
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
