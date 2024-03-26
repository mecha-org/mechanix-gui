use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    style::Styled,
    txt,
    widgets::{Div, Text},
    Color, Node,
};

#[derive(Debug, Clone)]
pub enum ClockMessage {
    TimeTick(String),
}

#[derive(Debug)]
pub struct ClockComponent {
    pub current_time: String,
}

impl Component for ClockComponent {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                // .bg(0xFF00FFFF)
                ,
                [
                    size: [80, 24],
                ],
            )
            .push(node!(Text::new(
                 txt!(self.current_time.clone()))
                 .style("font", "SpaceGrotesk-Bold")
                 .style("color",Color::rgb(255., 255., 255.))
                 .style("size", 14.0),
                [size_pct: [100.0, Auto]])),
        )
    }
}
