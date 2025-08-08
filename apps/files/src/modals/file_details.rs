use crate::gui::{FileManagerState, Message};
use chrono::{DateTime, Local};
use mctk_core::layout::{Alignment, Dimension, Direction, Size};
use mctk_core::node;
use mctk_core::style::FontWeight;
use mctk_core::style::Styled;
use mctk_core::widgets::HDivider;
use mctk_core::widgets::{Div, IconButton, IconType, Text};
use mctk_core::{lay, msg, rect, size, size_pct, txt, Color};

#[derive(Debug)]
struct FileDetails {
    file_name: String,
    file_size: String,
    last_modified: String,
    full_path: String,
}

// File viewer layout
pub fn file_details_view(s: &FileManagerState) -> node::Node {
    let file_name = s
        .selected_file
        .clone()
        .unwrap()
        .file_name()
        .clone()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let file_size = s
        .selected_file
        .clone()
        .unwrap()
        .metadata()
        .unwrap()
        .len()
        .to_string();

    let last_modified: DateTime<Local> = s
        .selected_file
        .clone()
        .unwrap()
        .metadata()
        .unwrap()
        .modified()
        .unwrap()
        .into();

    let full_path = s
        .selected_file
        .clone()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let last_modified = last_modified.format("%d %b %Y %I:%M%p").to_string();
    let file_details = FileDetails {
        file_name,
        file_size,
        last_modified,
        full_path,
    };

    let header = node!(
        Div::new().bg(Color::BLACK),
        lay![
            direction: Direction::Row,
            cross_alignment: Alignment::Center,
            axis_alignment: Alignment::Start,
            padding: [5., 20., 5., 20.],
        ]
    )
    .push(node!(
        IconButton::new("back_icon")
            .on_click(Box::new(|| msg!(Message::GoBack)))
            .icon_type(IconType::Png)
            .style(
                "size",
                Size {
                    width: Dimension::Px(32.0),
                    height: Dimension::Px(34.0),
                }
            )
            .style("background_color", Color::TRANSPARENT)
            .style("border_color", Color::TRANSPARENT)
            .style("active_color", Color::rgba(85., 85., 85., 0.50)),
        lay![margin: [5., 5., 5., 5.], size: [32, 34]]
    ))
    .push(node!(
        Text::new(txt!("Details"))
            .style("color", Color::WHITE)
            .style("size", 24.0)
            .style("line_height", 24.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Normal),
        lay![margin: [5., 20., 5., 5.]]
    ))
    .push(node!(
        IconButton::new("dots_icon") // Add the three-dots icon
            .on_click(Box::new(|| msg!(Message::OpenModal(true, "".to_string())))) // Open the options modal
            .icon_type(IconType::Png)
            .style(
                "size",
                Size {
                    width: Dimension::Px(30.0),
                    height: Dimension::Px(30.0),
                }
            )
            .style("background_color", Color::TRANSPARENT)
            .style("border_color", Color::TRANSPARENT)
            .style("active_color", Color::rgba(85., 85., 85., 0.50))
            .style("radius", 10.),
        lay![
            size: [42, 42],
            axis_alignment: Alignment::End,
            cross_alignment: Alignment::Center,
        ]
    ));

    let mut root = node!(
        Div::new().bg(Color::BLACK),
        lay![
            direction: Direction::Column,
            cross_alignment: Alignment::Stretch,
            axis_alignment: Alignment::Start,
            size_pct: [100, 100],
        ]
    );

    root = root.push(header);
    root = root.push(node!(HDivider {
        size: 1.,
        color: Color::MID_GREY,
    }));

    // Content area scrollable
    let mut content = node!(
        Div::new().bg(Color::TRANSPARENT),
        lay![
            direction: Direction::Column,
            cross_alignment: Alignment::Center,
            axis_alignment: Alignment::Start,
            padding: [20., 20., 20., 20.],
            size_pct: [100, 100],
        ]
    );

    content = content.push(node!(
        Text::new(txt!(format!("{:?}", file_details)))
            .style("color", Color::WHITE)
            .style("size", 30.0)
            .style("line_height", 24.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Normal),
        lay![margin: [5., 5., 5., 5.]]
    ));

    root
}
