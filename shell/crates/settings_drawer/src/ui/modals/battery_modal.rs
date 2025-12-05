use futures::{SinkExt, channel::mpsc};
use gpui::*;

use crate::ui::{
    icon::{Icon, IconName},
    widgets::IconButton,
};

#[derive(Debug)]
struct PerformanceMode {
    performance_mode: String,
    text: String,
    is_active: bool,
}

pub struct BatteryWindow {
    pub title: String,
}

impl BatteryWindow {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl Render for BatteryWindow {
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
            .p_4()
            .child(
                // Header
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .h(px(60.))
                    .child(
                        div()
                            .text_size(px(20.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xE1E1E1))
                            .child(self.title.clone()),
                    )
                    .child(
                        IconButton::new("id_settings")
                            .icon(IconName::Settings)
                            .icon_color(rgb(0xF4F4F4))
                            .size((px(36.), px(36.)))
                            .border(px(0.))
                            .on_click(ctx.listener(|_, _, _, _| {
                                println!("settings clicked");
                            })),
                    ),
            )
            .child(
                // Content with scroll offset
                div().flex().flex_col().gap_2().relative().children(
                    performance_modes.iter().enumerate().map(|(idx, mode)| {
                        let mut icon_color = rgb(0xC67600);
                        let mut icon = IconName::PowerModeBalanced;

                        if mode.performance_mode == "high" {
                            icon_color = rgb(0x8570FF);
                            icon = IconName::HighPower;
                        } else if mode.performance_mode == "saver" {
                            icon_color = rgb(0xC67600);
                            icon = IconName::SavingPower;
                        };

                        let mut main_div = div()
                            .id(("mode", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(60.))
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
                            .child(
                                div()
                                    .text_color(rgb(0x8F8F8F))
                                    .text_base()
                                    .child(if mode.is_active { "Connected" } else { "" }),
                            );

                        main_div
                    }),
                ),
            )
    }
}
