use mctk_core::layout::Direction;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component, txt, Color};
use mctk_core::{component::Component, lay, node, size_pct, widgets::Div, Node};
use std::hash::Hash;

use super::model::ClockModel;

#[derive(Debug)]
pub struct Clock {
    pub date_format: String,
    pub time_format: String,
}

impl Component for Clock {
    fn init(&mut self) {
        ClockModel::start_streaming();
    }

    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.date_format.hash(hasher);
        self.date_format.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let date = &ClockModel::date(&self.date_format);
        let time = &ClockModel::time(&self.time_format);
        let time_node = node!(
            Text::new(txt!(time.clone()))
                .with_class("font-space-mono font-bold")
                .style("color", Color::rgb(230., 230., 230.))
                .style("size", 72.0),
            lay![]
        );

        let date_node = node!(
            Text::new(txt!(date.clone()))
                .with_class("text-white text-sm font-space-mono font-bold"),
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
