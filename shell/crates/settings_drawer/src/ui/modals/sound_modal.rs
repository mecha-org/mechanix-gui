use commons::widgets::wing;
use futures::SinkExt;
use gpui::{prelude::FluentBuilder, *};
use shell_state::{ShellState, VolumeMessage};
use theme::prelude::{AlphaExt, Theme};

use crate::{
    prelude::*,
    ui::{
        FINAL_MODAL_SIZE,
        icon::{Icon, IconName},
        modals::{MODAL_HEADER_HEIGHT, ROW_HEIGHT},
    },
};

impl SettingsDrawer {
    pub fn render_sound_modal(&self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        let mut sound_list = ShellState::global(cx).sound_devices.clone();
        let default_sound_device = ShellState::global(cx).default_sound_device.clone();
        let volume_tx = ShellState::global(cx).volume_tx.clone().unwrap();

        let default_description = default_sound_device.description.clone();
        sound_list.sort_by_key(|device| device.description != default_description);

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
                            .child("Sound")
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
                            .id("scrollable")
                            .flex_1()
                            .relative()
                            .overflow_y_scroll()
                            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                                px(16.)
                            } else {
                                px(18.)
                            })
                            .children(sound_list.iter().enumerate().map(|(idx, device)| {
                                let device_name = device
                                    .name
                                    .clone()
                                    .unwrap_or_else(|| "Unknown".to_string());
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
                                        .text_color(text_color),
                                );

                                let mut device_div = if is_active {
                                    div()
                                        .id(("device_item", idx))
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
                                                        .child(device_desc.clone()),
                                                ),
                                        )
                                        .child(connect_div)
                                } else {
                                    div()
                                        .id(("device_item", idx))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .h(px(ROW_HEIGHT))
                                        .px_4()
                                        .rounded(px(8.))
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
                                };

                                if !is_active {
                                    let device_name_clone = device_name.clone();
                                    let volume_tx_clone = volume_tx.clone();

                                    device_div = device_div.on_click(cx.listener(
                                        move |this: &mut SettingsDrawer,
                                              _event: &ClickEvent,
                                              _window: &mut Window,
                                              cx: &mut Context<Self>| {
                                            let device_name = device_name_clone.clone();
                                            let mut volume_tx = volume_tx_clone.clone();

                                            cx.background_executor()
                                                .spawn(async move {
                                                    let _ = volume_tx
                                                        .send(VolumeMessage::SetDefaultOutputSoundDevice {
                                                            name: device_name,
                                                        })
                                                        .await;
                                                })
                                                .detach();
                                            Self::start_close_animation(this, cx);

                                            cx.notify();
                                        },
                                    ));
                                }

                                device_div
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
