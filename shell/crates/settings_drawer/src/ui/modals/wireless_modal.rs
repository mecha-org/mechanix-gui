use commons::widgets::wing;
use futures::SinkExt;
use gpui::*;
use shell_state::{NmMessage, ShellState};
use theme::prelude::{AlphaExt, Theme};

use crate::{
    helper::get_wireless_strength_icon,
    prelude::*,
    ui::{
        FINAL_MODAL_SIZE,
        icon::{Icon, IconName},
        modals::{MODAL_HEADER_HEIGHT, ROW_HEIGHT},
    },
};

impl SettingsDrawer {
    pub fn render_wireless_modal(&mut self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        let wireless_details = ShellState::global(cx).wireless_details.clone();
        let nm_tx = ShellState::global(cx).nm_tx.clone().unwrap();
        let network_list = wireless_details
            .networks
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|n| !n.ssid.trim().is_empty())
            .collect::<Vec<_>>();

        div()
            .flex()
            .flex_col()
            .size_full()
            .child({
                let mut w = wing()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .bg(colors.background_1000)
                    .border_color(colors.accent_200.with_alpha(0.4))
                    .border_1()
                    .border_t_0()
                    .rounded(px(8.))
                    .overflow_hidden()
                    .child(
                        div()
                            .child("Wireless")
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
                            .border_b_1()
                            .border_color(colors.accent_200.with_alpha(0.4))
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
                            .border_b_1()
                            .border_color(colors.accent_200.with_alpha(0.4))
                            .children(network_list.iter().enumerate().map(
                                |(idx, network)| {
                                    let ssid = network.ssid.clone();
                                    let is_active = network.is_active;
                                    let is_known = network.is_known;

                                    let (icon_color, text_color) = Self::get_icon_and_text_color(is_active, cx);

                                    let wireless_icon = get_wireless_strength_icon(
                                        network.is_active,
                                        network.signal_strength,
                                        network.security.clone(),
                                    );

                                    let connect_div = div().child(
                                        Icon::new(IconName::Connected)
                                            .size((px(24.), px(24.)))
                                            .text_color(text_color),
                                    );

                                    let mut network_div = if is_active {
                                        div()
                                            .id(("network_item", idx))
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
                                                            Icon::new(wireless_icon)
                                                                .size((px(28.), px(28.)))
                                                                .text_color(icon_color),
                                                        ),
                                                    )
                                                    .child(
                                                        div()
                                                            .pl_2()
                                                            .font_weight(FontWeight::NORMAL)
                                                            .text_color(text_color)
                                                            .child(network.ssid.clone())
                                                    ),
                                            )
                                            .child(connect_div)
                                    } else {
                                        div()
                                            .id(("network_item", idx))
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
                                                            Icon::new(wireless_icon)
                                                                .size((px(28.), px(28.)))
                                                                .text_color(icon_color),
                                                        ),
                                                    )
                                                    .child(
                                                        div()
                                                            .pl_2()
                                                            .font_weight(FontWeight::NORMAL)
                                                            .text_color(text_color)
                                                            .child(network.ssid.clone())
                                                    ),
                                            )
                                    };

                                    if !is_active && !is_known {
                                        network_div = network_div.on_click(cx.listener(
                                            move |_, _, _, _| {
                                                println!(
                                                    "TODO: open settings for new network {:?} - {:?}",
                                                    ssid, is_known
                                                );
                                            },
                                        ));
                                    } else if !is_active && is_known {
                                        let ssid_clone = ssid.clone();
                                        let nm_tx_clone = nm_tx.clone();

                                        network_div = network_div.on_click(cx.listener(
                                            move |this: &mut SettingsDrawer,
                                                  _event: &ClickEvent,
                                                  _window: &mut Window,
                                                  cx: &mut Context<Self>| {
                                                let ssid = ssid_clone.clone();
                                                let mut nm_tx = nm_tx_clone.clone();

                                                cx.background_executor()
                                                    .spawn(async move {
                                                        let _ = nm_tx
                                                            .send(NmMessage::ConnectKnownNetwork { name: ssid })
                                                            .await;
                                                    })
                                                    .detach();
                                                Self::start_close_animation(this, cx);

                                                cx.notify();
                                            },
                                        ));
                                    }
                                    network_div
                                },
                            )),
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
