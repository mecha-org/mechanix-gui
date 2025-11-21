pub mod app;
pub mod models;
pub mod theme;

use gpui::*;

pub struct Launcher {}

impl Launcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Launcher {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::black())
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .text_color(gpui::white())
            .child("Launcher")
    }
}
