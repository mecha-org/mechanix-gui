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
    gui::{self, Message, PinKey},
};

pub struct Permission {
    pub message: String,
    pub pin_enabled: bool,
}

impl std::fmt::Debug for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Permission")
            .field("message", &self.message)
            .finish()
    }
}

impl Component for Permission {
    fn view(&self) -> Option<Node> {
        let icon = "lock_icon";
        let pin_enabled = self.pin_enabled;
        let message = self.message.clone();

        let icon_node = node!(
            Div::new(),
            lay![
                 size_pct: [100, Auto],
                 axis_alignment: Alignment::Center,
                 margin: [40, 0, 0, 0]
            ]
        )
        .push(
            node!(
                Div::new().bg(Color::rgb(44., 47., 54.)).border(
                    Color::TRANSPARENT,
                    0.,
                    (9.6, 9.6, 9.6, 9.6)
                ),
                lay![
                     size: [60, 60],
                     axis_alignment: Center,
                     cross_alignment: Center,
                ]
            )
            .push(node!(
                Svg::new(icon),
                lay![
                    size: [38, 38],
                ],
            )),
        );

        let message_node = node!(
            Div::new(),
            lay![
                 size_pct: [100, Auto],
                 axis_alignment: Alignment::Center,
                 margin: [26, 0, 0, 0]
            ]
        )
        .push(node!(
            Text::new(txt!(message))
                .style("color", Color::WHITE)
                .style("size", 16.0)
                .style("font_weight", FontWeight::Semibold), // .style("v_alignment", VerticalPosition::Center)
        ));

        // let action_node = node!(
        //     Div::new(),
        //     lay![
        //          size_pct: [100, Auto],
        //          axis_alignment: Alignment::Center,
        //          margin: [24, 0, 0, 0]
        //     ]
        // )
        // .push(node!(
        //     Text::new(txt!("Action: Power Off"))
        //         .style("color", Color::WHITE)
        //         .style("size", 16.0)
        //         .style("font_weight", FontWeight::Semibold), // .style("v_alignment", VerticalPosition::Center)
        // ));

        let footer = node!(
            Div::new().bg(Color::TRANSPARENT),
            lay![
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                size: [Auto, 84],
                // cross_alignment: Alignment::Center,
                padding: [14, 14, 14, 14],

            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, Auto]
                ]
            )
            .push(node!(
                IconButton::new("close_icon")
                    .on_click(Box::new(|| msg!(gui::Message::Close)))
                    .style("background_color", Color::rgb(32., 36., 49.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 13.),
                lay![
                    size: [56, 56],
                ]
            ))
            .push(node!(
                IconButton::new(if pin_enabled {
                    "next_icon"
                } else {
                    "submit_icon"
                })
                .on_click(if pin_enabled {
                    Box::new(|| msg!(gui::Message::Next))
                } else {
                    Box::new(|| msg!(gui::Message::Submit))
                })
                .style("background_color", Color::rgb(32., 36., 49.))
                .style("active_color", Color::rgba(255., 255., 255., 0.50))
                .style("padding", 8.)
                .style("radius", 13.),
                lay![
                    size: [56, 56],
                    position_type: Absolute,
                    position: [0.0, Auto, 0.0, 0.0],
                ]
            )),
        );

        Some(
            node!(
                Div::new(),
                lay![
                    size_pct: [100],
                    direction: Column,
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(icon_node)
            .push(message_node)
            // .push(action_node)
            .push(footer),
        )
    }
}
