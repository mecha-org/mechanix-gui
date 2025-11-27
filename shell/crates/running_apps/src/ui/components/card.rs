use gpui::prelude::*;
use gpui::*;
use tracing::info;
use crate::config::constants::*;
use crate::models::models::{ DragDirection };
use crate::prelude::app_manager::AppManagerMessage;
use crate::ui::{ CardDragData, RunningApps };
use crate::ui::icon::{ Icon, IconName };

impl RunningApps {
    fn on_app_click(&mut self, app_id: String, cx: &mut Context<Self>) {
        let tx_clone = self.message_tx.clone();
        let app_id_clone = app_id.clone();

        cx.background_executor()
            .spawn(async move {
                let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();

                if
                    let Err(_) = tx_clone.send(AppManagerMessage::LaunchApp {
                        app_id: app_id_clone,
                        reply_to: reply_tx,
                    }).await
                {
                } else {
                    if let Ok(Ok(success)) = reply_rx.await {
                        info!("✅ App activated successfully:: {}", success);
                    }
                }
            })
            .detach();

        cx.stop_propagation();
    }
    fn handle_drag_move(
        &mut self,
        event: &DragMoveEvent<CardDragData>,
        _window: &mut Window,
        cx: &mut Context<Self>
    ) {
        if !self.is_dragging {
            return;
        }

        // Use the stored start position from drag data
        let delta_x = event.event.position.x - self.drag_start_x;
        let delta_y = event.event.position.y - self.drag_start_y;
        self.determine_drag_direction(delta_x, delta_y);

        match self.drag_direction {
            Some(DragDirection::Horizontal) => self.handle_horizontal_drag(delta_x, cx),
            Some(DragDirection::Vertical) => self.handle_vertical_drag(delta_y, cx),
            // No direction determined yet
            None => {}
        }
    }

    fn handle_card_mouse_down(
        &mut self,
        app_id: usize,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>
    ) {
        self.is_dragging = true;
        self.is_animating = false;
        self.dragging_card = Some(app_id);
        self.drag_start_x = event.position.x;
        self.drag_start_y = event.position.y;
        self.drag_start_offset = self.scroll_offset;
        self.drag_direction = None;
        cx.stop_propagation();
    }

    pub fn render_card(&self, cx: &mut Context<'_, Self>, index: usize) -> impl IntoElement {
        let id = self.apps[index].id;
        let app_id = self.apps[index].app_id.clone();
        let offset_y = self.apps[index].offset_y;
        let app_icon_path = self.apps[index].app_icon_path.clone();
        let app_name: Option<String> = self.apps[index].app_name.clone();
        let card_id: SharedString = format!("card-{}", app_id).into();

        div()
            .id(card_id)

            .on_click(
                cx.listener(move |view, _, _, cx| {
                    view.on_app_click(app_id.clone(), cx);
                })
            )
            .relative()
            .flex()
            .w(px(CARD_WIDTH))
            .h(px(CARD_HEIGHT))
            .justify_center()
            .items_center()
            .child(
                div()
                    .absolute()
                    .left_0()
                    .top_0()
                    .child(Icon::from(IconName::BgApp).size((px(CARD_WIDTH), px(CARD_HEIGHT))))
            )
            .relative()
            .when_some(app_icon_path.clone(), |d, s| {
                let image_path = std::path::PathBuf::from(s);
                d.child(img(image_path).w(px(40.0)).h(px(40.0)))
            })

            .rounded(px(16.0))
            .top(offset_y)
            .cursor_pointer()
            .on_drag_move(cx.listener(Self::handle_drag_move))
            .on_drag(CardDragData::new(), move |_: &CardDragData, pos, _, cx| {
                let data = CardDragData::new().position(pos);
                cx.new(|_| data)
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |view, event, window, cx| {
                    view.handle_card_mouse_down(id, event, window, cx);
                })
            )
            .child(
                div()
                    .flex()
                    .absolute()
                    .top(px(8.0))
                    .left(px(8.0))
                    .w(px(100.0))
                    .h(px(16.0))
                    .justify_start()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .h(px(16.0))
                            .w(px(106.0))
                            .child(
                                div()
                                    .rounded(px(3.2))
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .flex()
                                    .justify_center()
                                    .items_center()
                                    .when_some(app_icon_path.clone(), |d, s| {
                                        let image_path = std::path::PathBuf::from(s).clone();

                                        d.child(img(image_path).w(px(16.0)).h(px(16.0)))
                                    })
                            )
                            .child(
                                div()
                                    .font_weight(FontWeight(400.0))
                                    .text_size(px(16.0))
                                    .text_color(rgb(0xf4f4f4))
                                    .when_some(app_name.clone(), |d, s| { d.child(s) })
                            )
                    )
            )
    }
}
