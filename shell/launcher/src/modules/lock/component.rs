use mctk_core::component;
use mctk_core::layout::Alignment;
use mctk_core::widgets::Image;
use mctk_core::{component::Component, lay, node, size, size_pct, widgets::Div, Node};

#[derive(Debug)]
pub struct Lock {}

impl Component for Lock {
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
                Image::new("lock_icon"),
                lay![
                    size: [28, 28],
                ],
            )),
        )
    }
}
