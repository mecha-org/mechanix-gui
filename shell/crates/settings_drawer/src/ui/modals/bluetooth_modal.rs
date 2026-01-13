use crate::{
    helper::get_bluetooth_icon,
    prelude::*,
    ui::{
        FINAL_MODAL_SIZE,
        icon::{Icon, IconName},
        modals::{MODAL_HEADER_HEIGHT, ROW_HEIGHT},
    },
};
use commons::widgets::wing;
use futures::SinkExt;
use gpui::*;
use shell_state::{BtMessage, ShellState};
use theme::prelude::{AlphaExt, Theme};

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
pub struct BluetoothModalScroll {
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    drag_start_y: Pixels,
    is_dragging: bool,
}

impl Default for BluetoothModalScroll {
    fn default() -> Self {
        Self {
            scroll_offset: px(0.),
            last_scroll_offset: px(0.),
            drag_start_y: px(0.),
            is_dragging: false,
        }
    }
}

impl BluetoothModalScroll {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ScrollBehavior for BluetoothModalScroll {
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
        px(list_count as f32 * ROW_HEIGHT) + px(16.)
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

            let container_height = window_height - px(MODAL_HEADER_HEIGHT) - px(ROW_HEIGHT);
            let content_height = self.estimate_content_height(list_count);

            let (min_scroll, max_scroll) =
                self.calculate_scroll_bounds(content_height, container_height);

            self.scroll_offset = (self.last_scroll_offset + delta_y).clamp(min_scroll, max_scroll);
        }
    }
}

impl SettingsDrawer {
    pub fn render_bluetooth_modal(&mut self, cx: &mut gpui::Context<SettingsDrawer>) -> AnyElement {
        let colors = Theme::global(cx).colors.clone();

        let bluetooth_details = ShellState::global(cx).bluetooth_details.clone();
        let bt_tx = ShellState::global(cx).bt_tx.clone().unwrap();
        let mut device_list = bluetooth_details
            .available_devices
            .clone()
            .unwrap_or_default();
        device_list.sort_by_key(|d| !d.connected);

        let list_count = device_list.len();

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
                            .child("Bluetooth")
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
                            .overflow_hidden()
                            .text_size(if self.modal_size != FINAL_MODAL_SIZE {
                                px(16.)
                            } else {
                                px(18.)
                            })
                            .border_b_1()
                            .border_color(colors.accent_200.with_alpha(0.4))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                                    cx.stop_propagation();
                                    this.bluetooth_modal_scroll.on_mouse_down(event);
                                    cx.notify();
                                }),
                            )
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _event: &MouseUpEvent, _window, cx| {
                                    this.bluetooth_modal_scroll.on_mouse_up();
                                    cx.notify();
                                }),
                            )
                            .on_mouse_move(cx.listener(
                                move |this, event: &MouseMoveEvent, window, cx| {
                                    let window_height = window.bounds().size.height;
                                    this.bluetooth_modal_scroll.on_mouse_move(
                                        event,
                                        window_height,
                                        list_count,
                                    );
                                    cx.notify();
                                },
                            ))
                            .child(
                                div()
                                    .absolute()
                                    .top(self.bluetooth_modal_scroll.scroll_offset())
                                    .left(px(0.))
                                    .right(px(0.))
                                    .flex()
                                    .flex_col()
                                    .children(device_list.iter().enumerate().map(|(idx, bt)| {
                                        let name = &bt.name;
                                        let is_connected = bt.connected;
                                        let is_paired = bt.paired;

                                        let (icon_color, text_color) =
                                            Self::get_icon_and_text_color(is_connected, cx);

                                        let bluetooth_icon = get_bluetooth_icon(is_connected);

                                        let connect_div = div().child(
                                            Icon::new(IconName::Connected)
                                                .size((px(24.), px(24.)))
                                                .text_color(text_color),
                                        );

                                        let mut bluetooth_div = if is_connected {
                                            div()
                                                .id(("bluetooth_item", idx))
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
                                                                Icon::new(bluetooth_icon)
                                                                    .size((px(28.), px(28.)))
                                                                    .text_color(icon_color),
                                                            ),
                                                        )
                                                        .child(
                                                            div()
                                                                .pl_2()
                                                                .font_weight(FontWeight::NORMAL)
                                                                .text_color(text_color)
                                                                .child(name.clone()),
                                                        ),
                                                )
                                                .child(connect_div)
                                        } else {
                                            div()
                                                .id(("bluetooth_item", idx))
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .h(px(ROW_HEIGHT))
                                                .px_4()
                                                .hover(|style| {
                                                    style.bg(colors.accent_200.with_alpha(0.1))
                                                })
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .text_align(TextAlign::Left)
                                                        .child(
                                                            div().pr_2().child(
                                                                Icon::new(bluetooth_icon)
                                                                    .size((px(28.), px(28.)))
                                                                    .text_color(icon_color),
                                                            ),
                                                        )
                                                        .child(
                                                            div()
                                                                .pl_2()
                                                                .font_weight(FontWeight::NORMAL)
                                                                .text_color(text_color)
                                                                .child(name.clone()),
                                                        ),
                                                )
                                        };

                                        if !is_connected && !is_paired {
                                            bluetooth_div =
                                                bluetooth_div
                                                    .on_click(cx.listener(move |_, _, _, _| {
                                                    println!(
                                                        "TODO: open portal/settings with params:"
                                                    );
                                                }));
                                        } else if !is_connected && is_paired {
                                            let address = bt.address.clone();
                                            let bt_tx_clone = bt_tx.clone();

                                            bluetooth_div = bluetooth_div.on_click(cx.listener(
                                            move |this: &mut SettingsDrawer,
                                                  _event: &ClickEvent,
                                                  _window: &mut Window,
                                                  cx: &mut Context<Self>| {
                                                let address_clone = address.clone();
                                                let mut bt_tx = bt_tx_clone.clone();

                                                cx.background_executor()
                                                    .spawn(async move {
                                                        let _ = bt_tx
                                                            .send(BtMessage::ConnectDevice {
                                                                address: address_clone,
                                                            })
                                                            .await;
                                                    })
                                                    .detach();
                                                Self::start_close_animation(this, cx);
                                            },
                                        ));
                                        }

                                        bluetooth_div
                                    })),
                            ),
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
