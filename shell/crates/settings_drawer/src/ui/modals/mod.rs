use gpui::*;

pub mod bluetooth_modal;
pub mod display_modal;
pub mod extended_screen;
pub mod performance_modal;
pub mod sound_modal;
pub mod wireless_modal;

use crate::{
    prelude::{AMBER_600, AMBER_600_10, DARK_NEUTRAL_0, DARK_NEUTRAL_700, DARK_NEUTRAL_800},
    ui::{
        SettingsDrawer,
        icon::{Icon, IconName},
    },
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
            .relative()
            .child(
                div()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(DARK_NEUTRAL_0))
                    .child(title),
            )
            .into_any()
    }
    pub fn render_settings_div(&self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
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
            .hover(|style| style.bg(rgba(AMBER_600_10)))
            .child(
                Icon::new(IconName::Settings)
                    .size((px(28.), px(28.)))
                    .text_color(rgb(AMBER_600)),
            )
            .child(
                div()
                    .pl_2()
                    .font_weight(FontWeight::NORMAL)
                    .text_color(rgb(AMBER_600))
                    .child("Settings"),
            )
            .on_click(cx.listener(|_, _, _, _| {
                println!("call settings...");
            }))
            .into_any()
    }
}
