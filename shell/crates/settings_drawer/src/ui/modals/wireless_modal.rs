use futures::{SinkExt, channel::mpsc};
use gpui::*;
use networkmanager::interfaces::wireless::WirelessNetworkInfo;

use crate::{
    events::NmEvents, get_wireless_strength_icon, prelude::*, ui::{
        icon::{Icon, IconName},
    }
};

const ROW_HEIGHT: f32 = 60.0;
const HEADER_HEIGHT: f32 = 60.0;
const FOOTER_HEIGHT: f32 = 60.0;

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

    fn estimate_content_height(&self) -> Pixels {
        let item_height = px(ROW_HEIGHT);
        let gap = px(8.);
        let item_count = self.network_list.len() as f32;

        if item_count == 0.0 {
            px(0.)
        } else {
            item_count * item_height + (item_count - 1.0) * gap
        }
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _: &mut Window,
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
            
            let container_height = window_height - px(8.) - px(HEADER_HEIGHT) - px(FOOTER_HEIGHT);
            let content_height = self.estimate_content_height();

            let (min_scroll, max_scroll) =
                self.calculate_scroll_bounds(content_height, container_height);

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
            .bg(rgb(DARK_NEUTRAL_900))
            .size_full()
            .border_1()
            .rounded_xl()
            .border_color(rgb(AMBER_900))
            // Header  
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .p_4()
                    .h(px(HEADER_HEIGHT))
                    .border_b_1()
                    .bg(rgb(DARK_NEUTRAL_800))
                    .flex_shrink_0()  
                    .child(
                        div()
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(DARK_NEUTRAL_0))
                            .child(self.title.clone()),
                    )
            )
            // Scrollable container  
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_hidden()  
                    .relative()  
                    .on_mouse_down(MouseButton::Left, ctx.listener(Self::on_mouse_down))
                    .on_mouse_up(MouseButton::Left, ctx.listener(Self::on_mouse_up))
                    .on_mouse_move(ctx.listener(Self::on_mouse_move))
                    .child(
                        // Content with scroll offset 
                        div()
                            .absolute()  
                            .top(self.scroll_offset)  
                            .left(px(0.))
                            .right(px(0.))
                            .flex()
                            .flex_col()
                            .gap_2()
                            .children(self.network_list.iter().enumerate().map(
                                |(idx, network)| {
                                    let ssid = network.ssid.clone();
                                    let is_active = network.is_active;
                                    let is_known = network.is_known;
                                    let nm_tx = self.nm_tx.clone();
                                    let icon_color = if network.is_active {
                                        rgb(AMBER_600)
                                    } else {
                                        rgb(DARK_NEUTRAL_100)
                                    };         
                                    let wifi_icon = get_wireless_strength_icon(
                                        network.is_active,
                                        network.signal_strength,
                                        network.security.clone(),
                                    );                      
                           
                                    let mut network_div = if is_active {
                                        div()
                                        .id(("network_item", idx))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .h(px(ROW_HEIGHT))
                                        .px_4()
                                        .bg(rgba(AMBER_600_10))
                                        .border_y_1()
                                        .border_color(rgb(AMBER_900))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .child(
                                                    div().pr_2().child(
                                                        Icon::new(wifi_icon)
                                                        .size((px(28.), px(28.)))
                                                        .text_color(icon_color),
                                                    ),
                                                )
                                                .child(
                                                    div()
                                                    .text_color(icon_color)
                                                    .text_lg()
                                                    .child(network.ssid.clone())
                                                    ),
                                        )
                                        .child( 
                                            Icon::new(IconName::ConnectedIcon)
                                                .size((px(24.), px(24.)))
                                                .text_color(rgb(AMBER_600))
                                        )
                                    } 
                                    else {
                                        div()
                                        .id(("network_item", idx))
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .h(px(ROW_HEIGHT))
                                        .px_4()
                                        .rounded_md()
                                        .bg(rgb(DARK_NEUTRAL_900))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .child(
                                                    div().pr_2().child(
                                                        Icon::new(wifi_icon)
                                                        .size((px(28.), px(28.)))
                                                        .text_color(icon_color),
                                                    ),
                                                )
                                                 .child(
                                                    div()
                                                    .text_color(icon_color)
                                                    .text_lg()
                                                    .child(network.ssid.clone())
                                                    ),
                                        )
                                    };

                                    if !is_active && !is_known {
                                        network_div = network_div.on_click(ctx.listener(
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

                                        network_div = network_div.on_click(ctx.listener(
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
                                                window.remove_window();
                                            },
                                        ));
                                    }

                                    network_div
                                },
                            )),
                    ),
            )
            // Footer  
            .child(
                div()
                    .id("id_settings")
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_start()
                    .border_t_1()
                    .border_color(rgb(DARK_NEUTRAL_700))
                    .h(px(FOOTER_HEIGHT))
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
                    }))
            )
    }
}