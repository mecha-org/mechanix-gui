use commons::widgets::wing;
use gpui::prelude::*;
use gpui::*;

struct WingWithChildrenExample;

impl Render for WingWithChildrenExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_8()
            .p_8()
            .bg(rgb(0x0f172a))
            .justify_center()
            .items_center()
            .child({
                let mut w = wing()
                    .h(px(220.0))
                    .w(px(320.0))
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .bg(rgb(0x1e293b))
                    .border_color(rgb(0x334155))
                    .child(
                        div()
                            .absolute()
                            .top(px(0.0))
                            .left(px(0.0))
                            .w(px(120.0))
                            .h(px(30.0))
                            .flex()
                            .justify_center()
                            .items_center()
                            .rounded(px(4.0))
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0xffffff))
                            .child("Top Wing"),
                    )
                    .child(
                        div()
                            .w(px(200.0))
                            .h(px(140.0))
                            .bg(rgb(0x334155))
                            .rounded(px(6.0))
                            .flex()
                            .flex_col()
                            .gap_3()
                            .justify_center()
                            .items_center()
                            .p_4()
                            .child(
                                div()
                                    .w(px(56.0))
                                    .h(px(56.0))
                                    .bg(rgb(0x0ea5e9))
                                    .rounded(px(8.0))
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_3xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xffffff))
                                            .child("W"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x94a3b8))
                                    .child("Wing Component"),
                            ),
                    )
                    .child(
                        div()
                            .absolute()
                            .bottom(px(0.0))
                            .right(px(0.0))
                            .w(px(120.0))
                            .h(px(30.0))
                            .flex()
                            .justify_center()
                            .items_center()
                            .rounded(px(4.0))
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0xffffff))
                            .child("Bottom Wing"),
                    );

                w.upper_wing_size(Size::new(px(120.0), px(30.0)));
                w.lower_wing_size(Size::new(px(120.0), px(30.0)));
                w.include_upper_wing_in_bounds(true);
                w.include_lower_wing_in_bounds(true);
                w.border_width(px(2.0));
                w.border_radius(px(6.0));
                w
            })
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(800.0), px(900.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| WingWithChildrenExample),
        )
        .unwrap();
        cx.activate(true);
    });
}
