use commons::widgets::wing;
use gpui::prelude::*;
use gpui::*;

struct WingBoundsDemo {
    hovered_wing: Option<usize>,
}

impl Render for WingBoundsDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let hovered_wing = self.hovered_wing;

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
                            .child("Wing Bounds Control Demo"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .child("Hover over the wings to see how bounds inclusion affects hit detection"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_12()
                    .justify_center()
                    // Column 1: Both wings included in bounds (default)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xaaaaaa))
                                    .text_center()
                                    .child("Both Wings In Bounds"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .text_center()
                                    .child("(Default behavior)"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0()
                                    .child(
                                        div()
                                            .id("wing-1")
                                            .on_mouse_move(cx.listener(
                                                |this, _event: &MouseMoveEvent, _, _cx| {
                                                    this.hovered_wing = Some(1);
                                                },
                                            ))
                                            .child({
                                                let mut w = wing();
                                                w.upper_wing_size(size(px(30.0), px(20.0)));
                                                w.lower_wing_size(size(px(30.0), px(20.0)));
                                                // Both wings included in bounds (default)
                                                w.include_upper_wing_in_bounds(true);
                                                w.include_lower_wing_in_bounds(true);
                                                w.border_width(px(2.0));
                                                w.w(px(100.0))
                                                    .h(px(80.0))
                                                    .bg(rgb(0x3b82f6))
                                                    .border_color(rgb(0x60a5fa))
                                                    .when(hovered_wing == Some(1), |wing| {
                                                        wing.bg(rgb(0x60a5fa))
                                                            .border_color(rgb(0xfbbf24))
                                                    })
                                            }),
                                    )
                                    .child(
                                        div()
                                            .id("wing-2")
                                            .on_mouse_move(cx.listener(
                                                |this, _event: &MouseMoveEvent, _, _cx| {
                                                    this.hovered_wing = Some(2);
                                                },
                                            ))
                                            .child({
                                                let mut w = wing();
                                                w.upper_wing_size(size(px(30.0), px(20.0)));
                                                w.lower_wing_size(size(px(30.0), px(20.0)));
                                                // Both wings included in bounds (default)
                                                w.include_upper_wing_in_bounds(true);
                                                w.include_lower_wing_in_bounds(true);
                                                w.border_width(px(2.0));
                                                w.w(px(100.0))
                                                    .h(px(80.0))
                                                    .bg(rgb(0x10b981))
                                                    .border_color(rgb(0x34d399))
                                                    .when(hovered_wing == Some(2), |wing| {
                                                        wing.bg(rgb(0x34d399))
                                                            .border_color(rgb(0xfbbf24))
                                                    })
                                            }),
                                    ),
                            ),
                    )
                    // Column 2: Upper wing excluded from bounds
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xaaaaaa))
                                    .text_center()
                                    .child("Upper Wing Excluded"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .text_center()
                                    .child("(No hit detection on top)"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0()
                                    .child(
                                        div()
                                            .id("wing-3")
                                            .on_mouse_move(cx.listener(
                                                |this, _event: &MouseMoveEvent, _, _cx| {
                                                    this.hovered_wing = Some(3);
                                                },
                                            ))
                                            .child({
                                                let mut w = wing();
                                                w.upper_wing_size(size(px(30.0), px(20.0)));
                                                w.lower_wing_size(size(px(30.0), px(20.0)));
                                                // Upper wing excluded from bounds
                                                w.include_upper_wing_in_bounds(false);
                                                w.include_lower_wing_in_bounds(true);
                                                w.border_width(px(2.0));
                                                w.w(px(100.0))
                                                    .h(px(80.0))
                                                    .bg(rgb(0x3b82f6))
                                                    .border_color(rgb(0x60a5fa))
                                                    .when(hovered_wing == Some(3), |wing| {
                                                        wing.bg(rgb(0x60a5fa))
                                                            .border_color(rgb(0xfbbf24))
                                                    })
                                            }),
                                    )
                                    .child(
                                        div()
                                            .id("wing-4")
                                            .on_mouse_move(cx.listener(
                                                |this, _event: &MouseMoveEvent, _, _cx| {
                                                    this.hovered_wing = Some(4);
                                                },
                                            ))
                                            .child({
                                                let mut w = wing();
                                                w.upper_wing_size(size(px(30.0), px(20.0)));
                                                w.lower_wing_size(size(px(30.0), px(20.0)));
                                                // Upper wing excluded from bounds
                                                w.include_upper_wing_in_bounds(false);
                                                w.include_lower_wing_in_bounds(true);
                                                w.border_width(px(2.0));
                                                w.w(px(100.0))
                                                    .h(px(80.0))
                                                    .bg(rgb(0x10b981))
                                                    .border_color(rgb(0x34d399))
                                                    .when(hovered_wing == Some(4), |wing| {
                                                        wing.bg(rgb(0x34d399))
                                                            .border_color(rgb(0xfbbf24))
                                                    })
                                            }),
                                    ),
                            ),
                    )
                    // Column 3: Lower wing excluded from bounds
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xaaaaaa))
                                    .text_center()
                                    .child("Lower Wing Excluded"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .text_center()
                                    .child("(No hit detection on bottom)"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0()
                                    .child(
                                        div()
                                            .id("wing-5")
                                            .on_mouse_move(cx.listener(
                                                |this, _event: &MouseMoveEvent, _, _cx| {
                                                    this.hovered_wing = Some(5);
                                                },
                                            ))
                                            .child({
                                                let mut w = wing();
                                                w.upper_wing_size(size(px(30.0), px(20.0)));
                                                w.lower_wing_size(size(px(30.0), px(20.0)));
                                                // Lower wing excluded from bounds
                                                w.include_upper_wing_in_bounds(true);
                                                w.include_lower_wing_in_bounds(false);
                                                w.border_width(px(2.0));
                                                w.w(px(100.0))
                                                    .h(px(80.0))
                                                    .bg(rgb(0x3b82f6))
                                                    .border_color(rgb(0x60a5fa))
                                                    .when(hovered_wing == Some(5), |wing| {
                                                        wing.bg(rgb(0x60a5fa))
                                                            .border_color(rgb(0xfbbf24))
                                                    })
                                            }),
                                    )
                                    .child(
                                        div()
                                            .id("wing-6")
                                            .on_mouse_move(cx.listener(
                                                |this, _event: &MouseMoveEvent, _, _cx| {
                                                    this.hovered_wing = Some(6);
                                                },
                                            ))
                                            .child({
                                                let mut w = wing();
                                                w.upper_wing_size(size(px(30.0), px(20.0)));
                                                w.lower_wing_size(size(px(30.0), px(20.0)));
                                                // Lower wing excluded from bounds
                                                w.include_upper_wing_in_bounds(true);
                                                w.include_lower_wing_in_bounds(false);
                                                w.border_width(px(2.0));
                                                w.w(px(100.0))
                                                    .h(px(80.0))
                                                    .bg(rgb(0x10b981))
                                                    .border_color(rgb(0x34d399))
                                                    .when(hovered_wing == Some(6), |wing| {
                                                        wing.bg(rgb(0x34d399))
                                                            .border_color(rgb(0xfbbf24))
                                                    })
                                            }),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .mt_8()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xaaaaaa))
                            .child("How to test:"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x666666))
                            .child("• Column 1: Try hovering over any part of the wings - they all respond"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x666666))
                            .child("• Column 2: The upper wing extensions don't respond to hover"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x666666))
                            .child("• Column 3: The lower wing extensions don't respond to hover"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x888888))
                            .mt_2()
                            .child("Notice how the wings still render identically but behave differently!"),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1000.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| WingBoundsDemo {
                    hovered_wing: None,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
