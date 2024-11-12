use std::fmt::Debug;

use mctk_core::{
    component::{Component, Message},
    event, lay,
    layout::{Alignment, Direction},
    node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, Image, Text},
    Color,
};

pub struct SettingsRowComponent {
    pub title: String,
    pub value: String,
    pub icon_1: String,
    pub icon_2: String,
    pub color: Color,
    pub on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl Debug for SettingsRowComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsRowComponent")
            .field("title", &self.title)
            .field("icon", &self.icon_1)
            .field("icon", &self.icon_2)
            .finish()
    }
}

impl Component for SettingsRowComponent {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn view(&self) -> Option<node::Node> {
        let title = self.title.clone();
        let value = self.value.clone();
        let icon_1 = self.icon_1.clone();
        let icon_2 = self.icon_2.clone();
        let color = self.color.clone();

        let text_node = node!(Text::new(txt!(title))
            .style("color", color)
            .style("size", 20.0)
            .style("line_height", 20.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Medium));

        let value_node = node!(Text::new(txt!(value))
            .style("color", Color::rgb(197.0, 197.0, 197.0))
            .style("size", 20.0)
            .style("line_height", 20.)
            .style("font", "Space Grotesk")
            .style("font_weight", FontWeight::Medium));

        Some(
            node!(
                Div::new(),
                lay![
                    padding: [10, 0, 10, 0],
                    size_pct: [100],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [70],
                        axis_alignment: Alignment::Start
                    ],
                )
                .push(node!(
                    Image::new(icon_1),
                    lay![
                        size: [24, 24],
                        margin: [0, 5],
                        padding: [0., 0., 0., 15.]
                    ]
                ))
                .push(text_node),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [30],
                        axis_alignment: Alignment::End
                    ]
                )
                .push(value_node)
                .push(node!(
                    Image::new(icon_2),
                    lay![
                        size: [24, 24],
                        margin: [0, 5],
                    ]
                )),
            ),
        )
    }
}
