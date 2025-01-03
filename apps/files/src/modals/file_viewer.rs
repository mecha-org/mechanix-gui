use mctk_core::layout::{Alignment, Dimension, Direction, Size};
use mctk_core::node;
use mctk_core::style::FontWeight;
use mctk_core::style::Styled;
use mctk_core::widgets::{Div, IconButton, IconType, Image, Text};
use mctk_core::widgets::{HDivider, Scrollable};
use mctk_core::{lay, msg, rect, size, size_pct, txt, Color};

use crate::gui::{FileManagerState, Message};

// File viewer layout
pub fn file_viewer_view(s: &FileManagerState) -> node::Node {
    let file_name = s
        .view_file
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

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
                    height: Dimension::Px(34.0)
                }
            )
            .style("background_color", Color::TRANSPARENT)
            .style("border_color", Color::TRANSPARENT)
            .style("active_color", Color::rgba(85., 85., 85., 0.50)),
        lay![margin:[5.,5.,5.,5.], size:[32,34]]
    ))
    .push(node!(
        Text::new(txt!(file_name))
            .style("color", Color::WHITE)
            .style("size", 24.0)
            .style("line_height", 24.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Normal),
        lay![margin:[5.,20.,5.,5.]]
    ));

    let mut root = node!(
        Div::new().bg(Color::BLACK),
        lay![
            direction: Direction::Column,
            cross_alignment: Alignment::Stretch,
            axis_alignment: Alignment::Start,
            size_pct:[100,100]
        ]
    );

    root = root.push(header);
    root = root.push(node!(HDivider {
        size: 1.,
        color: Color::MID_GREY
    }));

    // Content area scrollable
    let mut content = node!(
        Div::new().bg(Color::TRANSPARENT),
        lay![
            direction: Direction::Column,
            cross_alignment: Alignment::Center,
            axis_alignment: Alignment::Start,
            padding:[20.,20.,20.,20. ],
            size_pct:[100,100],
        ]
    );

    if s.file_is_image {
        if let Some(file) = &s.view_file {
            let file_path = file.to_string_lossy().to_string();
            let img_new = Image::new(file_path.clone());
            content = content.push(node!(
                Image::dynamic_load_from(img_new, Some(file_path)),
                lay![size:[200,200], margin:[10.,10.,10.,10.]]
            ));
        } else {
            content = content.push(node!(Text::new(txt!("No file selected"))
                .style("color", Color::WHITE)
                .style("size", 18.0)
                .style("line_height", 24.0)
                .style("font", "Space Grotesk")));
        }
    } else if s.file_is_pdf {
        content = content.push(node!(Text::new(txt!("PDF viewing is not implemented"))
            .style("color", Color::WHITE)
            .style("size", 18.0)
            .style("line_height", 24.0)
            .style("font", "Space Grotesk")));
    } else if let Some(content_str) = &s.file_content {
        // Scrollable text area
        let mut scroll = node!(
            Scrollable::new(size!(440, 320)),
            lay![
                size: [440, 320],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        scroll = scroll.push(node!(
            Text::new(txt!(content_str.clone()))
                .style("color", Color::WHITE)
                .style("size", 14.0)
                .style("line_height", 20.0)
                .style("font", "Space Grotesk"),
            lay![margin:[5.,5.,5.,5.]]
        ));

        content = content.push(scroll);
    } else if s.file_no_preview {
        content = content.push(node!(Text::new(txt!(
            "No preview available for this file."
        ))
        .style("color", Color::WHITE)
        .style("size", 18.0)
        .style("line_height", 24.0)
        .style("font", "Space Grotesk")));
    } else {
        content = content.push(node!(Text::new(txt!("Loading or no file selected."))
            .style("color", Color::WHITE)
            .style("size", 18.0)
            .style("line_height", 24.0)
            .style("font", "Space Grotesk")));
    }

    root = root.push(content);
    root
}
