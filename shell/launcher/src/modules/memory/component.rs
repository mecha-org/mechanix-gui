use mctk_core::layout::Direction;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component, txt, Color};
use mctk_core::{
    component::Component, lay, node, rect, size, state_component_impl, widgets::Div, Node,
};

use std::hash::Hash;

use crate::utils::fill_grid_with_true;

const MEMORY_GRID_SIZE: (u8, u8) = (7, 10);

#[derive(Debug)]
struct MemoryState {
    grid: Vec<Vec<bool>>,
}

#[component(State = "MemoryState")]
#[derive(Debug, Default)]
pub struct Memory {
    pub used_memory: u64,
}

impl Memory {
    pub fn new(used_memory: u64) -> Self {
        Self {
            used_memory,
            ..Default::default()
        }
    }
}

#[state_component_impl(MemoryState)]
impl Component for Memory {
    fn init(&mut self) {
        let count_true =
            (self.used_memory * MEMORY_GRID_SIZE.0 as u64 * MEMORY_GRID_SIZE.1 as u64) / 100; // Desired count of true values
        let grid = fill_grid_with_true(
            MEMORY_GRID_SIZE.0 as usize,
            MEMORY_GRID_SIZE.1 as usize,
            count_true as usize,
        );

        self.state = Some(MemoryState { grid });
    }

    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.used_memory.hash(hasher);
    }

    fn new_props(&mut self) {
        let count_true =
            (self.used_memory * MEMORY_GRID_SIZE.0 as u64 * MEMORY_GRID_SIZE.1 as u64) / 100; // Desired count of true values
        let grid = fill_grid_with_true(
            MEMORY_GRID_SIZE.0 as usize,
            MEMORY_GRID_SIZE.1 as usize,
            count_true as usize,
        );
        self.state_mut().grid = grid;
    }

    fn view(&self) -> Option<Node> {
        let mut memory_blocks = node!(
            Div::new(),
            lay![
                direction: Direction::Column,
                margin: [ 14., 0., 0., 0. ]
            ]
        );

        for i in 0..MEMORY_GRID_SIZE.0 {
            let mut row = node!(
                Div::new(),
                lay![
                    margin: [0., 0., 4., 0.]
                ]
            )
            .key(i.into());
            for j in 0..MEMORY_GRID_SIZE.1 {
                let occupied = self.state_ref().grid[i as usize][j as usize];

                row = row.push(
                    node!(
                        MemoryBlock { occupied },
                        lay![
                            margin: [0., 0., 0., 4.]
                        ]
                    )
                    .key(j as u64),
                );
            }
            memory_blocks = memory_blocks.push(row);
        }

        Some(
            node!(Div::new(), lay![direction: Direction::Column])
                .push(node!(Text::new(txt!("MEMORY"))
                    .with_class("text-white font-space-mono font-bold")
                    .style("size", 15.0)))
                .push(memory_blocks),
        )
    }
}

#[derive(Debug)]
struct MemoryBlock {
    pub occupied: bool,
}

impl Component for MemoryBlock {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.occupied.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let occupied = self.occupied;

        let color = if occupied {
            Color::rgb(0., 85., 255.)
        } else {
            Color::rgba(217., 217., 217., 0.21)
        };

        Some(node!(
            Div::new().bg(color),
            lay![
                size: [8, 8]
            ]
        ))
    }
}
