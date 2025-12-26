use gpui::*;

use crate::{
    prelude::*,
    ui::icon::{Icon, IconName},
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
        // extended screen
        let extend_options = vec![
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
        ];
        extend_options
    }

    pub fn render_extended_screen_options(
        &self,
        cx: &mut gpui::Context<SettingsDrawer>,
    ) -> AnyElement {
        let extend_options = self.extended_screens();
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(rgb(DARK_NEUTRAL_900))
            .rounded_xl()
            .border_1()
            .border_color(rgb(AMBER_800))
            .child(self.render_header_div(cx, "Extended screen"))
            .child(div().flex().flex_col().flex_1().relative().children(
                extend_options.iter().enumerate().map(|(idx, ex)| {
                    let is_active = ex.is_active;

                    let icon_color = if is_active {
                        rgb(AMBER_600)
                    } else {
                        rgb(DARK_NEUTRAL_0)
                    };

                    let icon = match ex.extend_type {
                        ExtendedType::ExtendedDetected => IconName::ExtendedDetected,
                        ExtendedType::MirrorScreen => IconName::MirrorScreen,
                        ExtendedType::ExtendedOnly => IconName::ExtendedOnly,
                        ExtendedType::SecondScreen => IconName::SecondScreen,
                    };

                    let connect_div = div().child(
                        Icon::new(IconName::ConnectedIcon)
                            .size((px(24.), px(24.)))
                            .text_color(rgb(AMBER_600)),
                    );

                    let main_div = if is_active {
                        div()
                            .id(("sink", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(60.))
                            .px_4()
                            .bg(rgba(AMBER_600_10))
                            .border_y_1()
                            .border_color(rgb(AMBER_900))
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
                                            .text_color(icon_color)
                                            .child(ex.text.clone()),
                                    ),
                            )
                            .child(connect_div)
                    } else {
                        div()
                            .id(("mode", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(60.))
                            .px_4()
                            .hover(|style| style.bg(rgba(AMBER_600_10)))
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
                                            .text_color(icon_color)
                                            .child(ex.text.clone()),
                                    ),
                            )
                            .on_click(cx.listener(move |_, _, _, _| {
                                println!("option clicked...");
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
