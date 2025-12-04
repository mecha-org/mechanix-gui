use futures::{SinkExt, channel::mpsc};
use gpui::*;
use networkmanager::interfaces::wireless::WirelessNetworkInfo;

use crate::{
    events::NmEvents,
    get_wireless_strength_icon,
    ui::{
        icon::{Icon, IconName},
        widgets::IconButton,
    },
};

pub struct WirelessWindow {
    pub title: String,
    pub network_list: Vec<WirelessNetworkInfo>,
    pub nm_tx: mpsc::Sender<NmEvents>,

    // Scrolling state
    scroll_offset: Pixels,
    last_scroll_offset: Pixels,
    drag_start_y: Pixels,
    is_dragging: bool,
}

impl WirelessWindow {
    pub fn new(
        title: String,
        network_list: Vec<WirelessNetworkInfo>,
        nm_tx: mpsc::Sender<NmEvents>,
    ) -> Self {
        Self {
            title,
            network_list,
            nm_tx,
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
        let header_height = px(60.); // Height of your header
        let padding = px(32.); // Total padding (p_4 is 16px * 2)
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
        let item_height = px(60.); // Height of each network item
        let gap = px(8.); // gap_2 is 8px
        let item_count = self.network_list.len() as f32;

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

impl Render for WirelessWindow {
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
                            .children(self.network_list.iter().enumerate().map(
                                |(idx, network)| {
                                    let ssid = network.ssid.clone();
                                    let is_active = network.is_active;
                                    let is_known = network.is_known.clone();
                                    let nm_tx = self.nm_tx.clone();
                                    let icon_color = if network.is_active {
                                        rgb(0xC67600)
                                    } else {
                                        rgb(0xD2D2D2)
                                    };                                    

                                    let mut network_div = div()
                                        .id(("network_item", idx))
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
                                                        Icon::new(get_wireless_strength_icon(
                                                            network.is_active,
                                                            network.signal_strength,
                                                            network.security.clone(),
                                                        ))
                                                        .size((px(28.), px(28.)))
                                                        .text_color(icon_color),
                                                    ),
                                                )
                                                .child(network.ssid.clone()),
                                        )
                                        .child(div().text_color(rgb(0x8F8F8F)).text_base().child(
                                            if network.is_active { "Connected" } else { "" },
                                        ));

                                    if !is_active && !is_known {
                                        // proceed to open settings with params
                                        network_div = network_div.on_click(ctx.listener(
                                            move |_, _, _, _| {
                                                println!(
                                                    "open settings for new network {:?} - {:?}",
                                                    ssid, is_known
                                                );
                                            },
                                        ));
                                    } else if !is_active && is_known {
                                        // proceed to connect
                                        let ssid_clone = ssid.clone();
                                        let nm_tx_clone = nm_tx.clone();

                                        network_div =
                                            network_div
                                            .on_click(ctx.listener(
                                move |_,
                                 _event: &ClickEvent,
                                 window: &mut Window,
                                 cx: &mut Context<Self>| {
                                    let ssid = ssid_clone.clone();
                                    let mut nm_tx = nm_tx_clone.clone();
                                    
                                    cx.background_executor()
                                        .spawn(async move {
                                            let _ = nm_tx
                                                .send(NmEvents::ConnectKnownNetwork { name: ssid })
                                                .await;

                                        })
                                        .detach();
                                },
                            ))
                                    }

                                    network_div
                                },
                            )),
                    ),
            )
    }
}
