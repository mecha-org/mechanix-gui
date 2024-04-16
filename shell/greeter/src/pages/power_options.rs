use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction},
    node, rect, size, size_pct,
    style::{HorizontalPosition, Styled, VerticalPosition},
    txt,
    widgets::{Div, IconButton, Svg, Text},
    Color, Node,
    msg
};

use crate::gui::{Message, Routes};

#[derive(Debug)]
pub struct PowerOptions {}

impl Component for PowerOptions {


    fn view(&self) -> Option<Node> {
        let footer = node!(
            Div::new(),
            lay![
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [9, 18, 9, 18]
            ]
        )
        .push(node!(
            IconButton::new("close_icon")
                .on_click(Box::new(|| msg!(Message::ChangeRoute(
                    Routes::Users
                ))))
                .style("background_color", Color::rgb(21., 23., 29.))
                .style("active_color", Color::rgba(29., 23., 29., 0.5))
                .style("padding", 8.)
                .style("radius", 12.),
            lay![
                size: [60, 60],
            ]
        ));

        Some(
            node!(
                Div::new().bg(Color::BLACK),
                lay![
                    size_pct: [100],
                    // axis_alignment: Alignment::Center,
                    direction: Direction::Column,
                    cross_alignment: Stretch,
                    padding: [40, 20, 0, 20]
                ]
            )
            .push(
                node!(
                    Div::new().bg(Color::rgba(34., 37., 44., 0.7)).border(
                        Color::TRANSPARENT,
                        1.,
                        (8., 8., 8., 8.)
                    ),
                    lay![
                        padding: [16],
                        margin: [10, 0, 10, 0],
                        cross_alignment: Alignment::Center,
                    ]
                )
                .push(node!(
                    Svg::new("shutdown_icon"),
                    lay![
                        size: size!(32.0, 32.0),
                        margin: [0, 0, 0, 8]
                    ]
                ))
                .push(node!(Text::new(txt!("Shut down"))
                    .style("color", Color::rgb(197., 200., 207.))
                    .style("size", 22.0)
                    .style("h_alignment", HorizontalPosition::Left)
                    .style("v_alignment", VerticalPosition::Center)
                    .style("font", "SpaceGrotesk-Medium"))),
            )
            .push(
                node!(
                    Div::new().bg(Color::rgba(34., 37., 44., 0.7)).border(
                        Color::TRANSPARENT,
                        1.,
                        (8., 8., 8., 8.)
                    ),
                    lay![
                        padding: [16],
                        margin: [10, 0, 10, 0]
                    ]
                )
                .push(node!(
                    Svg::new("restart_icon"),
                    lay![
                        size: size!(32.0, 32.0),
                        margin: [0, 0, 0, 8]]
                ))
                .push(node!(Text::new(txt!("Restart"))
                    .style("color", Color::rgb(197., 200., 207.))
                    .style("size", 22.0)
                    .style("h_alignment", HorizontalPosition::Left)
                    .style("v_alignment", VerticalPosition::Center)
                    .style("font", "SpaceGrotesk-Medium"))),
            )
            .push(
                node!(
                    Div::new().bg(Color::rgba(34., 37., 44., 0.7)).border(
                        Color::TRANSPARENT,
                        1.,
                        (8., 8., 8., 8.)
                    ),
                    lay![
                        padding: [16],
                        margin: [10, 0, 10, 0]
                    ]
                )
                .push(node!(
                    Svg::new("sleep_icon"),
                    lay![
                        size: size!(32.0, 32.0), 
                        margin: [0, 0, 0, 8]]
                )).push(node!(Text::new(txt!("Sleep"))
                .style("color", Color::rgb(197., 200., 207.))
                .style("size", 22.0)
                .style("h_alignment", HorizontalPosition::Left)
                .style("v_alignment", VerticalPosition::Center)
                .style("font", "SpaceGrotesk-Medium"))),
            )
            .push(footer)
            ,
        )
    }
}
