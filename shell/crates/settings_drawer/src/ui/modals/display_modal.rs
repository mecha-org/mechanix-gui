use commons::widgets::wing;
use gpui::*;
use icons::prelude::SettingsDrawerIcons;
use theme::prelude::{AlphaExt, Theme};

use crate::ui::FINAL_MODAL_SIZE;
use crate::ui::modals::MODAL_HEADER_HEIGHT;
use crate::{
    prelude::*,
    ui::widgets::{Switch, SwitchSize},
};
use icons::prelude::*;

impl SettingsDrawer {
    pub fn render_display_modal(&self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();
        let SettingsDrawerIcons {
            auto_brightness,
            dark_mode,
            ..
        } = Icons::global(cx).settings_drawer.clone();

        let (auto_brightness_icon_color, auto_brightness_text_color) =
            Self::get_icon_and_text_color(self.auto_brightness, cx);
        let (dark_mode_icon_color, dark_mode_text_color) =
            Self::get_icon_and_text_color(self.dark_mode, cx);

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
                    .child(
                        div()
                            .child("Display brightness")
                            .flex()
                            .w_full()
                            .justify_start()
                            .text_color(colors.foreground_400)
                            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                                px(18.)
                            } else {
                                px(24.)
                            })
                            .pl(px(16.))
                            .pt(px(8.))
                            .border_b_1()
                            .border_color(colors.accent_200.with_alpha(0.4))
                            .h(px(MODAL_HEADER_HEIGHT)),
                    )
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
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .text_align(TextAlign::Left)
                                            .child(
                                                svg()
                                                    .external_path(SharedString::from(
                                                        auto_brightness
                                                            .to_string_lossy()
                                                            .to_string(),
                                                    ))
                                                    .size(px(28.))
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
                                                svg()
                                                    .external_path(SharedString::from(
                                                        dark_mode.to_string_lossy().to_string(),
                                                    ))
                                                    .size(px(28.))
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
                    .child(self.render_settings_div(cx));
                w.upper_wing_size(Size::new(px(237.0), px(36.0)));
                w.border_width(px(1.0));
                w.border_radius(px(8.0));
                w
            })
            .into_any()
    }
}
