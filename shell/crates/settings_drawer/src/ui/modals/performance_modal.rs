use commons::widgets::wing;
use gpui::*;
use icons::prelude::*;
use theme::prelude::{AlphaExt, Theme};

use crate::ui::modals::{MODAL_WING_HEIGHT, MODAL_WING_WIDTH};
use crate::{
    prelude::*,
    ui::{
        FINAL_MODAL_SIZE,
        modals::ROW_HEIGHT,
    },
};

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
        let connected = Icons::global(cx).settings_drawer.connected.clone();

        let SettingsDrawerIcons {
            high_performance,
            low_performance,
            power_mode_balanced,
            ..
        } = Icons::global(cx).settings_drawer.clone();

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
            .size_full()
            .child({
                let mut w = wing()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .relative()
                    .bg(colors.background_1000)
                    .border_color(colors.accent_200.with_alpha(0.4))
                    .border_1()
                    .border_t_0()
                    .rounded(px(8.))
                    .overflow_hidden()
                    .child(self.render_header_div(cx, "Battery"))
                    .child(
                        div()
                            .flex_1()
                            .relative()
                            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                                px(18.)
                            } else {
                                px(20.)
                            })
                            .children(performance_modes.iter().enumerate().map(|(idx, mode)| {
                                let is_active = mode.is_active;

                                let (icon_color, text_color) =
                                    Self::get_icon_and_text_color(is_active, cx);

                                let icon = if mode.performance_mode == "high" {
                                    &high_performance
                                } else if mode.performance_mode == "saver" {
                                    &low_performance
                                } else {
                                    &power_mode_balanced
                                };

                                let connect_div = div().child(
                                    svg()
                                        .external_path(SharedString::from(
                                            connected.to_string_lossy().to_string(),
                                        ))
                                        .size(px(26.))
                                        .text_color(text_color),
                                );

                                let mut mode_div = if is_active {
                                    div()
                                        .id(("mode_item", idx))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .h(px(ROW_HEIGHT))
                                        .px_4()
                                        .bg(colors.accent_200.with_alpha(0.1))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .text_align(TextAlign::Left)
                                                .child(
                                                    div().pr_2().child(
                                                        svg()
                                                            .external_path(SharedString::from(
                                                                icon.to_string_lossy().to_string(),
                                                            ))
                                                            .size(px(30.))
                                                            .text_color(icon_color),
                                                    ),
                                                )
                                                .child(
                                                    div()
                                                        .pl_2()
                                                        .font_weight(FontWeight::NORMAL)
                                                        .text_color(text_color)
                                                        .child(mode.text.clone()),
                                                ),
                                        )
                                        .child(connect_div)
                                } else {
                                    div()
                                        .id(("mode_item", idx))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .h(px(ROW_HEIGHT))
                                        .px_4()
                                        .hover(|style| style.bg(colors.accent_200.with_alpha(0.1)))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .text_align(TextAlign::Left)
                                                .child(
                                                    div().pr_2().child(
                                                        svg()
                                                            .external_path(SharedString::from(
                                                                icon.to_string_lossy().to_string(),
                                                            ))
                                                            .size(px(30.))
                                                            .text_color(icon_color),
                                                    ),
                                                )
                                                .child(
                                                    div()
                                                        .pl_2()
                                                        .font_weight(FontWeight::NORMAL)
                                                        .text_color(text_color)
                                                        .child(mode.text.clone()),
                                                ),
                                        )
                                };

                                if !is_active {
                                    mode_div = mode_div.on_click(cx.listener(
                                        move |_this: &mut SettingsDrawer,
                                              _event: &ClickEvent,
                                              _window: &mut Window,
                                              _cx: &mut Context<Self>| {
                                            println!("Implement battery performance mode change");
                                        },
                                    ));
                                }

                                mode_div
                            })),
                    )
                    .child(self.render_settings_div(cx, "/battery".to_string()));
                w.upper_wing_size(Size::new(px(MODAL_WING_WIDTH), px(MODAL_WING_HEIGHT)));
                w.border_width(px(1.0));
                w.border_radius(px(8.0));
                w
            })
            .into_any()
    }
}
