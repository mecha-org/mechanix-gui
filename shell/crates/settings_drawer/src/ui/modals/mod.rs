use gpui::*;
use theme::prelude::{AlphaExt, Theme};

pub mod bluetooth_modal;
pub mod display_modal;
pub mod extended_screen;
pub mod performance_modal;
pub mod sound_modal;
pub mod wireless_modal;

use crate::ui::{
    SettingsDrawer,
    icon::{Icon, IconName},
};
pub use bluetooth_modal::BluetoothModalScroll;
pub use wireless_modal::WirelessModalScroll;

const ROW_HEIGHT: f32 = 60.0;

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
            .items_center()
            .justify_between()
            .w_full()
            .p_4()
            .h(px(ROW_HEIGHT))
            .border_b_1()
            .bg(colors.background_1000)
            .flex_shrink_0()
            .relative()
            .child(
                div()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(colors.foreground_300)
                    .child(title),
            )
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
            .border_color(colors.accent_200.with_alpha(0.4))
            .h(px(ROW_HEIGHT))
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
