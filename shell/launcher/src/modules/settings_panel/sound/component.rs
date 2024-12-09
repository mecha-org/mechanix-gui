use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{Div, SlideBar, Text, SlideBarType};
use mctk_core::{component, lay, msg, rect, size, size_pct, txt, Color};
use mctk_core::{component::Component, node, Node};
use std::hash::Hash;


use crate::gui;

#[derive(Debug)]
pub struct Sound {
    pub value: u8,
}

impl Component for Sound {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.value.hash(hasher);
    }
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new(),
                lay![direction: Direction::Column, cross_alignment:Alignment::Stretch, size_pct:[100, Auto]]
            )
            .push(node!(Text::new(txt!("SOUND"))
                .with_class("text-white font-space-mono font-normal")
                .style("size", 15.0)))
            .push(node!(
                SlideBar::new()
                .value(self.value)
                .slider_type(SlideBarType::Line)
                .active_color(Color::rgb(226., 102., 0.))
                .on_slide(Box::new(|value| msg!(gui::Message::SliderChanged(gui::SliderSettingsNames::Sound { value }))))
                .col_spacing(8.)
                .col_width(3.75)
                , 
                lay![size: [Auto, 45], margin:[10., 0., 0., 0.]]
            )),
        )
    }
}
