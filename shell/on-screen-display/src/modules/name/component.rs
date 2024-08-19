use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component, txt, Color};
use mctk_core::{component::Component, node, Node};
use std::hash::Hash;

#[derive(Debug)]
pub struct MachineName {
    pub name: String,
}

impl Component for MachineName {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.name.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        Some(node!(Text::new(txt!(self.name.to_uppercase().clone()))
            .style("color", Color::WHITE)
            .style("size", 15.0)
            .style("font", "SpaceMono-Bold")
            .style("font_weight", FontWeight::Bold)))
    }
}
