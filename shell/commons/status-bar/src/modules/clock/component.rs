use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    node, rect, size, size_pct,
    style::{FontWeight, HorizontalPosition, Styled},
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
                // .bg(Color::RED)
                ,
                [
                    size: [80, 24],
                    padding: [ 3., 0., 1., 0. ]
                ],
            )
            .push(node!(
                Text::new(txt!(self.current_time.clone()))
                    .style("color", Color::WHITE)
                    .style("size", 15.0)
                    .style("font_weight", FontWeight::Semibold),
                lay![
                    size_pct: [100],
                ]
            )),
        )
    }
}
