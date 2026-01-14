use commons::widgets::wing;
use gpui::*;
use theme::prelude::{AlphaExt, Theme};

use crate::{
    prelude::*,
    ui::{
        FINAL_MODAL_SIZE,
        icon::{Icon, IconName},
        modals::{MODAL_HEADER_HEIGHT, ROW_HEIGHT},
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ExtendedType {
    #[default]
    ExtendedDetected,
    MirrorScreen,
    ExtendedOnly,
    SecondScreen,
}

#[derive(Debug)]
struct ExtendOption {
    text: String,
    extend_type: ExtendedType,
    is_active: bool,
}

impl SettingsDrawer {
    fn extended_screens(&self) -> Vec<ExtendOption> {
        vec![
            ExtendOption {
                text: "Extended detected".to_string(),
                extend_type: ExtendedType::ExtendedDetected,
                is_active: true,
            },
            ExtendOption {
                text: "Mirror screen".to_string(),
                extend_type: ExtendedType::MirrorScreen,
                is_active: false,
            },
            ExtendOption {
                text: "Extended only".to_string(),
                extend_type: ExtendedType::ExtendedOnly,
                is_active: false,
            },
            ExtendOption {
                text: "Second screen".to_string(),
                extend_type: ExtendedType::SecondScreen,
                is_active: false,
            },
        ]
    }

    pub fn render_extended_screen_options(
        &self,
        cx: &mut gpui::Context<SettingsDrawer>,
    ) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();
        let extend_options = self.extended_screens();

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
                            .child("Extended screen")
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
                            .h(px(MODAL_HEADER_HEIGHT)),
                    )
                    .child(
                        div()
                            .flex_1()
                            .relative()
                            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                                px(16.)
                            } else {
                                px(18.)
                            })
                            .children(extend_options.iter().enumerate().map(|(idx, ex)| {
                                let is_active = ex.is_active;

                                let (icon_color, text_color) =
                                    Self::get_icon_and_text_color(is_active, cx);

                                let icon = match ex.extend_type {
                                    ExtendedType::ExtendedDetected => IconName::ExtendedDetected,
                                    ExtendedType::MirrorScreen => IconName::MirrorScreen,
                                    ExtendedType::ExtendedOnly => IconName::ExtendedOnly,
                                    ExtendedType::SecondScreen => IconName::SecondScreen,
                                };

                                let connect_div = div().child(
                                    Icon::new(IconName::Connected)
                                        .size((px(24.), px(24.)))
                                        .text_color(text_color),
                                );

                                let mut option_div = if is_active {
                                    div()
                                        .id(("option_item", idx))
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
                                                        .child(ex.text.clone()),
                                                ),
                                        )
                                        .child(connect_div)
                                } else {
                                    div()
                                        .id(("option_item", idx))
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
                                                        .child(ex.text.clone()),
                                                ),
                                        )
                                };

                                if !is_active {
                                    option_div = option_div.on_click(cx.listener(
                                        move |_this: &mut SettingsDrawer,
                                              _event: &ClickEvent,
                                              _window: &mut Window,
                                              _cx: &mut Context<Self>| {
                                            println!("implement extended screen option change");
                                        },
                                    ));
                                }

                                option_div
                            })),
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
