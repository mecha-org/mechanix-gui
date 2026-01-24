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
    pub drag_bounds: Option<Bounds<Pixels>>,
    pub pattern: SliderPattern,
    pub id: ElementId,
}

pub enum SliderEvent {
    Change(f32),
}

impl SliderState {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            min: 0.,
            max: 100.,
            value: 0.,
            bounds: Bounds::default(),
            drag_bounds: None,
            pattern: SliderPattern::Dots,
            id: id.into(),
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
        let new_value = value.clamp(self.min, self.max);
        if (self.value - new_value).abs() < f32::EPSILON {
            return;
        }
        self.value = new_value;
        cx.emit(SliderEvent::Change(self.value));
        cx.notify();
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    fn value_to_pixels(&self, width: f32) -> f32 {
        if self.max == self.min || width == 0.0 {
            0.0
        } else {
            ((self.value - self.min) / (self.max - self.min)) * width
        }
    }

    fn pixels_to_value(&self, pixels: f32, width: f32) -> f32 {
        if self.max == self.min || width == 0.0 {
            self.min
        } else {
            let normalized = pixels.clamp(0.0, width) / width;
            self.min + normalized * (self.max - self.min)
        }
    }

    fn dot_fill_ratio(active_width: f32, idx: usize, unit_size: f32) -> f32 {
        let dot_start = idx as f32 * unit_size;
        let dot_end = dot_start + DOT_SIZE;

        if active_width <= dot_start {
            0.0
        } else if active_width >= dot_end {
            1.0
        } else {
            ((active_width - dot_start) / DOT_SIZE).clamp(0.0, 1.0)
        }
    }
    fn update_value_by_position(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Use drag_bounds if actively dragging, otherwise use bounds
        let bounds = self.drag_bounds.as_ref().unwrap_or(&self.bounds);
        let width = bounds.size.width;

        if width <= px(0.) {
            return;
        }

        let inner_x = (position.x - bounds.origin.x).clamp(px(0.), width);
        let new_value = self.pixels_to_value(inner_x.into(), width.into());
        self.set_value(new_value, window, cx);
    }

    fn start_drag(&mut self) {
        // Freeze bounds at drag start
        self.drag_bounds = Some(self.bounds);
    }

    fn end_drag(&mut self) {
        // Clear frozen bounds
        self.drag_bounds = None;
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
        let theme_inactive = colors.background_600;
        let theme_active = colors.accent_200;
        let theme_bg = colors.background_900;

        let state = self.state.read(cx);
        let width = self.width.unwrap_or(167.0);
        let height = self.height.unwrap_or(66.0);
        let active_width = state.value_to_pixels(width);
        let pattern = state.pattern;
        let entity_id = self.state.entity_id();

        let accent_dots_column = Icons::global(cx)
            .settings_drawer
            .slider_accent_dots_column
            .clone();

        let slider_gray_dot_column = Icons::global(cx)
            .settings_drawer
            .slider_gray_dot_column
            .clone();

        let active_dot_path = SharedString::from(accent_dots_column.to_string_lossy().to_string());
        let inactive_dot_path =
            SharedString::from(slider_gray_dot_column.to_string_lossy().to_string());

        let render_dot = |active: bool| {
            svg()
                .external_path(if active {
                    active_dot_path.clone()
                } else {
                    inactive_dot_path.clone()
                })
                .text_color(if active {
                    colors.accent_200
                } else {
                    colors.background_600
                })
                .w(px(DOT_SIZE))
                .h(px(height))
        };

        let unit_size = match pattern {
            SliderPattern::Dots => DOT_SIZE + DOT_GAP,
            SliderPattern::Bars => BAR_SEGMENT_WIDTH + BAR_GAP_WIDTH,
        };
        let total_segments = (width / unit_size).floor() as usize;

        let segments = (0..total_segments)
            .map(|idx| {
                let end_pos = (idx as f32 + 1.0) * unit_size;
                let is_active = end_pos <= active_width;

                match pattern {
                    SliderPattern::Dots => {
                        let dot_div = div().w(px(DOT_SIZE)).h_full().child(render_dot(is_active));
                        let gap_div = div().w(px(DOT_GAP)).h(px(height)).bg(theme_bg);
                        div()
                            .flex()
                            .flex_row()
                            .gap_0()
                            .children(vec![dot_div, gap_div])
                    }
                    SliderPattern::Bars => {
                        let bar_div =
                            div()
                                .w(px(BAR_SEGMENT_WIDTH))
                                .h(px(height))
                                .bg(if is_active {
                                    theme_active
                                } else {
                                    theme_inactive
                                });
                        let gap_div = div().w(px(BAR_GAP_WIDTH)).h(px(height)).bg(theme_bg);
                        div()
                            .flex()
                            .flex_row()
                            .gap_0()
                            .children(vec![bar_div, gap_div])
                    }
                }
            })
            .collect::<Vec<_>>();

        div()
            .id(self.id)
            .w(px(width))
            .h(px(height))
            .flex()
            .pl_2()
            .child(
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
                            .bg(theme_bg)
                            .children(segments)
                            .on_mouse_down(
                                MouseButton::Left,
                                window.listener_for(
                                    &self.state,
                                    move |state, e: &MouseDownEvent, window, cx| {
                                        cx.stop_propagation();
                                        state.update_value_by_position(e.position, window, cx);
                                    },
                                ),
                            )
                            .on_drag(DragThumb(entity_id), {
                                let state = self.state.clone();
                                move |drag, _, _, cx| {
                                    cx.stop_propagation();
                                    state.update(cx, |s, _| s.start_drag());
                                    cx.new(move |_| drag.clone())
                                }
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
                                                window,
                                                cx,
                                            );
                                        }
                                    }
                                },
                            ))
                            .on_mouse_up(
                                MouseButton::Left,
                                window.listener_for(&self.state, move |state, _, _, cx| {
                                    // Clear frozen bounds when drag ends
                                    state.end_drag();
                                    cx.notify();
                                }),
                            )
                            .child({
                                let state = self.state.clone();
                                canvas(
                                    move |bounds, _, cx| {
                                        // Only update bounds, never drag_bounds
                                        state.update(cx, |s, _| {
                                            s.bounds = bounds;
                                        });
                                    },
                                    |_, _, _, _| {},
                                )
                                .absolute()
                                .size_full()
                            }),
                    ),
            )
    }
}

#[derive(Clone)]
struct DragThumb(EntityId);

impl Render for DragThumb {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}
