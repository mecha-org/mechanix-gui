use mctk_core::component;
use mctk_core::layout::Alignment;
use mctk_core::widgets::Svg;
use mctk_core::{component::Component, lay, node, size, size_pct, widgets::Div, Node};

#[derive(Debug)]
pub struct Settings {}

impl Component for Settings {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                ,
                [
                    size_pct: [100],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ],
            )
            .push(node!(
                Svg::new("settings_icon"),
                lay![
                    size: [28, 28],
                ],
            )),
        )
    }
}
