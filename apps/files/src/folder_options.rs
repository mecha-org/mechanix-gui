use mctk_core::layout::{Alignment, Direction};
use mctk_core::node;
use mctk_core::style::Styled;
use mctk_core::widgets::{Button, Div};
use mctk_core::{lay, msg, rect, size, size_pct, txt, Color};

use crate::gui::Message;

/// folder_modal_view
///
/// This function returns a node::Node that represents a modal dialog for folder options.
/// The modal includes buttons for pasting, deleting, renaming, and closing the modal.
/// The modal is styled with a dark background and dark border.
/// The buttons are styled with a white text color and a dark grey active color.
/// The modal is positioned at the top left corner of the screen.
pub fn folder_modal_view() -> node::Node {
    // Create the modal for folder options
    node!(
        Div::new().bg(Color::rgba(29., 29., 29., 1.)).border(
            Color::rgba(127., 127., 135., 1.),
            0.,
            (10., 10., 10., 10.)
        ),
        lay![
            size: [200, 200],
            direction: Direction::Column,
            position_type: Absolute,
            position: [10., 210., 0., 0.],
            cross_alignment: Alignment::Stretch,
            axis_alignment: Alignment::Stretch,
            // padding: [10., 10., 10., 10.],
        ]
    )
    .push(node!(
        Button::new(txt!("Paste"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("text_color", Color::WHITE)
            .style("font_size", 16.0)
            .style("line_height", 18.0)
            .on_click(Box::new(|| msg!(Message::Paste))),
        lay![margin: [0., 5., 5., 5.],]
    ))
    .push(node!(
        Button::new(txt!("Delete"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("text_color", Color::WHITE)
            .style("font_size", 16.0)
            .style("line_height", 18.0)
            .on_click(Box::new(|| msg!(Message::DeleteSelected))),
        lay![margin: [5., 5., 5., 5.],]
    ))
    .push(node!(
        Button::new(txt!("Rename"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("text_color", Color::WHITE)
            .style("font_size", 16.0)
            .style("line_height", 18.0)
            .on_click(Box::new(|| msg!(Message::RenameSelected))),
        lay![margin: [5., 5., 5., 5.],]
    ))
    .push(node!(
        Button::new(txt!("Close"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("text_color", Color::WHITE)
            .style("font_size", 16.0)
            .style("line_height", 18.0)
            .on_click(Box::new(|| msg!(Message::OpenFolderOptionsModal(false)))),
        lay![margin: [5., 5., 5., 5.],]
    ))
}
