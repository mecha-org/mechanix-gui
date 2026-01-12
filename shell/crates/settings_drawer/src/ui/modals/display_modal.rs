use gpui::*;
use theme::prelude::{AlphaExt, Theme};

use crate::ui::FINAL_MODAL_SIZE;
use crate::{
    prelude::*,
    ui::{
        icon::{Icon, IconName},
        widgets::{Switch, SwitchSize},
    },
};
// TODO: when auto brightness is enabled, hightlight the row

impl SettingsDrawer {
    pub fn render_display_modal(&self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        let (auto_brightness_icon_color, auto_brightness_text_color) = Self::get_icon_and_text_color(self.auto_brightness, cx);
        let (dark_mode_icon_color, dark_mode_text_color) = Self::get_icon_and_text_color(self.dark_mode, cx);

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.render_header_div(cx, "Display brightness"))
            .child(
                div()
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
                    .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                        px(16.)
                    } else {
                        px(18.)
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .relative()
                            .w_full()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_start()
                                    .items_start()
                                    .pl_2()
                                    .py_6()
                                    .child(self.render_brightness_slider(cx, 391.)),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .p_4()
                                    .flex_shrink_0()
                                    .border_y_1()
                                    .border_color(colors.accent_200.with_alpha(0.4))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .text_align(TextAlign::Left)
                                            .child(
                                                Icon::new(IconName::AutoBrightness)
                                                    .size((px(28.), px(28.)))
                                                    .text_color(auto_brightness_icon_color),
                                            )
                                            .child(
                                                div()
                                                    .pl_2()
                                                    .font_weight(FontWeight::NORMAL)
                                                    .text_color(auto_brightness_text_color)
                                                    .child("Auto brightness"),
                                            ),
                                    )
                                    .child(
                                        div().child(
                                            Switch::new("auto_brightness_switch")
                                                .checked(self.auto_brightness)
                                                .size(SwitchSize::Medium)
                                                .on_click(cx.listener(
                                                    move |view, checked, _, cx| {
                                                        view.auto_brightness = *checked;
                                                        cx.notify();
                                                    },
                                                )),
                                        ),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .p_4()
                                    .flex_shrink_0()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .text_align(TextAlign::Left)
                                            .child(
                                                Icon::new(IconName::DarkMode)
                                                    .size((px(28.), px(28.)))
                                                    .text_color(dark_mode_icon_color),
                                            )
                                            .child(
                                                div()
                                                    .pl_2()
                                                    .font_weight(FontWeight::NORMAL)
                                                    .text_color(dark_mode_text_color)
                                                    .child("Dark mode"),
                                            ),
                                    )
                                    .child(
                                        div().child(
                                            Switch::new("dark_mode_switch")
                                                .checked(self.dark_mode)
                                                .size(SwitchSize::Medium)
                                                .on_click(cx.listener(
                                                    move |view, checked, _, cx| {
                                                        view.dark_mode = *checked;
                                                        cx.notify();
                                                    },
                                                )),
                                        ),
                                    ),
                            ),
                    ) 
                    .child(self.render_settings_div(cx)),
            )
            .into_any()
    }
}
