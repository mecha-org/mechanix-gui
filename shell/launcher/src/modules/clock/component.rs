use mctk_core::layout::Direction;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component, txt, Color};
use mctk_core::{component::Component, lay, node, size_pct, widgets::Div, Node};
use std::hash::Hash;

#[derive(Debug)]
pub struct Clock {
    pub date: String,
    pub time: String,
}

impl Component for Clock {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.date.hash(hasher);
        self.time.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let date = self.date.clone();
        let time = self.time.clone();

        let time_node = node!(
            Text::new(txt!(time))
                .with_class("font-space-mono font-bold")
                .style("color", Color::rgb(230., 230., 230.))
                .style("size", 72.0),
            lay![]
        );

        let date_node = node!(
            Text::new(txt!(date)).with_class("text-white text-sm font-space-mono font-bold"),
            lay![
                size_pct: [100],
            ]
        );

        let mut clock_node = node!(
            Div::new(),
            lay![
                direction: Direction::Column
            ],
        );

        clock_node = clock_node.push(time_node);
        clock_node = clock_node.push(date_node);

        Some(clock_node)
    }
}
