use gpui::{Context, IntoElement, Render, Window, *};

pub mod icon;

pub struct StatusBar {
    pub notification_list: String,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            notification_list: "Test".to_string(),
        }
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::transparent_white())
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h_full()
            .text_color(gpui::white())
            .pl_4()
            .pr_4()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .child(self.notification_list.clone())
                    .text_lg()
                    .text_color(rgb(0xE9E9E9)),
            )
    }
}
