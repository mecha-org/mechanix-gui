use futures::SinkExt;
use gpui::*;
use shell_state::{NmMessage, ShellState};
use theme::prelude::{AlphaExt, Theme};

use crate::{
    helper::get_wireless_strength_icon,
    prelude::*,
    ui::icon::{Icon, IconName},
};

const HEADER_HEIGHT: f32 = 60.0;
const FOOTER_HEIGHT: f32 = 60.0;
const ROW_HEIGHT: f32 = 60.0;

pub trait ScrollBehavior {
    fn scroll_offset(&self) -> Pixels;
    fn set_scroll_offset(&mut self, offset: Pixels);
    fn is_dragging(&self) -> bool;

    fn calculate_scroll_bounds(
        &self,
        content_height: Pixels,
        container_height: Pixels,
    ) -> (Pixels, Pixels) {
        let max_scroll = px(0.);
        let min_scroll = container_height - content_height;

        if content_height <= container_height {
            (px(0.), px(0.))
        } else {
            (min_scroll, max_scroll)
        }
    }

    fn estimate_content_height(&self, item_count: usize) -> Pixels;

    fn on_mouse_down(&mut self, event: &MouseDownEvent);
    fn on_mouse_up(&mut self);
    fn on_mouse_move(&mut self, event: &MouseMoveEvent, window_height: Pixels, item_count: usize);
}

#[derive(Debug, Clone)]
pub struct WirelessModalScroll {
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    drag_start_y: Pixels,
    is_dragging: bool,
}

impl Default for WirelessModalScroll {
    fn default() -> Self {
        Self {
            scroll_offset: px(0.),
            last_scroll_offset: px(0.),
            drag_start_y: px(0.),
            is_dragging: false,
        }
    }
}

impl WirelessModalScroll {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ScrollBehavior for WirelessModalScroll {
    fn scroll_offset(&self) -> Pixels {
        self.scroll_offset
    }

    fn set_scroll_offset(&mut self, offset: Pixels) {
        self.scroll_offset = offset;
    }

    fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    fn estimate_content_height(&self, list_count: usize) -> Pixels {
        let item_height = px(ROW_HEIGHT);
        let gap = px(8.);
        let item_count = list_count as f32;

        if item_count == 0.0 {
            px(0.)
        } else {
            item_count * item_height + (item_count - 1.0) * gap
        }
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent) {
        self.drag_start_y = event.position.y;
        self.last_scroll_offset = self.scroll_offset;
        self.is_dragging = true;
    }

    fn on_mouse_up(&mut self) {
        self.is_dragging = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, window_height: Pixels, list_count: usize) {
        if self.is_dragging {
            let delta_y = event.position.y - self.drag_start_y;

            let container_height = window_height - px(HEADER_HEIGHT) - px(FOOTER_HEIGHT);
            let content_height = self.estimate_content_height(list_count);

            let (min_scroll, max_scroll) =
                self.calculate_scroll_bounds(content_height, container_height);

            self.scroll_offset = (self.last_scroll_offset + delta_y).clamp(min_scroll, max_scroll);
        }
    }
}

impl SettingsDrawer {
    pub fn render_wireless_modal(&mut self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        let wireless_details = ShellState::global(cx).wireless_details.clone();
        let nm_tx = ShellState::global(cx).nm_tx.clone().unwrap();
        let network_list = wireless_details.networks.clone().unwrap_or_default();
        let network_count = network_list.len();

        div()
            .flex()
            .flex_col()
            .bg(colors.background_1000)
            .size_full()
            .border_1()
            .rounded_xl()
            .border_color(colors.accent_200.with_alpha(0.4))
            .child(
                self.render_header_div(cx, "Wireless")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .relative()
                    .overflow_hidden()
                    .bg(colors.background_1000)
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                        cx.stop_propagation();
                        this.wireless_modal_scroll.on_mouse_down(event);
                        cx.notify();
                    }))
                    .on_mouse_up(MouseButton::Left, cx.listener(move |this, _event: &MouseUpEvent, _window, cx| {
                        this.wireless_modal_scroll.on_mouse_up();
                        cx.notify();
                    }))
                    .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, window, cx| {
                        let window_height = window.bounds().size.height;
                        this.wireless_modal_scroll.on_mouse_move(event, window_height, network_count);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .absolute()
                            .top(self.wireless_modal_scroll.scroll_offset())
                            .left(px(0.))
                            .right(px(0.))
                            .flex()
                            .flex_col()
                            .gap_2()
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
                    ),
            )
            .child(
                self.render_settings_div(cx)
            )
            .into_any()
    }
}
