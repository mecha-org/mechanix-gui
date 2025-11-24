use std::time::Duration;

use gpui::prelude::*;
use gpui::*;
use tokio::sync::mpsc;
use crate::config::constants::*;
use crate::models::models::{ AppManagerMessage, DragDirection };
use crate::ui::{ CardDragData, RunningApps };

impl RunningApps {
    fn animate_scroll(&mut self, cx: &mut Context<Self>) {
        if !self.is_animating || self.is_dragging {
            // already running → don’t start a new loop
            return;
        }
        // self.is_animating = true;

        // Start animation loop
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(16)).await;

                let mut should_stop = false;

                let _ = this.update(cx, |this, cx| {
                    // compute diff
                    let diff = this.target_scroll_offset - this.scroll_offset;
                    let threshold = px(0.5);

                    if diff.abs() < threshold {
                        // snap & stop
                        this.scroll_offset = this.target_scroll_offset;
                        should_stop = true;
                        this.is_animating = false;
                    } else {
                        // smooth lerp
                        let lerp_factor = 0.2;
                        this.scroll_offset += diff * lerp_factor;
                    }

                    cx.notify();
                });

                if should_stop {
                    break;
                }
            }
        }).detach();
    }

    fn recenter_scroll_after_removal(&mut self, cx: &mut Context<Self>) {
        if self.apps.is_empty() {
            self.scroll_offset = px(0.0);
            self.target_scroll_offset = px(0.0);
            self.current_center_index = 0;
            return;
        }

        self.target_scroll_offset = Self::calculate_center_offset_for_index_static(
            self.current_center_index
        );
        self.is_animating = true;
        self.animate_scroll(cx);
    }

    fn adjust_center_index_after_removal(&mut self, removed_index: Option<usize>) {
        let Some(idx) = removed_index else {
            return;
        };

        if idx < self.current_center_index {
            self.current_center_index = self.current_center_index.saturating_sub(1);
        } else if self.current_center_index >= self.apps.len() && !self.apps.is_empty() {
            self.current_center_index = self.apps.len() - 1;
        }
    }
    fn remove_card_and_update_state(
        &mut self,
        card_id: usize,
        removed_index: Option<usize>,
        cx: &mut Context<Self>
    ) {
        self.apps.retain(|a| a.id != card_id);
        self.is_removing = false;
        self.removing_card_id = None;

        self.adjust_center_index_after_removal(removed_index);
        self.recenter_scroll_after_removal(cx);

        cx.notify();
    }
    async fn execute_close_app(app_id: String, tx: mpsc::Sender<AppManagerMessage>) {
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();

        let send_result = tx.send(AppManagerMessage::CloseApp {
            app_id: app_id.clone(),
            reply_to: reply_tx,
        }).await;

        if let Err(e) = send_result {
            eprintln!("Failed to send CloseApp message: {}", e);
            return;
        }

        match reply_rx.await {
            Ok(Ok(success)) => {
                println!("✅ App {} closed successfully: {}", app_id, success);
            }
            Ok(Err(e)) => {
                eprintln!("❌ Error closing app {}: {}", app_id, e);
            }
            Err(e) => {
                eprintln!("❌ Reply channel error: {}", e);
            }
        }
    }
    fn send_close_app_message(
        app_id: String,
        tx: mpsc::Sender<AppManagerMessage>,
        cx: &mut Context<Self>
    ) {
        cx.background_executor()
            .spawn(async move {
                Self::execute_close_app(app_id, tx).await;
            })
            .detach();
    }

    fn finalize_card_removal(
        &mut self,
        card_id: usize,
        app_id_to_close: &Option<String>,
        message_tx: &mpsc::Sender<AppManagerMessage>,
        cx: &mut Context<Self>
    ) {
        // Snap to final position
        if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
            app.offset_y = app.target_offset_y;
        }

        // Send close message before removal
        if let Some(app_id) = app_id_to_close {
            Self::send_close_app_message(app_id.clone(), message_tx.clone(), cx);
        }

        let removed_index = self.apps.iter().position(|a| a.id == card_id);
        self.remove_card_and_update_state(card_id, removed_index, cx);
    }

    fn process_animation_frame(
        &mut self,
        card_id: usize,
        app_id_to_close: &Option<String>,
        message_tx: &mpsc::Sender<AppManagerMessage>,
        cx: &mut Context<Self>
    ) -> bool {
        let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) else {
            return true; // Card already gone
        };

        let diff = app.target_offset_y - app.offset_y;
        let threshold = px(0.5);
        if diff.abs() < threshold {
            self.finalize_card_removal(card_id, app_id_to_close, message_tx, cx);
            true
        } else {
            app.offset_y += diff * 0.2;
            cx.notify();
            false
        }
    }

    async fn run_removal_animation_loop(
        this: WeakEntity<Self>,
        cx: &mut AsyncApp,
        card_id: usize,
        app_id_to_close: Option<String>,
        message_tx: mpsc::Sender<AppManagerMessage>
    ) {
        loop {
            cx.background_executor().timer(Duration::from_millis(16)).await;

            let should_stop = this
                .update(cx, |view, cx| {
                    view.process_animation_frame(card_id, &app_id_to_close, &message_tx, cx)
                })
                .unwrap_or(true);

            if should_stop {
                break;
            }
        }
    }
    fn animate_snap_back(&mut self, card_id: usize, cx: &mut Context<Self>) {
        if self.is_animating {
            return;
        }

        self.is_animating = true;

        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(16)).await;

                let mut should_stop = false;

                let _ = this.update(cx, |this, cx| {
                    // Find card again each frame
                    let Some(app) = this.apps.iter_mut().find(|a| a.id == card_id) else {
                        // Card deleted mid-animation → stop
                        this.is_animating = false;
                        should_stop = true;
                        return;
                    };

                    let diff = app.target_offset_y - app.offset_y;
                    let threshold = px(0.5);

                    if diff.abs() < threshold {
                        // Snap and stop
                        app.offset_y = app.target_offset_y;
                        this.is_animating = false;
                        should_stop = true;

                        cx.notify();
                        return;
                    }

                    // Lerp movement
                    let lerp_factor = 0.25;
                    app.offset_y += diff * lerp_factor;

                    cx.notify();
                });

                if should_stop {
                    break;
                }
            }
        }).detach();
    }
    fn animate_card_removal(&mut self, card_id: usize, cx: &mut Context<Self>) {
        if !self.is_removing {
            return;
        }

        let app_id_to_close = self.find_app_id(card_id);
        let message_tx = self.message_tx.clone();

        cx.spawn(async move |this, cx| {
            Self::run_removal_animation_loop(this, cx, card_id, app_id_to_close, message_tx).await;
        }).detach();
    }
    fn handle_vertical_drop(&mut self, card_id: usize, cx: &mut Context<Self>) {
        if let Some(pos) = self.apps.iter().position(|a| a.id == card_id) {
            let offset_y = self.apps[pos].offset_y;

            if offset_y < px(VERTICAL_DISMISS_THRESHOLD) {
                if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                    app.target_offset_y = px(VERTICAL_TARGET_THRESHOLD);
                    self.is_removing = true;
                    self.removing_card_id = Some(card_id);
                    self.animate_card_removal(card_id, cx);
                }
            } else if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                app.target_offset_y = px(0.0);
                let card_id_copy = card_id;
                cx.spawn(async move |this, cx| {
                    let _ = this.update(cx, |this, cx| {
                        this.animate_snap_back(card_id_copy, cx);
                    });
                }).detach();
            }
        }
    }
    fn snap_to_nearest_card_with_threshold(&mut self) {
        if self.apps.is_empty() {
            return;
        }

        // Calculate drag distance from start
        let drag_delta = self.scroll_offset - self.drag_start_offset;
        let drag_distance = drag_delta.to_f64() as f32;
        // Calculate threshold: 20% of card width + gap
        let card_step = CARD_WIDTH + CARD_GAP;
        let switch_threshold = card_step * HORIZONTAL_SWITCH_THRESHOLD;

        let mut target_index = self.current_center_index;

        if drag_distance > switch_threshold {
            // Dragged right (showing previous card) - move to left
            target_index = target_index.saturating_sub(1);
        } else if drag_distance < -switch_threshold {
            // Dragged left (showing next card) - move to right
            if target_index < self.apps.len() - 1 {
                target_index += 1;
            }
        }
        // else: stay on current card (didn't drag enough)

        self.current_center_index = target_index;
        self.target_scroll_offset = Self::calculate_center_offset_for_index_static(target_index);
        self.is_animating = true;
        self.is_dragging = false;
    }

    fn handle_on_drop(&mut self, _: &CardDragData, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.is_dragging {
            return;
        }

        match self.drag_direction {
            Some(DragDirection::Vertical) => {
                if let Some(card_id) = self.dragging_card {
                    self.handle_vertical_drop(card_id, cx);
                }
            }
            Some(DragDirection::Horizontal) => {
                // Horizontal drag - snap to nearest card
                self.snap_to_nearest_card_with_threshold();
                self.animate_scroll(cx);
            }
            None => {}
        }

        self.is_dragging = false;
        self.dragging_card = None;
        self.drag_direction = None;
        cx.notify();
    }

    pub fn scroller_container(
        &self,
        cx: &mut Context<'_, Self>,
        should_center: bool
    ) -> impl IntoElement {
        div()
            .id("running_apps")
            .flex()
            .w_full()
            .h_full()
            .overflow_x_hidden()
            .on_drop(cx.listener(RunningApps::handle_on_drop))
            .items_center()
            .when(should_center, |d| d.justify_center())
            .child({
                let mut container = div().flex().flex_row().gap(px(CARD_GAP));

                if !should_center {
                    container = container.relative().left(self.scroll_offset);
                }
                // apps card
                for i in 0..self.apps.len() {
                    container = container.child(self.render_card(cx, i));
                }

                container
            })
    }
}
