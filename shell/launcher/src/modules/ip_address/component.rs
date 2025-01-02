use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component, txt, Color};
use mctk_core::{component::Component, node, Node};
use std::hash::Hash;

#[derive(Debug)]
pub struct IpAddress {
    pub ip_address: String,
}

impl Component for IpAddress {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.ip_address.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        Some(node!(Text::new(txt!(self.ip_address.clone()))
            .with_class("font-space-mono font-normal")
            .style("color", Color::rgb(201., 201., 201.))
            .style("size", 15.0)))
    }
}
