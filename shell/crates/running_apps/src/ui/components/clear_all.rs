use crate::models::models::AppCardAnimation;
use crate::ui::RunningApps;
use crate::ui::icon::IconName;
use gpui::prelude::*;
use gpui::*;

impl RunningApps {
    pub fn clear_all_cards(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.apps.is_empty() || self.is_animating() {
            return;
        }

        self.animation_state = AppCardAnimation::ClearingAll {
            start_time: std::time::Instant::now(),
        };

        for (tl, _) in self.apps.clone() {
            tl.close();
        }

        self.schedule_animation_frame(cx);
    }

    pub fn render_clear_all(&self, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_1_4()
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, event, _window_, cx| {
                    this.on_mouse_up(event, cx);
                }),
            )
            .child(
                div()
                    .bg(rgb(0x151515))
                    .p_2()
                    .rounded_md()
                    .text_color(gpui::white())
                    .flex()
                    .flex_row()
                    .gap_2()
                    .items_center()
                    .justify_center()
                    .child(img(IconName::CleanUp.resolve()))
                    .child("Close All")
                    .cursor_pointer()
                    .hover(|s| s.bg(rgb(0x252525)))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _event, window, cx| {
                            this.clear_all_cards(window, cx);
                            cx.stop_propagation();
                        }),
                    ),
            )
    }
}
