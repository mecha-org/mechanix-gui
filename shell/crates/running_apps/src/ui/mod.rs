use std::{ time::Duration };
mod icon;
use gpui::prelude::*;
use gpui::*;
pub mod models;
pub mod constants;
pub use constants::*;
pub use models::{
    RunningApps,
    AppCard,
    DragDirection,
    AppDetails,
    AppInstance,
    AppMessage,
    AppManagerMessage,
};
pub mod desktop_entries;
pub mod desktop_models;

pub mod app_manager;
pub use app_manager::AppManagerService;
use tokio::sync::mpsc;
use crate::ui::icon::{ Icon, IconName };

const NAVBAR_SIZE: (f32, f32) = (120.0, 29.0);
const APP_SIZE: (f32, f32) = (540.0, 620.0);

impl Render for RunningApps {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bar_fixed_pos = APP_SIZE.1 - NAVBAR_SIZE.1;
        let current_bar_y = bar_fixed_pos + self.bar_drag_offset;

        div()
            .w_full()
            .h_full()
            .when(self.show_apps, |this| {
                this.child(
                    div()
                        .w_full()
                        .h_full()
                        .absolute()
                        .top(px(self.position))
                        .child(self.running_apps(cx))
                )
            })
            .on_mouse_move(
                cx.listener(move |this, event: &MouseMoveEvent, window, cx| {
                    if let Some(start_y) = this.bar_drag_start_y {
                        let current_y = event.position.y.to_f64() as f32;
                        let offset = current_y - start_y;
                        this.bar_drag_offset = offset.min(0.0);
                        cx.notify();
                    }
                })
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.bar_drag_start_y.is_some() {
                        if this.bar_drag_offset < -80.0 {
                            //long swipe
                            //Show running apps
                            this.update_input_regions(window, !this.show_apps);
                            this.show_apps = !this.show_apps;
                        } else {
                            //short swipe
                            //Mimize all apps
                        }
                        this.bar_drag_start_y = None;
                        this.snap_bar_to(0.0, cx);
                        cx.notify();
                    }
                })
            )
            .child(
                div()
                    .id("running-apps-navbar")
                    .w_full()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .absolute()
                    .top(px(current_bar_y))
                    .h(px(NAVBAR_SIZE.1))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, window, cx| {
                            cx.stop_propagation();
                            this.bar_drag_start_y = Some(event.position.y.to_f64() as f32);
                            cx.notify();
                        })
                    )
                    .child(
                        div().bg(rgb(0x4d4d4d)).w(px(NAVBAR_SIZE.0)).h(px(4.0)) // img(IconName::Navbar.resolve()).id("running-apps-navbar")
                    )
            )
    }
}

// Drag data structure
#[derive(Clone, Copy)]
struct CardDragData {
    start_position: Point<Pixels>,
}

impl CardDragData {
    fn new() -> Self {
        Self {
            start_position: Point::default(),
        }
    }
    fn position(mut self, pos: Point<Pixels>) -> Self {
        self.start_position = pos;
        self
    }
}

impl Render for CardDragData {
    fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
        Empty
    }
}

impl RunningApps {
    pub fn new(message_tx: mpsc::Sender<AppManagerMessage>) -> Self {
        let apps = Vec::new();
        let last_index = if apps.is_empty() { 0 } else { apps.len() - 1 };
        let initial_scroll_offset = Self::calculate_center_offset_for_index_static(last_index);

        Self {
            scroll_offset: initial_scroll_offset,
            target_scroll_offset: initial_scroll_offset,
            is_dragging: false,
            drag_start_x: px(0.0),
            drag_start_y: px(0.0),
            drag_start_offset: px(0.0),
            apps,
            dragging_card: None,
            drag_direction: None,
            is_animating: false,
            is_removing: false,
            removing_card_id: None,
            current_center_index: last_index,
            is_cleaning_up: false,
            message_tx: message_tx,
            position: 0.0,
            bar_drag_offset: 0.0,
            bar_drag_start_y: None,
            show_apps: false,
        }
    }

    fn calculate_center_offset_for_index_static(index: usize) -> Pixels {
        let card_position = (index as f32) * (CARD_WIDTH + CARD_GAP);
        let center_point = (CONTAINER_WIDTH - CARD_WIDTH) / 2.0;
        px(center_point - card_position)
    }

    pub fn calculate_center_offset(&self, index: usize) -> Pixels {
        Self::calculate_center_offset_for_index_static(index)
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
            if target_index > 0 {
                target_index -= 1;
            }
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
                        this.scroll_offset = this.scroll_offset + diff * lerp_factor;
                    }

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

        cx.spawn(async move |this, mut cx| {
            Self::run_removal_animation_loop(
                this,
                &mut cx,
                card_id,
                app_id_to_close,
                message_tx
            ).await;
        }).detach();
    }

    fn find_app_id(&self, card_id: usize) -> Option<String> {
        self.apps
            .iter()
            .find(|a| a.id == card_id)
            .map(|app| app.app_id.clone())
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
            app.offset_y = app.offset_y + diff * 0.2;
            cx.notify();
            false
        }
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

    fn animate_clean_up(&mut self, cx: &mut Context<Self>) {
        if !self.should_start_animation() {
            return;
        }

        self.is_animating = true;

        self.send_close_all_apps(cx);
        self.start_cleanup_animation_loop(cx);
    }

    fn should_start_animation(&self) -> bool {
        if !self.is_cleaning_up || self.is_animating {
            return false;
        }

        true
    }

    fn send_close_all_apps(&self, cx: &mut Context<Self>) {
        if let tx = self.message_tx.clone() {
            cx.background_executor()
                .spawn(async move {
                    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();

                    if
                        let Err(e) = tx.send(AppManagerMessage::CloseAllApps {
                            reply_to: reply_tx,
                        }).await
                    {
                        eprintln!("❌ Failed to send CloseAllApps message: {}", e);
                        return;
                    }

                    match reply_rx.await {
                        Ok(Ok(success)) => println!("✅ All apps closed successfully: {}", success),
                        Ok(Err(e)) => eprintln!("❌ Error closing all apps: {}", e),
                        Err(e) => eprintln!("❌ Reply channel error: {}", e),
                    }
                })
                .detach();
        }
    }

    fn start_cleanup_animation_loop(&mut self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_millis(16)).await;

                let mut stop = false;

                let _ = this.update(cx, |this, cx| {
                    stop = this.step_cleanup_animation(cx);
                });

                if stop {
                    break;
                }
            }
        }).detach();
    }

    fn step_cleanup_animation(&mut self, cx: &mut Context<Self>) -> bool {
        if !self.is_cleaning_up {
            self.is_animating = false;
            return true; // stop
        }

        if self.all_cards_reached_target() {
            self.finish_cleanup(cx);
            return true; // stop
        }

        self.animate_card_positions();
        cx.notify();

        false // continue
    }

    fn all_cards_reached_target(&self) -> bool {
        self.apps.iter().all(|app| {
            let diff = app.target_offset_y - app.offset_y;
            diff.abs() < px(1.0)
        })
    }

    fn finish_cleanup(&mut self, cx: &mut Context<Self>) {
        self.apps.clear();
        self.scroll_offset = px(0.0);
        self.target_scroll_offset = px(0.0);
        self.current_center_index = 0;
        self.is_cleaning_up = false;
        self.is_animating = false;

        cx.notify();
    }
    fn animate_card_positions(&mut self) {
        let lerp_factor = 0.2;

        for app in self.apps.iter_mut() {
            let diff = app.target_offset_y - app.offset_y;
            app.offset_y += diff * lerp_factor;
        }
    }

    fn handle_clean_up(&mut self, cx: &mut Context<Self>) {
        if self.apps.is_empty() {
            return;
        }

        // Set all cards to animate upward with staggered delays
        for (i, app) in self.apps.iter_mut().enumerate() {
            // Stagger the animation by adding delay based on index
            let delay_offset = (i as f32) * 30.0; // 30px stagger between cards
            app.target_offset_y = px(VERTICAL_TARGET_THRESHOLD - delay_offset);
        }

        self.is_cleaning_up = true;
        self.is_animating = false;
        self.is_removing = false;

        // Start the animation
        self.animate_clean_up(cx);
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

    fn determine_drag_direction(&mut self, delta_x: Pixels, delta_y: Pixels) {
        if self.drag_direction.is_none() {
            let abs_delta_x = delta_x.abs();
            let abs_delta_y = delta_y.abs();

            if
                abs_delta_x > px(DRAG_DETECTION_THRESHOLD) ||
                abs_delta_y > px(DRAG_DETECTION_THRESHOLD)
            {
                self.drag_direction = if abs_delta_x > abs_delta_y {
                    Some(DragDirection::Horizontal)
                } else {
                    Some(DragDirection::Vertical)
                };
            }
        }
    }

    fn handle_horizontal_drag(&mut self, delta_x: Pixels, cx: &mut Context<Self>) {
        // Horizontal scrolling (swipe) - free scroll during drag
        self.scroll_offset = self.drag_start_offset + delta_x;

        // Compute bounds such that first/last card can be centered
        let center_offset = (CONTAINER_WIDTH - CARD_WIDTH) / 2.0;
        let max_scroll = px(center_offset - PADDING);

        // last card position (x) = (n-1) * (card_width + gap) + padding
        let last_index = if self.apps.is_empty() { 0 } else { self.apps.len() - 1 };
        let min_scroll = Self::calculate_center_offset_for_index_static(last_index);

        self.scroll_offset = self.scroll_offset.clamp(min_scroll, max_scroll);
        cx.notify();
    }

    fn handle_vertical_drag(&mut self, delta_y: Pixels, cx: &mut Context<Self>) {
        if let Some(card_id) = self.dragging_card {
            if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
                app.offset_y = if delta_y <= px(0.0) { delta_y } else { px(0.0) };
                cx.notify();
            }
        }
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
            } else {
                if let Some(app) = self.apps.iter_mut().find(|a| a.id == card_id) {
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

    fn on_app_click(&mut self, app_id: String, cx: &mut Context<Self>) {
        println!("RunningApps::on_app_click() - app_id: {}", app_id);
        if let ref tx = self.message_tx {
            let tx_clone = tx.clone();
            let app_id_clone = app_id.clone();

            cx.background_executor()
                .spawn(async move {
                    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();

                    if
                        let Err(e) = tx_clone.send(AppManagerMessage::LaunchApp {
                            app_id: app_id_clone,
                            reply_to: reply_tx,
                        }).await
                    {
                        eprintln!("❌ Failed to send ActivateAppInstance message: {}", e);
                    } else {
                        match reply_rx.await {
                            Ok(Ok(success)) => {
                                println!("✅ App activated successfully: {}", success);
                            }
                            Ok(Err(e)) => {
                                eprintln!("❌ Error activating app: {}", e);
                            }
                            Err(e) => {
                                eprintln!("❌ Reply channel error: {}", e);
                            }
                        }
                    }
                })
                .detach();
        }
        cx.stop_propagation();
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
                    app.offset_y = app.offset_y + diff * lerp_factor;

                    cx.notify();
                });

                if should_stop {
                    break;
                }
            }
        }).detach();
    }
}

impl RunningApps {
    fn closed_pos() -> f32 {
        APP_SIZE.1 - NAVBAR_SIZE.1
    }

    fn snap_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.position;
        let change = target - start;
        let duration_ms = 250.0; // Animation speed
        let start_time = std::time::Instant::now();

        cx.spawn(async move |this: WeakEntity<RunningApps>, cx: &mut AsyncApp| {
            loop {
                let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                // Check if animation is done
                if elapsed >= duration_ms {
                    this.update(cx, |this, cx| {
                        this.position = target;
                        cx.notify();
                    }).ok();
                    break;
                }

                let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                let ease = 1.0 - (1.0 - t).powi(3);
                let current = start + change * ease;

                this.update(cx, |this, cx| {
                    this.position = current;
                    cx.notify();
                }).ok();

                cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
            }
        }).detach();
    }

    fn snap_bar_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.bar_drag_offset;
        let change = target - start;
        let duration_ms = 250.0; // Animation speed
        let start_time = std::time::Instant::now();

        cx.spawn(async move |this: WeakEntity<RunningApps>, cx: &mut AsyncApp| {
            loop {
                let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                // Check if animation is done
                if elapsed >= duration_ms {
                    this.update(cx, |this, cx| {
                        this.bar_drag_offset = target;
                        cx.notify();
                    }).ok();
                    break;
                }

                let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                let ease = 1.0 - (1.0 - t).powi(3);
                let current = start + change * ease;

                this.update(cx, |this, cx| {
                    this.bar_drag_offset = current;
                    cx.notify();
                }).ok();

                cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
            }
        }).detach();
    }

    fn update_input_regions(&self, window: &mut Window, open: bool) {
        let mut regions = Vec::new();

        if open {
            regions.push(Bounds {
                origin: point(px(0.0), px(0.0)),
                size: size(px(APP_SIZE.0), px(APP_SIZE.1)),
            });
        } else {
            regions.push(Bounds {
                origin: point(
                    px((APP_SIZE.0 - NAVBAR_SIZE.0) / 2.0),
                    px(APP_SIZE.1 - NAVBAR_SIZE.1)
                ),
                size: size(px(NAVBAR_SIZE.0), px(NAVBAR_SIZE.1)),
            });
        }
        window.set_input_regions(Some(regions));
    }

    fn running_apps(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_apps = !self.apps.is_empty();
        let should_center = self.apps.len() == 1;

        div()
            .flex()
            .w_full()
            .h_full()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(rgb(0x000000))
            .when(has_apps, |this| {
                this.child(
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

                            for i in 0..self.apps.len() {
                                let id = self.apps[i].id;
                                let app_id = self.apps[i].app_id.clone();
                                let offset_y = self.apps[i].offset_y;
                                let app_icon_path = self.apps[i].app_icon_path.clone();
                                let app_name: Option<String> = self.apps[i].app_name.clone();
                                let card_id: SharedString = format!("card-{}", app_id).into();
                                container = container.child(
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
                                                .child(
                                                    Icon::from(IconName::BgApp).size((
                                                        px(CARD_WIDTH),
                                                        px(CARD_HEIGHT),
                                                    ))
                                                )
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
                                        .on_drag(
                                            CardDragData::new(),
                                            move |_: &CardDragData, pos, _, cx| {
                                                let data = CardDragData::new().position(pos);
                                                cx.new(|_| data)
                                            }
                                        )
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
                                                                .when_some(
                                                                    app_icon_path.clone(),
                                                                    |d, s| {
                                                                        let image_path =
                                                                            std::path::PathBuf
                                                                                ::from(s)
                                                                                .clone();

                                                                        d.child(
                                                                            img(image_path)
                                                                                .w(px(16.0))
                                                                                .h(px(16.0))
                                                                        )
                                                                    }
                                                                )
                                                        )
                                                        .child(
                                                            div()
                                                                .font_weight(FontWeight(400.0))
                                                                .text_size(px(16.0))
                                                                .text_color(rgb(0xf4f4f4))
                                                                .when_some(
                                                                    app_name.clone(),
                                                                    |d, s| { d.child(s) }
                                                                )
                                                        )
                                                )
                                        )
                                );
                            }

                            container
                        })
                )
            })
            .when(!has_apps, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_16()
                        .child(
                            div()
                                .text_color(rgb(0x666666))
                                .text_size(px(16.0))
                                .line_height(px(24.0))
                                .text_center()
                                .font_weight(FontWeight(400.0))
                                .max_w(px(300.0))
                                .child("There are no apps or droids")
                                .child(div().child("you are looking for_"))
                        )
                )
            })
            // fixed positioned footer button
            .child(
                div()
                    .absolute()
                    .bottom_16()
                    .child({
                        let is_enabled = !self.apps.is_empty();
                        let mut btn = div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .gap_2()
                            .w(px(109.0))
                            .h(px(36.0))
                            .px(px(8.0))
                            .py(px(12.0))
                            .rounded(px(8.0))
                            .bg(if is_enabled { rgb(0x363636) } else { rgb(0x202020) })
                            .cursor(
                                if is_enabled {
                                    CursorStyle::PointingHand
                                } else {
                                    CursorStyle::default()
                                }
                            )
                            .child(
                                Icon::from(IconName::CleanUp)
                                    .size((px(20.0), px(20.0)))
                                    .text_color(
                                        if is_enabled {
                                            rgb(0xf4f4f4)
                                        } else {
                                            rgb(0x4d4d4d)
                                        }
                                    )
                            )
                            .child(
                                div()
                                    .text_color(
                                        if is_enabled {
                                            rgb(0xf4f4f4)
                                        } else {
                                            rgb(0x4d4d4d)
                                        }
                                    )
                                    .opacity(if is_enabled { 1.0 } else { 0.4 })
                                    .text_size(px(16.0))
                                    .child("Clean up")
                            );

                        if is_enabled {
                            btn = btn.on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|view, _event, _window, cx| {
                                    view.handle_clean_up(cx);
                                })
                            );
                        }

                        btn
                    })
            )
    }
}
