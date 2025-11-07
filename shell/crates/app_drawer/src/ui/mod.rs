use gpui::*;

pub struct AppDrawer {}

impl AppDrawer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for AppDrawer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::black())
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .text_color(gpui::white())
            .child("AppDrawer")
    }
}
