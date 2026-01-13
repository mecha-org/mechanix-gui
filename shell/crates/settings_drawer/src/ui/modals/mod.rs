use commons::widgets::wing;
use gpui::*;
use theme::prelude::{AlphaExt, Theme};

pub mod bluetooth_modal;
pub mod display_modal;
pub mod extended_screen;
pub mod performance_modal;
pub mod sound_modal;
pub mod wireless_modal;

use crate::ui::{
    FINAL_MODAL_SIZE, SettingsDrawer,
    icon::{Icon, IconName},
};

pub const ROW_HEIGHT: f32 = 60.0;
pub const MODAL_HEADER_HEIGHT: f32 = 60.0;

impl SettingsDrawer {
    pub fn render_header_div(
        &self,
        cx: &mut gpui::Context<SettingsDrawer>,
        title: &'static str,
    ) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .relative()
            .w_full()
            .h(px(MODAL_HEADER_HEIGHT))
            .justify_center()
            .items_center()
            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                px(16.)
            } else {
                px(20.)
            })
            .child({
                let mut w = wing()
                    .absolute()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h(px(56.0))
                    .px_4()
                    .py_2()
                    .border_color(colors.accent_200.with_alpha(0.4))
                    .border_b_0()
                    .bg(colors.background_1000)
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(colors.foreground_300)
                            .child(title),
                    );
                w.upper_wing_size(Size::new(px(239.0), px(33.0)));
                w.border_width(px(1.0));
                w.border_radius(px(8.0));
                w
            })
            .into_any()
    }

    pub fn render_settings_div(&self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        div()
            .id("id_settings")
            .flex()
            .flex_row()
            .items_end()
            .justify_start()
            .border_t_1()
            .bg(colors.background_1000)
            .border_color(colors.background_800)
            .h(px(ROW_HEIGHT))
            .rounded_md()
            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                px(16.)
            } else {
                px(20.)
            })
            .p_4()
            .flex_shrink_0()
            .hover(|style| style.bg(colors.accent_200.with_alpha(0.1)))
            .child(
                Icon::new(IconName::Settings)
                    .size((px(28.), px(28.)))
                    .text_color(colors.accent_300),
            )
            .child(
                div()
                    .pl_2()
                    .font_weight(FontWeight::NORMAL)
                    .text_color(colors.accent_300)
                    .child("Settings"),
            )
            .on_click(cx.listener(|_, _, _, _| {
                println!("call settings...");
            }))
            .into_any()
    }

    pub fn get_icon_and_text_color(
        is_active: bool,
        cx: &mut gpui::Context<SettingsDrawer>,
    ) -> (Rgba, Rgba) {
        let colors = Theme::global(cx).colors.clone();

        let icon_color = if is_active {
            colors.accent_200
        } else {
            colors.foreground_900
        };

        let text_color = if is_active {
            colors.accent_200
        } else {
            colors.foreground_300
        };
        (icon_color, text_color)
    }
}
