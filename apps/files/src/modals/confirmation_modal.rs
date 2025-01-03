use mctk_core::layout::{Alignment, Direction};
use mctk_core::node;
use mctk_core::style::FontWeight;
use mctk_core::style::Styled;
use mctk_core::widgets::{Button, Div, Text, TextBox};
use mctk_core::{lay, msg, rect, size, size_pct, txt, Color};

use crate::gui::{FileManagerState, Message};

pub fn confirmation_modal_view(file_manager_state: &FileManagerState) -> node::Node {
    node!(
        Div::new().bg(Color::rgba(29., 29., 29., 1.)).border(
            Color::rgba(127., 127., 135., 1.),
            0.,
            (10., 10., 10., 10.)
        ),
        lay![
            size: [320, 160],
            direction: Direction::Column,
            position_type: Absolute,
            position: [80., 60., 0., 0.],
            cross_alignment: Alignment::Stretch,
            axis_alignment: Alignment::Stretch,
            padding: [15., 15.,  15., 10.]
        ]
    )
    .push(
        node!(
            Div::new(),
            lay![
                size_pct: [100, 42],
                cross_alignment: Alignment::Center,
                axis_alignment: Alignment::Center,
                direction: Direction::Column,
            ]
        )
        .push(node!(
            Text::new(txt!(file_manager_state.action_modal_title.clone())) // Use the action modal title from state
                .style("color", Color::WHITE)
                .style("size", 18.)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size: [Auto],
            ]
        )),
    )
    .push(
        // BUTTONS
        node!(
            Div::new(),
            lay![
                size_pct: [100, 68],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                // size_pct: [28, 100],
                axis_alignment: Alignment::Start,
            ]
        ))
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, 100],
                    axis_alignment: Alignment::Center,
                    cross_alignment:Alignment::Center,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                TextBox::new(Some("".to_string()))
                    .with_class("text-md border-1 bg-transparent")
                    .placeholder("Enter Name")
                    .on_change(Box::new(|s| msg!(Message::UpdateFolderName(s.to_string())))),
                lay![
                    size_pct: [80, 60],
                    margin: [0., 0., 10., 0.]
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, 50],
                        axis_alignment: Alignment::Stretch,
                        direction: Direction::Row,
                        margin: [0., 0., 10., 0.]
                    ]
                )
                .push(node!(
                    Button::new(txt!("Cancel"))
                        .style("text_color", Color::WHITE)
                        .style("background_color", Color::rgba(29., 29., 29., 1.))
                        .style("active_color", Color::rgba(68., 68., 68., 1.))
                        .style("font_size", 16.)
                        .style("line_height", 18.)
                        .style("radius", 8.)
                        .on_click(Box::new(move || {
                            msg!(Message::OpenActionModal(false)) // Close the action modal
                        })),
                    // .on_click(Box::new(move || {
                    //     msg!(Message::OpenModal(false)) // Close modal
                    // })),
                    lay![
                        size_pct: [48, 100],
                        padding: [0., 0., 0., 12.],
                        cross_alignment: Alignment::Start,
                        axis_alignment: Alignment::Start,
                    ]
                ))
                .push(node!(
                    Button::new(txt!("Confirm"))
                        .style("text_color", Color::BLACK)
                        .style("background_color", Color::WHITE)
                        .style("active_color", Color::rgba(194., 184., 184., 1.))
                        .style("font_size", 16.)
                        .style("line_height", 18.)
                        .style("radius", 8.)
                        .on_click(Box::new(move || {
                            msg!(Message::ConfirmAction) // Trigger confirm action
                        })),
                    lay![
                        size_pct: [48, 100],
                        padding: [0., 12., 0., 0.],
                        axis_alignment: Alignment::End,
                    ]
                )),
            ),
        ),
    )
}
