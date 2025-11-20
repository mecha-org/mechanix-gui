use gpui::*;

pub struct Homescreen {}

impl Homescreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Homescreen {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().w_full().h_full().bg(rgb(0xFF69B4)) // Hot pink color
    }
}
