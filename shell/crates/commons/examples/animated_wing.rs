use commons::widgets::wing;
use gpui::prelude::*;
use gpui::*;
use rand::Rng;
use std::time::Duration;
use std::time::Instant;

struct AnimatedWingExample {
    animation_key: u64,
    current_wing_state: WingState,
    target_wing_state: WingState,
    is_expanded: bool,
    last_frame_time: Option<Instant>,
    fps: f32,
    frame_times: Vec<f32>,
}

#[derive(Clone, Copy)]
struct WingState {
    width: f32,
    height: f32,
    upper_wing_width: f32,
    upper_wing_height: f32,
    lower_wing_width: f32,
    lower_wing_height: f32,
    border_width: f32,
    border_radius: f32,
}

impl WingState {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(400.0..550.0);
        let height = rng.gen_range(300.0..450.0);

        // Generate upper wing dimensions
        let upper_wing_height = rng.gen_range(8.0..40.0);
        // Clamp upper wing width to: width - upper_wing_height
        let max_upper_wing_width = f32::max(width - upper_wing_height, 0.0);
        let upper_wing_width = f32::min(rng.gen_range(10.0..50.0), max_upper_wing_width);

        // Generate lower wing dimensions
        let lower_wing_height = rng.gen_range(8.0..40.0);
        // Clamp lower wing width to: width - lower_wing_height
        let max_lower_wing_width = f32::max(width - lower_wing_height, 0.0);
        let lower_wing_width = f32::min(rng.gen_range(10.0..50.0), max_lower_wing_width);

        // Generate random border width
        let border_width = rng.gen_range(1.0..8.0);

        // Generate random border radius
        let border_radius = rng.gen_range(0.0..15.0);

        Self {
            width,
            height,
            upper_wing_width,
            upper_wing_height,
            lower_wing_width,
            lower_wing_height,
            border_width,
            border_radius,
        }
    }

    fn initial() -> Self {
        Self {
            width: 50.0,
            height: 50.0,
            upper_wing_width: 10.0,
            upper_wing_height: 10.0,
            lower_wing_width: 10.0,
            lower_wing_height: 10.0,
            border_width: 2.0,
            border_radius: 5.0,
        }
    }

    fn target() -> Self {
        Self {
            width: 400.0,
            height: 500.0,
            upper_wing_width: 30.0,
            upper_wing_height: 30.0,
            lower_wing_width: 30.0,
            lower_wing_height: 30.0,
            border_width: 4.0,
            border_radius: 10.0,
        }
    }

    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            width: self.width + (other.width - self.width) * t,
            height: self.height + (other.height - self.height) * t,
            upper_wing_width: self.upper_wing_width
                + (other.upper_wing_width - self.upper_wing_width) * t,
            upper_wing_height: self.upper_wing_height
                + (other.upper_wing_height - self.upper_wing_height) * t,
            lower_wing_width: self.lower_wing_width
                + (other.lower_wing_width - self.lower_wing_width) * t,
            lower_wing_height: self.lower_wing_height
                + (other.lower_wing_height - self.lower_wing_height) * t,
            border_width: self.border_width + (other.border_width - self.border_width) * t,
            border_radius: self.border_radius + (other.border_radius - self.border_radius) * t,
        }
    }
}

impl Render for AnimatedWingExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("render()");
        // Calculate FPS
        let now = Instant::now();
        if let Some(last_time) = self.last_frame_time {
            let delta = now.duration_since(last_time).as_secs_f32();
            if delta > 0.0 {
                let current_fps = 1.0 / delta;
                self.frame_times.push(current_fps);

                // Keep only last 30 frames for averaging
                if self.frame_times.len() > 30 {
                    self.frame_times.remove(0);
                }

                // Calculate average FPS
                self.fps = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
            }
        }
        self.last_frame_time = Some(now);

        // Request next frame for continuous FPS updates
        cx.notify();

        let current = self.current_wing_state;
        let target = self.target_wing_state;

        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_8()
            .p_8()
            .bg(rgb(0x1e1e1e))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xffffff))
                            .child("Animated Wing Example"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .child("Click on the wing to toggle between 50x50 and 400x500"),
                    )
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .text_color(if self.fps >= 55.0 {
                                rgb(0x00ff00)
                            } else if self.fps >= 30.0 {
                                rgb(0xffff00)
                            } else {
                                rgb(0xff0000)
                            })
                            .child(format!("FPS: {:.1}", self.fps)),
                    ),
            )
            .child(
                div().flex().items_center().justify_center().flex_1().child(
                    div()
                        .id("animated-wing-container")
                        .on_click(cx.listener(|this, _event: &ClickEvent, _, cx| {
                            // Set current state to the previous target
                            this.current_wing_state = this.target_wing_state;
                            // Toggle between initial and target states
                            this.is_expanded = !this.is_expanded;
                            this.target_wing_state = if this.is_expanded {
                                WingState::target()
                            } else {
                                WingState::initial()
                            };
                            // Increment animation key to restart animation
                            this.animation_key += 1;
                            cx.notify();
                        }))
                        .cursor_pointer()
                        .child({
                            let animation_key = self.animation_key;

                            div().with_animation(
                                ElementId::Integer(animation_key),
                                Animation::new(Duration::from_millis(600)).with_easing(ease_in_out),
                                move |element, delta| {
                                    // Interpolate between current and target state
                                    let state = current.lerp(&target, delta);

                                    let mut w = wing();
                                    w.upper_wing_size(size(
                                        px(state.upper_wing_width),
                                        px(state.upper_wing_height),
                                    ));
                                    w.lower_wing_size(size(
                                        px(state.lower_wing_width),
                                        px(state.lower_wing_height),
                                    ));
                                    w.border_width(px(state.border_width));
                                    w.border_radius(px(state.border_radius));

                                    element.child(
                                        w.w(px(state.width))
                                            .h(px(state.height))
                                            .bg(rgb(0x3b82f6))
                                            .border_color(rgb(0xffffff)),
                                    )
                                },
                            )
                        }),
                ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_4()
                    .bg(rgb(0x2a2a2a))
                    .rounded_md()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xaaaaaa))
                            .child("Current Animation State:"),
                    )
                    .child(div().text_xs().text_color(rgb(0x888888)).child(format!(
                        "Size: {:.1}px × {:.1}px",
                        target.width, target.height
                    )))
                    .child(div().text_xs().text_color(rgb(0x888888)).child(format!(
                        "Upper Wing: {:.1}px × {:.1}px",
                        target.upper_wing_width, target.upper_wing_height
                    )))
                    .child(div().text_xs().text_color(rgb(0x888888)).child(format!(
                        "Lower Wing: {:.1}px × {:.1}px",
                        target.lower_wing_width, target.lower_wing_height
                    )))
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x888888))
                            .child(format!("Border Width: {:.1}px", target.border_width)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x888888))
                            .child(format!("Border Radius: {:.1}px", target.border_radius)),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                let initial_state = WingState::initial();
                cx.new(|_| AnimatedWingExample {
                    animation_key: 0,
                    current_wing_state: initial_state,
                    target_wing_state: WingState::target(),
                    is_expanded: false,
                    last_frame_time: None,
                    fps: 0.0,
                    frame_times: Vec::new(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
