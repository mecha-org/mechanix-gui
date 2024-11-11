use crate::gui;
use crate::shared::h_divider::HDivider;
use crate::shared::slider::{Slider, SliderType};
use mctk_core::style::Styled;
use mctk_core::*;

#[derive(Debug)]
pub struct SoundScreen {}
impl component::Component for SoundScreen {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                // padding: [15.0, 10.0, 15.0, 10.0],
            ]
        );

        let mut c_node = node!(
            widgets::Div::new()
                // .scroll_y()
                .style("bar_width", 0.)
                .style("bar_color", Color::TRANSPARENT)
                .style("bar_background_color", Color::TRANSPARENT),
            lay![
                size_pct: [100, 80],
                axis_alignment: layout::Alignment::Stretch,
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 20.0, 0.0, 20.0]
            ],
        );

        //Title
        let mut header = node!(
            widgets::Div::new(),
            // Div::new().bg(Color::MID_GREY),
            lay![
                size_pct: [100, 15],
                axis_alignment: layout::Alignment::Stretch,
                // cross_alignment: Alignment::Center,
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [5.0, 0.0, 0.0, 0.0],
                margin: [25., 0., 0., 10.]
            ]
        );
        let header_text = node!(
            widgets::Text::new(txt!("Sound"))
                .style("font", "Space Grotesk")
                .style("size", 28.)
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("font_weight", style::FontWeight::Normal),
            lay![
                margin:[2.0, 5.0, 2.0, 5.0],
                size: size!(20.0, 50.0),
                axis_alignment: layout::Alignment::Stretch,
            ]
        );
        header = header.push(header_text);

        let mut footer = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100, 20],
                axis_alignment: layout::Alignment::End,
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                direction: layout::Direction::Column
            ]
        );
        footer = footer.push(node!(HDivider { size: 1. }));
        footer = footer.push(node!(
            widgets::Image::new("back_icon"),
            lay![
                size: [24, 24],
                direction: layout::Direction::Row,
                axis_alignment: layout::Alignment::Stretch,
            ]
        ));
        let slider_label = node!(
            widgets::Text::new(txt!("OUTPUT"))
                .style("color", Color::WHITE)
                .style("size", 15.0)
                .style("font", "SpaceMono-Bold")
                .style("font_weight", style::FontWeight::Normal),
            lay![
                margin: [45., 10., 0., 10.]
            ]
        );

        let slider = node!(
            Slider::new()
                .slider_type(SliderType::Line)
                .active_color(Color::rgb(226., 102., 0.))
                .on_slide(Box::new(|value| Box::new(())))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[10., 10., 0., 10.]]
        );
        c_node = c_node.push(header);
        c_node = c_node.push(slider_label);
        c_node = c_node.push(slider);

        main_node = main_node.push(c_node);
        base = base.push(main_node);
        base = base.push(footer);

        Some(base)
    }
}
