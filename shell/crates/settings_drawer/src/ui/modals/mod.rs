use gpui::*;
use icons::prelude::*;
use theme::prelude::{AlphaExt, Fonts, Theme};

pub mod bluetooth_modal;
pub mod display_modal;
pub mod extended_screen;
pub mod performance_modal;
pub mod sound_modal;
pub mod wireless_modal;

use crate::ui::{FINAL_MODAL_SIZE, SettingsDrawer};

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
            .child(title)
            .flex()
            .w_full()
            .justify_start()
            .text_color(colors.foreground_200)
            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                px(18.)
            } else {
                px(24.)
            })
            .font_weight(FontWeight::SEMIBOLD)
            .pl(px(16.))
            .pt(px(8.))
            .border_b_1()
            .border_color(colors.accent_200.with_alpha(0.4))
            .h(px(MODAL_HEADER_HEIGHT))
            .into_any()
    }

    pub fn render_settings_div(&self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();
        let settings = Icons::global(cx).settings_drawer.clone().settings;

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
                svg()
                    .external_path(SharedString::from(settings.to_string_lossy().to_string()))
                    .size(px(28.))
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
