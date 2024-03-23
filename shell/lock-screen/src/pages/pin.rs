use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::{HorizontalPosition, Styled},
    txt,
    widgets::{Button, Div, IconButton, Svg},
    Color, Node,
};

use crate::{components::pin_indicators::PinIndicators, gui::Message};

pub struct Pin {
    pub pin_length: usize,
}

impl std::fmt::Debug for Pin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Pin")
            .field("pin_length", &self.pin_length)
            .finish()
    }
}

impl Component for Pin {
    fn view(&self) -> Option<Node> {
        let pin_keys = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];

        let mut pin_buttons = node!(
            Div::new(),
            lay![
                wrap: true,
                size: [Auto],
                padding: [20, 40, 20, 40],
                axis_alignment: Alignment::Center,
            ],
        );

        for (i, pin_key) in pin_keys.into_iter().enumerate() {
            let pin_node: Node = node!(
                Button::new(txt!(pin_key))
                    .on_click(Box::new(|| msg!(Message::PinKeyClicked(
                        pin_key.to_string()
                    ))))
                    .style("h_alignment", HorizontalPosition::Center)
                    .style("radius", 12.)
                    .style("text_color", Color::WHITE)
                    .style("font_size", 32.)
                    .style("active_color", Color::rgb(15., 16., 21.))
                    .style("background_color", Color::rgb(21., 23., 29.)),
                lay![
                    size: [64, 64],
                    margin: [10],
                ],
            )
            .key(i as u64);
            pin_buttons = pin_buttons.push(pin_node);
        }

        Some(
            node!(
                Div::new().bg(Color::BLACK),
                lay![
                    size_pct: [100],
                    direction: Column,
                    //axis_alignment: Stretch,
                    cross_alignment: Stretch,
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, Auto],
                        direction: Direction::Column,
                        axis_alignment: Alignment::Center,
                        cross_alignment: Alignment::Center,
                        margin: [60, 20, 0, 28],
                        padding: [10, 0, 10, 0]
                    ],
                )
                .push(node!(PinIndicators {
                    pin_length: self.pin_length,
                }))
                .push(node!(
                    IconButton::new("backspace_icon")
                        .on_click(Box::new(|| msg!(Message::BackspaceClicked)))
                        .style("h_alignment", HorizontalPosition::Right)
                        .style("background_color", Color::TRANSPARENT)
                        .style("active_color", Color::TRANSPARENT),
                    lay![
                        size: [32, 32],
                        position_type: Absolute,
                        position: [0.0, Auto, Auto, 0.0]
                    ]
                )),
            )
            .push(pin_buttons)
            .push(
                node!(
                    Div::new(),
                    lay![
                        size: [Auto, 80]
                    ],
                )
                .push(node!(
                    IconButton::new("back_icon".to_string())
                        .on_click(Box::new(|| msg!(Message::BackClicked)))
                        // .style("h_alignment", HorizontalPosition::Center)
                        .style("radius", 12.)
                        .style("active_color", Color::rgb(15., 16., 21.))
                        .style("padding", 10.)
                        .style("background_color", Color::rgb(21., 23., 29.)),
                    lay![
                        size: [64, 64],
                        margin: [0., 20.],
                    ],
                )),
            ),
        )
    }
}
