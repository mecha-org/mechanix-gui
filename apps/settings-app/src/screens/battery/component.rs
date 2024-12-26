use mctk_core::component::{ComponentHasher, Message};
use mctk_core::event::{self, Event};
use mctk_core::layout::{Alignment, Direction};
use mctk_core::{component, msg, state_component_impl, Color, Point, Scale, AABB};
use mctk_core::{component::Component, lay, node, rect, size, size_pct, widgets::Div, Node};
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};

pub struct StatusIndicator {
    percentage: u8,
    on_change: Option<Box<dyn Fn(bool) -> Message + Send + Sync>>,
}

impl fmt::Debug for StatusIndicator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StatusIndicator")
            .field("active", &self.percentage)
            .finish()
    }
}

impl StatusIndicator {
    pub fn new(percentage: u8) -> Self {
        Self {
            percentage: percentage,
            on_change: None,
        }
    }

    pub fn on_change(mut self, change_fn: Box<dyn Fn(bool) -> Message + Send + Sync>) -> Self {
        self.on_change = Some(change_fn);
        self
    }
}

// #[state_component_impl(StatusIndicatorState)]
impl Component for StatusIndicator {
    fn view(&self) -> Option<Node> {
        let mut base = node!(
            Div::new().bg(Color::rgb(204., 230., 204.)).border(
                Color::TRANSPARENT,
                1.,
                (8., 8., 8., 8.)
            ),
            lay![
                size: [370, 40],
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
            ]
        );

        let status = node!(
            Div::new().bg(Color::rgb(61., 209., 61.)).border(
                Color::TRANSPARENT,
                1.,
                (8., 8., 8., 8.)
            ),
            lay![
                size_pct: [self.percentage, Auto],
            ]
        );

        if self.percentage.clone() != 0 {
            base = base.push(status);
        }

        Some(base)
    }
}
