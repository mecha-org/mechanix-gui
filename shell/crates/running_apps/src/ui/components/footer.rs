use std::time::Duration;

use gpui::prelude::*;
use gpui::*;
use crate::config::constants::VERTICAL_TARGET_THRESHOLD;
use crate::prelude::app_manager::AppManagerMessage;
use crate::ui::RunningApps;
use crate::ui::icon::{ Icon, IconName };

impl RunningApps {
    fn should_start_animation(&self) -> bool {
        if !self.is_cleaning_up || self.is_animating {
            return false;
        }

        true
    }

    fn animate_card_positions(&mut self) {
        let lerp_factor = 0.2;

        for app in self.apps.iter_mut() {
            let diff = app.target_offset_y - app.offset_y;
            app.offset_y += diff * lerp_factor;
        }
    }

    fn animate_clean_up(&mut self, cx: &mut Context<Self>) {
        if !self.should_start_animation() {
            return;
        }

        self.is_animating = true;

        self.send_close_all_apps(cx);
        self.start_cleanup_animation_loop(cx);
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
    fn send_close_all_apps(&self, cx: &mut Context<Self>) {
        let tx = self.message_tx.clone();
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
        cx.stop_propagation();
    }

    pub fn render_footer(&self, cx: &mut Context<'_, Self>) -> impl IntoElement {
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
                            .text_color(if is_enabled { rgb(0xf4f4f4) } else { rgb(0x4d4d4d) })
                    )
                    .child(
                        div()
                            .text_color(if is_enabled { rgb(0xf4f4f4) } else { rgb(0x4d4d4d) })
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
    }
}
