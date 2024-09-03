use mctk_core::component;
use mctk_core::layout::Alignment;
use mctk_core::widgets::Image;
use mctk_core::{component::Component, lay, node, size, size_pct, widgets::Div, Node};
use std::hash::Hash;

use crate::types::WirelessStatus;

#[derive(Debug)]
pub struct Wireless {
    pub wireless_status: WirelessStatus,
}

impl Component for Wireless {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.wireless_status.hash(hasher);
    }

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
                Image::new(format!("sm{:?}", self.wireless_status.to_string())),
                lay![
                    size: [28, 28],
                ],
            )),
        )
    }
}
