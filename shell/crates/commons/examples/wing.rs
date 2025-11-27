use commons::widgets::wing;
use gpui::prelude::*;
use gpui::*;

struct WingExample {
    clicked_wing: Option<usize>,
    hovered_wing: Option<usize>,
}

impl Render for WingExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let clicked_wing = self.clicked_wing;
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
                            .child("Wing Component Examples"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .child("Hover and click on the wings to see interactions"),
                    ),
            )
            // Row 1: Single wing (upper or lower only)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xaaaaaa))
                            .child("Single Wing - Upper or Lower Only"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_8()
                            .items_center()
                            .child(
                                div()
                                    .id("wing-3")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(3);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(3);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(25.0), px(12.0)));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0x10b981))
                                            .when(clicked_wing == Some(3), |wing| {
                                                wing.border_2().border_color(rgb(0xffffff))
                                            })
                                            .when(hovered_wing == Some(3), |wing| {
                                                wing.bg(rgb(0x34d399))
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
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(4);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.lower_wing_size(size(px(25.0), px(12.0)));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0xf59e0b))
                                            .when(clicked_wing == Some(4), |wing| {
                                                wing.border_2().border_color(rgb(0xffffff))
                                            })
                                            .when(hovered_wing == Some(4), |wing| {
                                                wing.bg(rgb(0xfbbf24))
                                            })
                                    }),
                            )
                            .child(
                                div()
                                    .id("wing-5")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(5);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(5);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(40.0), px(20.0)));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0x10b981))
                                            .when(clicked_wing == Some(5), |wing| {
                                                wing.border_2().border_color(rgb(0xffffff))
                                            })
                                            .when(hovered_wing == Some(5), |wing| {
                                                wing.bg(rgb(0x34d399))
                                            })
                                    }),
                            ),
                    ),
            )
            // Row 2: Wings with both upper and lower wings
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xaaaaaa))
                            .child("Both Upper and Lower Wings - Full Configuration"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_8()
                            .items_center()
                            .child(
                                div()
                                    .id("wing-9")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(9);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(9);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(15.0), px(8.0)));
                                        w.lower_wing_size(size(px(15.0), px(8.0)));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0xef4444))
                                            .when(clicked_wing == Some(9), |wing| {
                                                wing.border_2().border_color(rgb(0xffffff))
                                            })
                                            .when(hovered_wing == Some(9), |wing| {
                                                wing.bg(rgb(0xf87171))
                                            })
                                    }),
                            )
                            .child(
                                div()
                                    .id("wing-10")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(10);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(10);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(25.0), px(12.0)));
                                        w.lower_wing_size(size(px(25.0), px(12.0)));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0xef4444))
                                            .when(clicked_wing == Some(10), |wing| {
                                                wing.border_2().border_color(rgb(0xffffff))
                                            })
                                            .when(hovered_wing == Some(10), |wing| {
                                                wing.bg(rgb(0xf87171))
                                            })
                                    }),
                            )
                            .child(
                                div()
                                    .id("wing-11")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(11);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(11);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(35.0), px(18.0)));
                                        w.lower_wing_size(size(px(35.0), px(18.0)));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0xef4444))
                                            .when(clicked_wing == Some(11), |wing| {
                                                wing.border_2().border_color(rgb(0xffffff))
                                            })
                                            .when(hovered_wing == Some(11), |wing| {
                                                wing.bg(rgb(0xf87171))
                                            })
                                    }),
                            ),
                    ),
            )
            // Row 4: Asymmetric wings
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xaaaaaa))
                            .child("Asymmetric Wings - Different Upper and Lower Sizes"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_8()
                            .items_center()
                            .child(
                                div()
                                    .id("wing-12")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(12);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(12);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(30.0), px(10.0)));
                                        w.lower_wing_size(size(px(15.0), px(20.0)));
                                        w.w(px(100.0))
                                            .h(px(70.0))
                                            .bg(rgb(0x8b5cf6))
                                            .when(clicked_wing == Some(12), |wing| {
                                                wing.border_2().border_color(rgb(0xffffff))
                                            })
                                            .when(hovered_wing == Some(12), |wing| {
                                                wing.bg(rgb(0xa78bfa))
                                            })
                                    }),
                            )
                            .child(
                                div()
                                    .id("wing-13")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(13);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(13);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(40.0), px(25.0)));
                                        w.lower_wing_size(size(px(20.0), px(10.0)));
                                        w.w(px(100.0))
                                            .h(px(70.0))
                                            .bg(rgb(0x8b5cf6))
                                            .when(clicked_wing == Some(13), |wing| {
                                                wing.border_2().border_color(rgb(0xffffff))
                                            })
                                            .when(hovered_wing == Some(13), |wing| {
                                                wing.bg(rgb(0xa78bfa))
                                            })
                                    }),
                            ),
                    ),
            )
            // Row 5: Wings with borders and border radius combined
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0xaaaaaa))
                            .child("Wings with Borders and Border Radius - Combined Features"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_8()
                            .items_center()
                            .child(
                                div()
                                    .id("wing-14")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(14);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(14);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(25.0), px(12.0)));
                                        w.lower_wing_size(size(px(25.0), px(12.0)));
                                        w.border_width(px(2.0));
                                        w.border_radius(px(3.0));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0x06b6d4))
                                            .border_color(rgb(0xffffff))
                                            .when(clicked_wing == Some(14), |wing| {
                                                wing.bg(rgb(0x22d3ee))
                                            })
                                            .when(hovered_wing == Some(14), |wing| {
                                                wing.border_color(rgb(0xfbbf24))
                                            })
                                    }),
                            )
                            .child(
                                div()
                                    .id("wing-15")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(15);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(15);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(30.0), px(15.0)));
                                        w.lower_wing_size(size(px(30.0), px(15.0)));
                                        w.border_width(px(3.0));
                                        w.border_radius(px(6.0));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0x06b6d4))
                                            .border_color(rgb(0xffffff))
                                            .when(clicked_wing == Some(15), |wing| {
                                                wing.bg(rgb(0x22d3ee))
                                            })
                                            .when(hovered_wing == Some(15), |wing| {
                                                wing.border_color(rgb(0xfbbf24))
                                            })
                                    }),
                            )
                            .child(
                                div()
                                    .id("wing-16")
                                    .on_mouse_move(cx.listener(
                                        |this, _event: &MouseMoveEvent, _, _cx| {
                                            this.hovered_wing = Some(16);
                                        },
                                    ))
                                    .on_click(cx.listener(|this, _event: &ClickEvent, _, _cx| {
                                        this.clicked_wing = Some(16);
                                    }))
                                    .child({
                                        let mut w = wing();
                                        w.upper_wing_size(size(px(35.0), px(18.0)));
                                        w.lower_wing_size(size(px(35.0), px(18.0)));
                                        w.border_width(px(4.0));
                                        w.border_radius(px(10.0));
                                        w.w(px(80.0))
                                            .h(px(60.0))
                                            .bg(rgb(0x06b6d4))
                                            .border_color(rgb(0xffffff))
                                            .when(clicked_wing == Some(16), |wing| {
                                                wing.bg(rgb(0x22d3ee))
                                            })
                                            .when(hovered_wing == Some(16), |wing| {
                                                wing.border_color(rgb(0xfbbf24))
                                            })
                                    }),
                            ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| WingExample {
                    clicked_wing: None,
                    hovered_wing: None,
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}



