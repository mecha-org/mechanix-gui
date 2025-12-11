use futures::{SinkExt, channel::mpsc};
use gpui::*;

use crate::{
    prelude::*,
    ui::{
        icon::{Icon, IconName},
        widgets::IconButton,
    },
};

const ROW_HEIGHT: f32 = 60.0;

#[derive(Debug, Clone, PartialEq, Default)]
enum ExtendedType {
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

pub struct ExtendScreenOptions {
    pub title: String,
}

impl ExtendScreenOptions {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

impl Render for ExtendScreenOptions {
    fn render(&mut self, _window: &mut Window, ctx: &mut Context<Self>) -> impl IntoElement {
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
            .child(div().flex().flex_col().flex_1().relative().children(
                extend_options.iter().enumerate().map(|(idx, ex)| {
                    let is_active = ex.is_active;

                    let icon_color = if is_active {
                        rgb(AMBER_600)
                    } else {
                        rgb(DARK_NEUTRAL_0)
                    };
                    let mut icon = IconName::ExtendedDetected;

                    match ex.extend_type {
                        ExtendedType::ExtendedDetected => icon = IconName::ExtendedDetected,
                        ExtendedType::MirrorScreen => icon = IconName::MirrorScreen,
                        ExtendedType::ExtendedOnly => icon = IconName::ExtendedOnly,
                        ExtendedType::SecondScreen => icon = IconName::SecondScreen,
                    }

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
                            .bg(if is_active {
                                rgba(AMBER_600_10)
                            } else {
                                rgba(AMBER_900)
                            })
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
                                            .text_lg()
                                            .pl_2()
                                            .font_weight(FontWeight::NORMAL)
                                            .text_color(icon_color)
                                            .child(ex.text.clone()),
                                    ),
                            )
                            .child(if ex.is_active { connect_div } else { div() })
                    } else {
                        div()
                            .id(("mode", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(60.))
                            .px_4()
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
                                            .text_lg()
                                            .pl_2()
                                            .font_weight(FontWeight::NORMAL)
                                            .text_color(icon_color)
                                            .child(ex.text.clone()),
                                    ),
                            )
                            .on_click(ctx.listener(move |_, _, _, _| {
                                println!("option clicked...");
                            }))
                    };

                    main_div
                }),
            ))
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
