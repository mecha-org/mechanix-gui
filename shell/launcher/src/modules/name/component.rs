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
            .with_class("text-white font-space-mono font-bold")
            .style("size", 15.0)))
    }
}
