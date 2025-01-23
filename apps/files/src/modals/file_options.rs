use crate::gui::Message;
use mctk_core::layout::{Alignment, Direction};
use mctk_core::node;
use mctk_core::style::Styled;
use mctk_core::widgets::{Button, Div};
use mctk_core::{lay, msg, rect, size, size_pct, txt, Color};

pub fn file_options_view() -> node::Node {
    node!(
        Div::new().bg(Color::rgba(14., 14., 14., 1.)), // border
        lay![
            size: [200, 200],
            direction: Direction::Column,
            position_type: Absolute,
            position:[120., 190.,0., 0],
            cross_alignment: Alignment::Center,
            axis_alignment:Alignment::Center,
            // margin:[0., 10., 0., 0.],
            // padding: [20., 20., 20., 20.],
        ]
    )
    .push(node!(
        Button::new(txt!("Rename"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("text_color", Color::WHITE)
            .style("font_size", 16.0)
            .style("line_height", 18.0)
            .on_click(Box::new(|| msg!(Message::RenameSelected))),
        lay![margin: [5., 5., 5., 5.], size_pct: [100, 20]]
    ))
    .push(node!(
        Button::new(txt!("Copy"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("font_size", 16.0)
            .style("line_height", 18.0)
            .style("text_color", Color::WHITE)
            .on_click(Box::new(|| msg!(Message::CopySelected))),
        lay![margin: [5., 5., 5., 5.], size_pct: [100, 20]]
    ))
    .push(node!(
        Button::new(txt!("Delete"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("font_size", 15.0)
            .style("line_height", 24.0)
            .style("text_color", Color::WHITE)
            .on_click(Box::new(|| msg!(Message::DeleteSelected))),
        lay![margin: [5., 5., 5., 5.], size_pct: [100,20]]
    ))
    .push(node!(
        Button::new(txt!("Paste"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("font_size", 15.0)
            .style("line_height", 24.0)
            .style("text_color", Color::WHITE)
            .on_click(Box::new(|| msg!(Message::Paste))),
        lay![margin: [5., 5., 5., 5.], size_pct: [100, 20]]
    ))
    .push(node!(
        Button::new(txt!("Close"))
            .style("background_color", Color::TRANSPARENT)
            .style("active_color", Color::MID_GREY)
            .style("font_size", 16.0)
            .style("line_height", 18.0)
            .style("text_color", Color::WHITE)
            .on_click(Box::new(|| msg!(Message::OpenModal(false, "".to_string())))),
        lay![margin: [5., 5., 5., 5.], size_pct: [100, 20]]
    ))
}
