use gpui::*;
use theme::prelude::{AlphaExt, Theme};

use crate::{
    prelude::*,
    ui::icon::{Icon, IconName},
};

const ROW_HEIGHT: f32 = 60.0;

#[derive(Debug)]
struct PerformanceMode {
    performance_mode: String,
    text: String,
    is_active: bool,
}

impl SettingsDrawer {
    pub fn render_battery_performance_modal(
        &self,
        cx: &mut gpui::Context<SettingsDrawer>,
    ) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

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
            .bg(colors.background_1000)
            .size_full()
            .border_1()
            .rounded_xl()
            .border_color(colors.accent_200.with_alpha(0.4))
            .child(self.render_header_div(cx, "Battery"))
            .child(div().flex().flex_col().flex_1().relative().children(
                performance_modes.iter().enumerate().map(|(idx, mode)| {
                    let is_active = mode.is_active;

                    let (icon_color, text_color) =
                        Self::get_icon_and_text_color(is_active, cx);

                    let mut icon = IconName::PowerModeBalanced;

                    if mode.performance_mode == "high" {
                        icon = IconName::HighPower;
                    } else if mode.performance_mode == "saver" {
                        icon = IconName::SavingPower;
                    };

                    let connect_div = div().child(
                        Icon::new(IconName::ConnectedIcon)
                            .size((px(24.), px(24.)))
                            .text_color(icon_color),
                    );

                    let main_div = if is_active {
                        div()
                            .id(("mode", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(ROW_HEIGHT))
                            .px_4()
                            .bg(colors.accent_200.with_alpha(0.1))
                            .border_y_1()
                            .border_color(colors.accent_200.with_alpha(0.4))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .text_align(TextAlign::Left)
                                    .child(
                                        div().pr_2().child(
                                            Icon::new(icon)
                                                .size((px(28.), px(28.)))
                                                .text_color(icon_color),
                                        ),
                                    )
                                    .child(
                                        div()
                                            .pl_2()
                                            .font_weight(FontWeight::NORMAL)
                                            .text_color(text_color)
                                            .text_size(px(18.))
                                            .child(mode.text.clone()),
                                    ),
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
                            .hover(|style| style.bg(colors.accent_200.with_alpha(0.1)))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .text_align(TextAlign::Left)
                                    .child(
                                        div().pr_2().child(
                                            Icon::new(icon)
                                                .size((px(28.), px(28.)))
                                                .text_color(icon_color),
                                        ),
                                    )
                                    .child(
                                        div()
                                            .pl_2()
                                            .font_weight(FontWeight::NORMAL)
                                            .text_color(text_color)
                                            .text_size(px(18.))
                                            .child(mode.text.clone()),
                                    ),
                            )
                            .on_click(cx.listener(move |_, _, _, _| {
                                println!("mode clicked...");
                            }))
                    };

                    main_div
                }),
            ))
            // Footer
            .child(self.render_settings_div(cx))
            .into_any()
    }
}
