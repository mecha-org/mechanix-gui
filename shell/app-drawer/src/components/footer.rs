use std::{any::Any, collections::HashMap};

use mctk_core::{
    component::Component,
    lay,
    layout::Alignment,
    msg, node, rect, size, size_pct,
    style::Styled,
    widgets::{Div, IconButton},
    Color,
};

use crate::gui::{Message, Routes};

#[derive(Clone)]
pub struct Footer {
    pub buttons: Vec<(String, Message)>,
}
impl std::fmt::Debug for Footer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Footer")
            .field("buttons", &self.buttons.len())
            .finish()
    }
}

impl Component for Footer {
    fn view(&self) -> Option<mctk_core::Node> {
        let mut footer = node!(
            Div::new().bg(Color::rgba(5., 7., 10., 1.)),
            lay![
                // position_type: Absolute,
                // position: [Auto, 0.0, 0.0, 0.0],
                size_pct: [100, Auto],
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                padding: [14, 14, 14, 14],

            ]
        );

        let mut buttons = node!(
            Div::new(),
            lay![
                size: [480, 84]
            ]
        );

        for (icon, message) in self.buttons.clone().into_iter() {
            buttons = buttons.push(node!(
                IconButton::new(icon)
                    .on_click(Box::new(move || msg!(message.clone())))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                ]
            ))
        }

        footer = footer.push(buttons);

        Some(footer)
    }
}
