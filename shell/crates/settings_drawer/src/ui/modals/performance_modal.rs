use futures::{SinkExt, channel::mpsc};
use gpui::*;

use crate::{
    prelude::*,
    ui::{
        icon::{Icon, IconName},
        widgets::IconButton,
    },
};

const ROW_HEIGHT: f32 = 60.0;

#[derive(Debug)]
struct PerformanceMode {
    performance_mode: String,
    text: String,
    is_active: bool,
}

pub struct PerformanceWindow {
    pub title: String,
}

impl PerformanceWindow {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl Render for PerformanceWindow {
    fn render(&mut self, _window: &mut Window, ctx: &mut Context<Self>) -> impl IntoElement {
        let performance_modes = vec![
            PerformanceMode {
                performance_mode: "high".to_string(),
                text: "High performance".to_string(),
                is_active: true,
            },
            PerformanceMode {
                performance_mode: "saver".to_string(),
                text: "Battery saver performance".to_string(),
                is_active: false,
            },
        ];

        div()
            .flex()
            .flex_col()
            .bg(rgb(0x151515))
            .size_full()
            .border_1()
            .rounded_xl()
            .border_color(rgb(AMBER_900))
            .child(
                // Header
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .p_4()
                    .h(px(ROW_HEIGHT))
                    .border_b_1()
                    .bg(rgb(DARK_NEUTRAL_800))
                    .flex_shrink_0()
                    .child(
                        div()
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(DARK_NEUTRAL_0))
                            .child(self.title.clone()),
                    ),
            )
            .child(div().flex().flex_col().flex_1().relative().children(
                performance_modes.iter().enumerate().map(|(idx, mode)| {
                    let is_active = mode.is_active;

                    let icon_color = if is_active {
                        rgb(AMBER_600)
                    } else {
                        rgb(DARK_NEUTRAL_0)
                    };
                    let mut icon = IconName::PowerModeBalanced;

                    if mode.performance_mode == "high" {
                        icon = IconName::HighPower;
                    } else if mode.performance_mode == "saver" {
                        icon = IconName::SavingPower;
                    };

                    let connect_div = div().child(
                        Icon::new(IconName::ConnectedIcon)
                            .size((px(24.), px(24.)))
                            .text_color(rgb(AMBER_600)),
                    );

                    let main_div = if is_active {
                        div()
                            .id(("mode", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(60.))
                            .px_4()
                            .bg(if is_active {
                                rgba(AMBER_600_10)
                            } else {
                                rgba(AMBER_900)
                            })
                            .border_y_1()
                            .border_color(rgb(AMBER_900))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .text_color(rgb(0xD2D2D2))
                                    .text_lg()
                                    .text_align(TextAlign::Left)
                                    .child(
                                        div().pr_2().child(
                                            Icon::new(icon)
                                                .size((px(28.), px(28.)))
                                                .text_color(icon_color),
                                        ),
                                    )
                                    .child(mode.text.clone()),
                            )
                            .child(if mode.is_active { connect_div } else { div() })
                    } else {
                        div()
                            .id(("mode", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(60.))
                            .px_4()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .text_color(rgb(0xD2D2D2))
                                    .text_lg()
                                    .text_align(TextAlign::Left)
                                    .child(
                                        div().pr_2().child(
                                            Icon::new(icon)
                                                .size((px(28.), px(28.)))
                                                .text_color(icon_color),
                                        ),
                                    )
                                    .child(mode.text.clone()),
                            )
                            .on_click(ctx.listener(move |_, _, _, _| {
                                println!("mode clicked...");
                            }))
                    };

                    main_div
                }),
            ))
            // Footer
            .child(
                div()
                    .id("id_settings")
                    .flex()
                    .flex_row()
                    .items_end()
                    .justify_start()
                    .border_t_1()
                    .border_color(rgb(DARK_NEUTRAL_700))
                    .h(px(ROW_HEIGHT))
                    .p_4()
                    .flex_shrink_0()
                    .child(
                        Icon::new(IconName::Settings)
                            .size((px(28.), px(28.)))
                            .text_color(rgb(AMBER_600)),
                    )
                    .child(
                        div()
                            .text_size(px(18.))
                            .pl_2()
                            .font_weight(FontWeight::NORMAL)
                            .text_color(rgb(AMBER_600))
                            .child("Settings"),
                    )
                    .on_click(ctx.listener(|_, _, _, _| {
                        println!("settings clicked");
                    })),
            )
    }
}
