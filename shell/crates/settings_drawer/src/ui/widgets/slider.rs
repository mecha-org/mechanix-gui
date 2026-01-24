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

    fn value_to_pixels(&self, track_width: f32) -> f32 {
        if track_width <= 0.0 {
            0.0
        } else {
            // value is 0-100, convert to pixels
            (self.value / 100.0) * track_width
        }
    }

    /// Converts a pixel position to a percentage value (0-100)
    fn pixels_to_value(&self, pixels: f32, track_width: f32) -> f32 {
        if track_width <= 0.0 {
            self.min
        } else {
            (pixels / track_width) * 100.0
        }
    }

    /// Updates the slider value based on mouse position
    fn update_value_by_position(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let bounds = self.drag_bounds.as_ref().unwrap_or(&self.bounds);
        let track_width = bounds.size.width;

        if track_width <= px(0.) {
            return;
        }

        let inner_x = (position.x - bounds.origin.x).clamp(px(0.), track_width);
        let new_value = self.pixels_to_value(inner_x.into(), track_width.into());
        self.set_value(new_value, window, cx);
    }

    fn start_drag(&mut self) {
        self.drag_bounds = Some(self.bounds);
    }

    fn end_drag(&mut self) {
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
    fn render_dots_pattern(
        &self,
        track_width: f32,
        height: f32,
        window: &mut Window,
        cx: &mut App,
    ) -> Div {
        let colors = &Theme::global(cx).colors;
        let icons = Icons::global(cx);
        let state = self.state.read(cx);

        let unit_size = DOT_SIZE + DOT_GAP;
        let total_columns = (track_width / unit_size).floor() as usize;

        let fill_width = state.value_to_pixels(track_width);

        let inactive_dot_path = SharedString::from(
            icons
                .settings_drawer
                .slider_gray_dot_column
                .to_string_lossy()
                .to_string(),
        );
        let active_dot_path = SharedString::from(
            icons
                .settings_drawer
                .slider_accent_dots_column
                .to_string_lossy()
                .to_string(),
        );

        let entity_id = self.state.entity_id();

        let inactive_dots = (0..total_columns).map(|_idx| {
            div().w(px(DOT_SIZE)).h_full().child(
                svg()
                    .external_path(inactive_dot_path.clone())
                    .text_color(colors.background_600)
                    .w(px(DOT_SIZE))
                    .h(px(height)),
            )
        });

        let active_dots = (0..total_columns).map(|_idx| {
            div().w(px(DOT_SIZE)).h_full().child(
                svg()
                    .external_path(active_dot_path.clone())
                    .text_color(colors.accent_200)
                    .w(px(DOT_SIZE))
                    .h(px(height)),
            )
        });

        div()
            .flex()
            .relative()
            .w_full()
            .h_full()
            .items_center()
            .child(
                // Background track with inactive dots
                div()
                    .id("inactive-track")
                    .absolute()
                    .w_full()
                    .h_full()
                    .flex()
                    .flex_row()
                    .gap(px(DOT_GAP))
                    .bg(colors.background_900)
                    .children(inactive_dots),
            )
            .child(
                // Active overlay with clipped active dots
                div()
                    .id("active-overlay")
                    .absolute()
                    .w(px(fill_width))
                    .h_full()
                    .flex()
                    .flex_row()
                    .gap(px(DOT_GAP))
                    .overflow_hidden()
                    .children(active_dots),
            )
            .child(
                // Interactive layer for mouse events
                div()
                    .id("interactive-layer")
                    .absolute()
                    .w_full()
                    .h_full()
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
                            let DragThumb(id) = event.drag(cx);
                            if *id != entity_id {
                                return;
                            }
                            cx.stop_propagation();
                            state.update_value_by_position(event.event.position, window, cx);
                        },
                    ))
                    .on_drop(window.listener_for(
                        &self.state,
                        move |state, _event: &DragMoveEvent<DragThumb>, _window, cx| {
                            cx.stop_propagation();
                            
                            state.end_drag();
                            cx.notify();
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

    /// Renders the bars pattern slider with overlay approach
    fn render_bars_pattern(
        &self,
        track_width: f32,
        height: f32,
        window: &mut Window,
        cx: &mut App,
    ) -> Div {
        let colors = &Theme::global(cx).colors;
        let state = self.state.read(cx);

        let unit_width = BAR_SEGMENT_WIDTH + BAR_GAP_WIDTH;
        let no_of_segments = (track_width / unit_width).floor() as usize;

        let fill_width = state.value_to_pixels(track_width);
        let entity_id = self.state.entity_id();

        let inactive_segments = (0..no_of_segments).map(|_idx| {
            div()
                .flex()
                .flex_row()
                .gap_0()
                .child(
                    div()
                        .w(px(BAR_SEGMENT_WIDTH))
                        .h(px(height))
                        .bg(colors.background_600),
                )
                .child(
                    div()
                        .w(px(BAR_GAP_WIDTH))
                        .h(px(height))
                        .bg(colors.background_900),
                )
        });

        // Render active bars for the filled overlay
        let active_segments = (0..no_of_segments).map(|_idx| {
            div()
                .flex()
                .flex_row()
                .gap_0()
                .child(
                    div()
                        .w(px(BAR_SEGMENT_WIDTH))
                        .h(px(height))
                        .bg(colors.accent_200),
                )
                .child(
                    div()
                        .w(px(BAR_GAP_WIDTH))
                        .h(px(height))
                        .bg(colors.background_900),
                )
        });

        div()
            .flex()
            .relative()
            .w_full()
            .h_full()
            .items_center()
            .child(
                // Background track with inactive bars
                div()
                    .id("inactive-track")
                    .absolute()
                    .w_full()
                    .h_full()
                    .flex()
                    .flex_row()
                    .gap_0()
                    .bg(colors.background_900)
                    .children(inactive_segments),
            )
            .child(
                // Active overlay with clipped active bars
                div()
                    .id("active-overlay")
                    .absolute()
                    .w(px(fill_width))
                    .h_full()
                    .flex()
                    .flex_row()
                    .gap_0()
                    .overflow_hidden()
                    .children(active_segments),
            )
            .child(
                // Interactive layer for mouse events
                div()
                    .id("interactive-layer")
                    .absolute()
                    .w_full()
                    .h_full()
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
                            let DragThumb(id) = event.drag(cx);
                            if *id != entity_id {
                                return;
                            }
                            cx.stop_propagation();
                            state.update_value_by_position(event.event.position, window, cx);
                        },
                    ))
                    .on_drop(window.listener_for(
                        &self.state,
                        move |state, _event: &DragMoveEvent<DragThumb>, _window, cx| {
                            state.end_drag();
                            cx.notify();
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
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);

        let width = self.width.unwrap_or(167.0);
        let height = self.height.unwrap_or(66.0);
        let pattern = state.pattern;

        div()
            .id(self.id.clone())
            .w(px(width))
            .h(px(height))
            .flex()
            .pl_2()
            .child(match pattern {
                SliderPattern::Dots => self.render_dots_pattern(width, height, window, cx),
                SliderPattern::Bars => self.render_bars_pattern(width, height, window, cx),
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
