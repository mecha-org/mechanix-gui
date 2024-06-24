use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::{FontWeight, HorizontalPosition, Styled},
    txt,
    widgets::{Button, Div, IconButton, Svg, Text},
    Color, Node,
};

use crate::{
    components::pin_indicators::PinIndicators,
    gui::{Message, PinKey},
};

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
        let pin_keys = [
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "home",
            "9",
            "0",
            "backspace",
        ];

        let mut pin_buttons = node!(
            Div::new(),
            lay![
                wrap: true,
                size_pct: [100, Auto],
                axis_alignment: Alignment::Center,
            ],
        );

        for (i, pin_key) in pin_keys.into_iter().enumerate() {
            let pin_node: Node = (if pin_key == "home" {
                node!(
                    IconButton::new("close_icon")
                        .on_click(Box::new(|| msg!(Message::PinKeyClicked(PinKey::Close))))
                        .style("h_alignment", HorizontalPosition::Center)
                        .style("radius", 9.6)
                        .style("padding", 8.)
                        .style("active_color", Color::rgba(255., 255., 255., 0.50))
                        .style("background_color", Color::rgb(44., 47., 54.)),
                    lay![
                        size: [56, 56],
                        margin: [14],
                    ],
                )
            } else if pin_key == "backspace" {
                node!(
                    IconButton::new("backspace_icon")
                        .on_click(Box::new(|| msg!(Message::PinKeyClicked(PinKey::Backspace))))
                        .style("h_alignment", HorizontalPosition::Center)
                        .style("radius", 9.6)
                        .style("padding", 8.)
                        .style("active_color", Color::rgba(255., 255., 255., 0.50))
                        .style("background_color", Color::rgb(44., 47., 54.)),
                    lay![
                        size: [56, 56],
                        margin: [14],
                    ],
                )
            } else {
                node!(
                    Button::new(txt!(pin_key))
                        .on_click(Box::new(|| msg!(Message::PinKeyClicked(PinKey::Text {
                            key: pin_key.to_string()
                        }))))
                        .style("h_alignment", HorizontalPosition::Center)
                        .style("radius", 9.6)
                        .style("text_color", Color::WHITE)
                        .style("font_size", 29.)
                        .style("active_color", Color::rgba(255., 255., 255., 0.50))
                        .style("background_color", Color::rgb(44., 47., 54.)),
                    lay![
                        size: [56, 56],
                        margin: [14],
                    ],
                )
            })
            .key(i as u64);
            pin_buttons = pin_buttons.push(pin_node);
        }

        let mut pin_indicators_node = node!(
            PinIndicators {
                pin_length: self.pin_length,
            },
            lay![
                margin: [22, 0, 24, 0]
            ]
        );

        if self.pin_length == 0 {
            pin_indicators_node = node!(
                Div::new(),
                lay![
                     size_pct: [100, Auto],
                     axis_alignment: Alignment::Center,
                     margin: [22, 0, 24, 0]
                ]
            )
            .push(node!(
                Text::new(txt!("Enter pin to confirm"))
                    .style("color", Color::rgba(228., 231., 238., 0.40))
                    .style("size", 19.0)
                    .style("font_weight", FontWeight::Normal), // .style("v_alignment", VerticalPosition::Center)
            ));
        }

        Some(
            node!(
                Div::new(),
                lay![
                    size_pct: [100],
                    direction: Column,
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(pin_indicators_node)
            .push(pin_buttons),
        )
    }
}
