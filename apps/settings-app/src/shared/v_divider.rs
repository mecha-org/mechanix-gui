use mctk_core::{
    component::{self, Component},
    lay,
    layout::{Alignment, Direction},
    node, size, size_pct,
    widgets::Div,
    Color, Node,
};
use std::hash::Hash;

#[derive(Debug)]
pub struct VDivider {
    pub size: f32,
    pub color: Color,
}

impl Component for VDivider {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        (self.size as u32).hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new().border(self.color, self.size, (0., 0., 0., 0.)),
                lay![
                    direction: Direction::Column,
                    size_pct: [Auto, 100],
                    axis_alignment: Alignment::Stretch
                ]
            )
            .push(node!(
                Div::new(),
                lay![
                    size: [ 1, Auto ]
                ]
            )),
        )
    }
}
