use crate::gui::{FileManagerState, Message};
use mctk_core::layout::{Alignment, Direction};
use mctk_core::node;
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{Button, Div, Text};
use mctk_core::{lay, msg, rect, size, size_pct, txt, Color};

pub fn delete_modal_view(file_manager_state: &FileManagerState) -> node::Node {
    node!(
        Div::new().bg(Color::rgba(29., 29., 29., 1.)).border(
            Color::rgba(127., 127., 135., 1.),
            0.,
            (8., 8., 8., 8.)
        ),
        lay![
            size: [300, 140],
            direction: Direction::Column,
            position_type: Absolute,
            position: [140., 80., 0., 0.],
            cross_alignment: Alignment::Stretch,
            axis_alignment: Alignment::Stretch,
            padding: [15., 15., 15., 10.]

        ]
    )
    .push(
        node!(
            Div::new(),
            lay![
                size_pct: [100, 72],
                cross_alignment: Alignment::Start,
                axis_alignment: Alignment::Start,
            ]
        )
        .push(node!(
            Text::new(txt!(format!(
                "Delete {}?",
                file_manager_state.delete_item_name
            )))
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
                size_pct: [100, 28],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                size_pct: [28, 100]
                axis_alignment: Alignment::Start,
            ]
        ))
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [72, 100]
                    axis_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(
                Button::new(txt!("Cancel"))
                    .style("text_color", Color::WHITE)
                    .style("background_color", Color::rgba(68., 68., 68., 1.))
                    .style("active_color", Color::rgba(82., 81., 81., 1.))
                    .style("font_size", 16.)
                    .style("line_height", 18.)
                    .style("radius", 6.)
                    .on_click(Box::new(move || {
                        msg!(Message::OpenDeleteModal(false)) // Close modal
                    })),
                lay![
                    size_pct: [48, 100],
                    padding: [0., 0., 0., 8.],
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
                    .style("radius", 6.)
                    .on_click(Box::new(move || {
                        msg!(Message::ConfirmDelete) // Trigger confirm action
                    })),
                lay![
                    size_pct: [48, 100],
                    padding: [0., 12., 0., 0.],
                    axis_alignment: Alignment::End,
                ]
            )),
        ),
    )
}
