use std::char::ToLowercase;

use futures::SinkExt;
use gpui::*;
use shell_state::{ShellState, VolumeMessage};
use theme::prelude::{AlphaExt, Theme};

use crate::ui::FINAL_MODAL_SIZE;
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
        let mut sound_list = ShellState::global(cx).sound_devices.clone();
        let default_sound_device = ShellState::global(cx).default_sound_device.clone();
        let volume_tx = ShellState::global(cx).volume_tx.clone().unwrap();

        let default_description = default_sound_device.description.clone();
        sound_list.sort_by_key(|device| {
            device.description != default_description
        });

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.render_header_div(cx, "Sound"))
            .child(
                div()
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
                    .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                        px(16.)
                    } else {
                        px(18.)
                    })
                    .child(div().flex().flex_col().flex_1().relative().children(
                        sound_list.iter().enumerate().map(|(idx, device)| {
                            let device_name =
                                device.name.clone().unwrap_or_else(|| "Unknown".to_string());
                            let device_desc = device
                                .description
                                .clone()
                                .unwrap_or_else(|| "Unknown".to_string());

                            let default_device_name = default_sound_device
                                .name
                                .clone()
                                .unwrap_or_else(|| "Unknown".to_string());

                            let is_active = device_name == default_device_name;

                            let (icon_color, text_color) =
                                Self::get_icon_and_text_color(is_active, cx);

                            let icon = if device_desc.to_lowercase().contains("built-in") {
                                IconName::SystemSpeaker
                            } else {
                                IconName::ExternalSpeaker
                            };

                            let connect_div = div().child(
                                Icon::new(IconName::Connected)
                                    .size((px(24.), px(24.)))
                                    .text_color(icon_color),
                            );
                            let main_div = if is_active {
                                div()
                                    .id(("device", idx))
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
                                                    .child(device_desc.clone()),
                                            ),
                                    )
                                    .child(if is_active { connect_div } else { div() })
                            } else {
                                 let mut volume_tx = volume_tx.clone();
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
                                                    .child(device_desc.clone()),
                                            ),
                                    )
                                    .on_click(cx.listener(
                                            move |this: &mut SettingsDrawer,
                                                          _event: &ClickEvent,
                                                          _window: &mut Window,
                                                          cx: &mut Context<Self>| {
                                        let mut volume_tx_1 = volume_tx.clone();
                                        let device_name = device_name.clone();

                                        cx.background_executor()
                                            .spawn(async move {
                                                let _ = volume_tx_1.send(VolumeMessage::SetDefaultOutputSoundDevice { name: device_name }).await;

                                            })
                                            .detach();
                                          Self::start_close_animation(this, cx);

                                        cx.notify();
                                    }))
                            };
                            main_div
                        }),
                    ))
                    .child(self.render_settings_div(cx)),
            )
            .into_any()
    }
}
