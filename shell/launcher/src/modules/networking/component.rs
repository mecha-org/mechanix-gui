use mctk_core::layout::Direction;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{Div, Text};
use mctk_core::{component, lay, size_pct, txt, Color};
use mctk_core::{component::Component, node, Node};
use std::hash::Hash;

#[derive(Debug)]
pub struct Networking {
    pub status: String,
}

impl Component for Networking {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.status.hash(hasher);
    }
    fn view(&self) -> Option<Node> {
        Some(
            node!(Div::new(), lay![direction: Direction::Column])
                .push(node!(Text::new(txt!("NET"))
                    .style("color", Color::WHITE)
                    .style("size", 15.0)
                    .style("font", "SpaceMono-Bold")
                    .style("font_weight", FontWeight::Bold)))
                .push(node!(
                    Text::new(txt!(self.status.clone()))
                        .style("color", Color::rgb(201., 201., 201.))
                        .style("size", 24.0)
                        .style("font", "SpaceMono-Bold")
                        .style("font_weight", FontWeight::Normal),
                    lay![size_pct: [100]]
                )),
        )
    }
}
