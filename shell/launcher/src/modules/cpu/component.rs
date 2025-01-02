use mctk_core::layout::Direction;
use mctk_core::renderables::{curve::InstanceBuilder as CurveInstanceBuilder, Curve, Renderable};
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component, txt, Color, Point, AABB};
use mctk_core::{component::Component, lay, node, size, size_pct, widgets::Div, Node};
use std::collections::VecDeque;
use std::hash::Hash;

pub const GRID_SIZE: (u8, u8) = (5, 5);
pub const GRID_MAX_VAL: u32 = 100;

#[derive(Debug)]
pub struct CPU {
    pub usages: VecDeque<u8>,
}

impl Component for CPU {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new(),
                lay![
                    direction: Direction::Column,

                ]
            )
            .push(node!(Text::new(txt!("CPU"))
                .with_class("text-white font-space-mono font-bold")
                .style("size", 15.0),))
            .push(node!(
                CPUCurve {
                    usages: self.usages.clone()
                },
                lay![size: [122, 99]]
            )),
        )
    }
}

#[derive(Debug)]
struct CPUCurve {
    pub usages: VecDeque<u8>,
}

impl Component for CPUCurve {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.usages.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        Some(node!(
            Div::new(),
            lay![
                size_pct: [100]
            ]
        ))
    }
    fn render(&mut self, context: component::RenderContext) -> Option<Vec<Renderable>> {
        let width = context.aabb.width();
        let height = context.aabb.height();
        let AABB { pos, .. } = context.aabb;
        let cell_width = width / GRID_SIZE.0 as f32;
        let cell_height = height / GRID_SIZE.1 as f32;

        let anchors: Vec<Point> = self
            .usages
            .clone()
            .iter()
            .enumerate()
            .map(|(i, u)| {
                (
                    (GRID_SIZE.0 as f32 - i as f32),
                    (*u as f32 / (GRID_MAX_VAL / GRID_SIZE.1 as u32) as f32),
                )
            })
            .map(|(x, y)| (x, y))
            .map(|(x, y)| Point::new(x * cell_width + pos.x, height - y * cell_height + pos.y))
            .collect();

        let curve = CurveInstanceBuilder::default()
            .anchors(anchors.clone())
            .anchor_color(Color::WHITE)
            .anchor_width(3.)
            .width(1.5)
            .build()
            .unwrap();

        let mut rs = vec![];
        rs.push(Renderable::Curve(Curve::from_instance_data(curve)));

        Some(rs)
    }
}
