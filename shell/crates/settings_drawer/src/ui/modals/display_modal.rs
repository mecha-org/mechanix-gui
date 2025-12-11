use futures::{SinkExt, channel::mpsc};
use gpui::*;

use crate::{
    prelude::*,
    ui::{
        icon::{Icon, IconName},
        widgets::{Switch, SwitchSize},
    },
};

const ROW_HEIGHT: f32 = 60.0;

pub struct DisplayWindow {
    pub title: String,
    pub auto_brightness: bool,
    pub dark_mode: bool,
}

impl DisplayWindow {
    pub fn new(title: String, auto_brightness: bool, dark_mode: bool) -> Self {
        Self {
            title,
            auto_brightness: false,
            dark_mode: false,
        }
    }
}

impl Render for DisplayWindow {
    fn render(&mut self, _window: &mut Window, ctx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(DARK_NEUTRAL_900))
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
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .relative()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p_4()
                            .flex_shrink_0()
                            .border_y_1()
                            .border_color(rgb(DARK_NEUTRAL_700))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .text_align(TextAlign::Left)
                                    .child(
                                        Icon::new(IconName::AutoBrightness)
                                            .size((px(28.), px(28.)))
                                            .text_color(rgb(AMBER_600)),
                                    )
                                    .child(
                                        div()
                                            .text_lg()
                                            .pl_2()
                                            .font_weight(FontWeight::NORMAL)
                                            .text_color(rgb(AMBER_600))
                                            .child("Auto brightness"),
                                    ),
                            )
                            .child(
                                div().child(
                                    Switch::new("auto_brightness_switch")
                                        .checked(self.auto_brightness)
                                        .size(SwitchSize::Medium)
                                        .on_click(ctx.listener(move |view, checked, _, cx| {
                                            view.auto_brightness = *checked;
                                            cx.notify();
                                        })),
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
                                            .text_color(rgb(DARK_NEUTRAL_0)),
                                    )
                                    .child(
                                        div()
                                            .text_lg()
                                            .pl_2()
                                            .font_weight(FontWeight::NORMAL)
                                            .text_color(rgb(DARK_NEUTRAL_0))
                                            .child("Dark mode"),
                                    ),
                            )
                            .child(
                                div().child(
                                    Switch::new("dark_mode_switch")
                                        .checked(self.dark_mode)
                                        .size(SwitchSize::Medium)
                                        .on_click(ctx.listener(move |view, checked, _, cx| {
                                            view.dark_mode = *checked;
                                            cx.notify();
                                        })),
                                ),
                            ),
                    ),
            )
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
