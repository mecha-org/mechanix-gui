use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    style::{HorizontalPosition, Styled},
    txt,
    widgets::{Div, Text},
    Color, Node,
};

#[derive(Debug, Clone)]
pub enum WindowTitleMessage {
    CurrentWindowTitleUpdate(String),
    TopLevelActiveUpdate(bool),
}

#[derive(Debug)]
pub struct WindowTitleComponent {
    pub current_window_title: String,
}

impl Component for WindowTitleComponent {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                // .bg(0xFF00FFFF)
                ,
                [
                    size: [215, 24],
                ],
            )
            .push(node!(Text::new(
                txt!(self.current_window_title.clone()))
                .style("font", "SpaceGrotesk-Bold")
                .style("color",Color::rgb(255., 255., 255.))
                .style("size", 14.0)
                .style("h_alignment", HorizontalPosition::Center),
               [size_pct: [100.0, Auto]])),
        )
    }
}
