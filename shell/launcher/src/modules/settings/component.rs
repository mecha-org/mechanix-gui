use mctk_core::layout::Alignment;
use mctk_core::style::Styled;
use mctk_core::widgets::{IconButton, IconType, Image};
use mctk_core::{component::Component, lay, node, size, size_pct, widgets::Div, Node};
use mctk_core::{msg, Color};

use crate::gui::Message;

#[derive(Debug)]
pub struct Settings {}

impl Component for Settings {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                Div::new()
                ,
                [
                    size_pct: [100],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ],
            )
            .push(node!(
                IconButton::new("settings_icon")
                    .icon_type(IconType::Png)
                    .on_click(Box::new(|| msg!(Message::AppOpen {
                        app_id: "mechanix-settings".to_string(),
                        layer: None
                    })))
                    .style("active_color", Color::TRANSPARENT)
                    .style("size", size!(36, 36))
                    .with_class("bg-transparent border-0"),
                lay![
                    size: [36, 36],
                ],
            )),
        )
    }
}
