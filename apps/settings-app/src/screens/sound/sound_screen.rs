use crate::components::{footer_node, header_node, text_node};
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

        let slider = node!(
            Slider::new()
                .slider_type(SliderType::Line)
                .active_color(Color::rgb(226., 102., 0.))
                .on_slide(Box::new(|value| Box::new(())))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[10., 10., 0., 10.]]
        );
        main_node = main_node.push(header_node("Sound"));
        main_node = main_node.push(text_node("OUTPUT"));
        main_node = main_node.push(slider);
        main_node = main_node.push(footer_node());
        base = base.push(main_node);

        Some(base)
    }
}
