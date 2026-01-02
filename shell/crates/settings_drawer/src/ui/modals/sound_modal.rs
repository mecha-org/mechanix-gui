use gpui::*;
use theme::prelude::{AlphaExt, Theme};

use crate::{
    prelude::*,
    ui::icon::{Icon, IconName},
};

#[derive(Debug, Clone, PartialEq, Default)]
enum OutputType {
    #[default]
    SystemSpeaker,
    ExternalSpeaker,
    Headphone,
}

#[derive(Debug)]
struct SinkDevice {
    name: String,
    device_type: OutputType,
    is_active: bool,
}

impl SettingsDrawer {
    pub fn render_sound_modal(&self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        let sink_list = vec![
            SinkDevice {
                name: "System speaker".to_string(),
                device_type: OutputType::SystemSpeaker,
                is_active: true,
            },
            SinkDevice {
                name: "Headphones".to_string(),
                device_type: OutputType::Headphone,
                is_active: false,
            },
            SinkDevice {
                name: "External speaker".to_string(),
                device_type: OutputType::ExternalSpeaker,
                is_active: false,
            },
        ];

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(colors.background_1000)
            .size_full()
            .border_1()
            .rounded_xl()
            .border_color(colors.accent_200.with_alpha(0.4))
            .child(self.render_header_div(cx, "Sound"))
            .child(div().flex().flex_col().flex_1().relative().children(
                sink_list.iter().enumerate().map(|(idx, sink)| {
                    let is_active = sink.is_active;

                    let (icon_color, text_color) = Self::get_icon_and_text_color(is_active, cx);

                    let mut icon = IconName::SystemSpeaker;

                    match sink.device_type {
                        OutputType::SystemSpeaker => icon = IconName::SystemSpeaker,
                        OutputType::ExternalSpeaker => icon = IconName::ExternalSpeaker,
                        OutputType::Headphone => icon = IconName::Headphone,
                    }

                    let connect_div = div().child(
                        Icon::new(IconName::ConnectedIcon)
                            .size((px(24.), px(24.)))
                            .text_color(icon_color),
                    );

                    let main_div = if is_active {
                        div()
                            .id(("sink", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(60.))
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
                                            .child(sink.name.clone()),
                                    ),
                            )
                            .child(if sink.is_active { connect_div } else { div() })
                    } else {
                        div()
                            .id(("mode", idx))
                            .flex()
                            .items_center()
                            .justify_between()
                            .h(px(60.))
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
                                            .child(sink.name.clone()),
                                    ),
                            )
                            .on_click(cx.listener(move |_, _, _, _| {
                                println!("sink device clicked...");
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
