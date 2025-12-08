use bluez::interfaces::device::BluetoothDevice;
use futures::{SinkExt, channel::mpsc};
use gpui::*;

use crate::{
    events::BtEvents, get_bluetooth_icon, ui::{
        icon::{Icon, IconName},
        widgets::IconButton,
    }
};

pub struct BluetoothWindow {
    pub title: String,
    pub device_list: Vec<BluetoothDevice>,
    pub bt_tx: mpsc::Sender<BtEvents>,

    // Scrolling state
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    drag_start_y: Pixels,
    is_dragging: bool,
}

impl BluetoothWindow {
    pub fn new(
        title: String,
        device_list: Option<Vec<BluetoothDevice>>,
        bt_tx: mpsc::Sender<BtEvents>,
    ) -> Self {
        Self {
            title,
            device_list: device_list.unwrap_or_default(),
            bt_tx,
            scroll_offset: px(0.),
            last_scroll_offset: px(0.),
            drag_start_y: px(0.),
            is_dragging: false,
        }
    }

    fn calculate_scroll_bounds(
        &self,
        content_height: Pixels,
        window_height: Pixels,
    ) -> (Pixels, Pixels) {
        let header_height = px(60.);
        let padding = px(32.); 
        let container_height = window_height - header_height - padding;

        let max_scroll = px(0.);
        let min_scroll = container_height - content_height;

        if content_height <= container_height {
            (px(0.), px(0.))
        } else {
            (min_scroll, max_scroll)
        }
    }

    fn estimate_content_height(&self) -> Pixels {
        let item_height = px(60.);  
        let gap = px(8.); 
        let item_count = self.device_list.len() as f32;

        item_count * item_height + (item_count - 1.0).max(0.0) * gap
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.drag_start_y = event.position.y;
        self.last_scroll_offset = self.scroll_offset;
        self.is_dragging = true;
        cx.stop_propagation();
    }

    fn on_mouse_up(&mut self, _event: &MouseUpEvent, _: &mut Window, _cx: &mut Context<Self>) {
        self.is_dragging = false;
    }

    fn on_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging {
            let delta_y = event.position.y - self.drag_start_y;
            let window_height = window.bounds().size.height;
            let content_height = self.estimate_content_height();

            let (min_scroll, max_scroll) =
                self.calculate_scroll_bounds(content_height, window_height);

            self.scroll_offset = (self.last_scroll_offset + delta_y).clamp(min_scroll, max_scroll);

            cx.notify();
        }
    }
}

impl Render for BluetoothWindow {
    fn render(&mut self, _window: &mut Window, ctx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x151515))
            .size_full()
            .p_4()
            .child(
                // Header
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .h(px(60.))
                    .child(
                        div()
                            .text_size(px(20.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xE1E1E1))
                            .child(self.title.clone()),
                    )
                    .child(
                        IconButton::new("id_settings")
                            .icon(IconName::Settings)
                            .icon_color(rgb(0xF4F4F4))
                            .size((px(36.), px(36.)))
                            .border(px(0.))
                            .on_click(ctx.listener(|_, _, _, _| {
                                println!("settings clicked");
                            })),
                    ),
            )
            .child(
                // Scrollable container
                div()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .flex_1()
                    .on_mouse_down(MouseButton::Left, ctx.listener(Self::on_mouse_down))
                    .on_mouse_up(MouseButton::Left, ctx.listener(Self::on_mouse_up))
                    .on_mouse_move(ctx.listener(Self::on_mouse_move))
                    .child(
                        // Content with scroll offset
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .top(self.scroll_offset)
                            .relative()
                            .children(self.device_list.iter().enumerate().map(
                                |(idx, bt)| {
                                    let name = &bt.name;
                                    let is_connected = bt.connected;
                                    let is_paired = bt.paired;
                                    let bt_tx = self.bt_tx.clone();

                                    let icon_color = if is_connected.clone() {
                                        rgb(0xC67600)
                                    } else {
                                        rgb(0xD2D2D2)
                                    };                                    

                                    let mut bluetooth_div = div()
                                        .id(("bluetooth_item", idx))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .h(px(60.))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .text_color(rgb(0xD2D2D2))
                                                .text_lg()
                                                .text_align(TextAlign::Left)
                                                .child(
                                                    div().pr_2().child(
                                                        Icon::new(get_bluetooth_icon(
                                                            is_connected.clone(),
                                                        ))
                                                        .size((px(28.), px(28.)))
                                                        .text_color(icon_color),
                                                    ),
                                                )
                                                .child(name.clone()),
                                        )
                                        .child(div().text_color(rgb(0x8F8F8F)).text_base().child(
                                            if is_connected { "Connected" } else { "" },
                                        ));

                                    if !is_connected && !is_paired {
                                        // TODO: call pair and connect device
                                        // TODO: check if device need auth - open portal
                                        bluetooth_div = bluetooth_div.on_click(ctx.listener(
                                            move |_, _, _, _| {
                                                println!(
                                                    "TODO: open portal/settings with params:"
                                                );
                                            },
                                        ));
                                    } else if !is_connected && is_paired {
                                        let address = bt.address.clone();
                                        let bt_tx_clone = bt_tx.clone();

                                        bluetooth_div =
                                            bluetooth_div
                                            .on_click(ctx.listener(
                                            move |_,
                                            _event: &ClickEvent,
                                            window: &mut Window,
                                            cx: &mut Context<Self>| {
                                        let address_clone = address.clone();
                                        let mut bt_tx = bt_tx_clone.clone();

                                                
                                                cx.background_executor()
                                                    .spawn(async move {
                                                        let _ = bt_tx
                                                            .send(BtEvents::ConnectDevice {
                                                                address: address_clone,
                                                            })
                                                            .await;

                                                    })
                                                    .detach();
                                                window.remove_window();
                                            },
                                        ))
                                    } 

                                    bluetooth_div
                                },
                            )),
                    ),
            )
    }
}
