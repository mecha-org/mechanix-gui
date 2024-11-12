use crate::shared::slider::{Slider, SliderType};
use crate::{components::*, tab_item_node};
use crate::{
    gui::{Message, Routes},
    shared::h_divider::HDivider,
};
use mctk_core::component::*;
use mctk_core::event::*;
use mctk_core::style::Styled;
use mctk_core::widgets::*;
use mctk_core::*;

#[derive(Debug)]
pub struct DisplayScreen {}
impl Component for DisplayScreen {
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
                size_pct: [100, 80],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0],
            ]
        );

        let slider = node!(
            Slider::new()
                .slider_type(SliderType::Box)
                .active_color(Color::rgb(15., 168., 255.))
                .on_slide(Box::new(|_| { Box::new(()) }))
                .col_spacing(7.75)
                .row_spacing(7.75)
                .col_width(4.),
            lay![size: [Auto, 45], margin:[10., 10., 45., 0.]]
        );

        let screen_off_time = tab_item_node!(
            [text_node("Screen Time")],
            [text_bold_node("30s"), icon_node("right_arrow_icon")]
        );
        main_node = main_node.push(header_node("Display"));
        main_node = main_node.push(text_node("BRIGHTNESS"));
        main_node = main_node.push(slider);
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(screen_off_time);
        main_node = main_node.push(node!(HDivider { size: 1. }));
        base = base.push(main_node);
        base = base.push(footer_node());
        Some(base)
    }
}
