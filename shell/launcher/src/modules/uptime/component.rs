use mctk_core::layout::Direction;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{Div, Text};
use mctk_core::{component, lay, size_pct, txt, Color};
use mctk_core::{component::Component, node, Node};
use std::hash::Hash;

#[derive(Debug)]
pub struct Uptime {
    pub time: String,
}

impl Component for Uptime {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.time.hash(hasher);
    }
    fn view(&self) -> Option<Node> {
        Some(
            node!(Div::new(), lay![direction: Direction::Column])
                .push(node!(Text::new(txt!("UPTIME"))
                    .with_class("text-white font-space-mono font-bold")
                    .style("size", 15.0)))
                .push(node!(
                    Text::new(txt!(self.time.clone()))
                        .with_class("font-space-mono font-normal text-2xl")
                        .style("color", Color::rgb(201., 201., 201.)),
                    lay![size_pct: [100]]
                )),
        )
    }
}
