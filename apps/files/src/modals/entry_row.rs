use std::path::PathBuf;
use std::sync::Arc;

use mctk_core::component::Component;
use mctk_core::layout::{Alignment, Dimension, Direction, Size};
use mctk_core::style::Styled;
use mctk_core::widgets::{self, Div, IconButton, IconType, Text};
use mctk_core::{event, Node};
use mctk_core::{lay, rect, size, size_pct, txt, Color};
use mctk_core::{msg, node};

use crate::gui::Message;

pub struct ClicableIconComponent {
    pub on_click: Option<Box<dyn Fn() -> Box<Message> + Send + Sync>>,
}

impl std::fmt::Debug for ClicableIconComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClicableIconComponent").finish()
    }
}

impl Component for ClicableIconComponent {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn container(&self) -> Option<Vec<usize>> {
        Some(vec![0])
    }

    fn view(&self) -> Option<Node> {
        let base = node!(
            Div::new(),
            lay![
                size_pct: [80, Auto],
                axis_alignment: Alignment::Start,
            ]
        );
        Some(base)
    }
}
pub struct EntryRow {
    pub is_file: bool,
    pub title: String,
    pub icon_1: String,
    pub icon_2: String,
    pub selected_entry: Option<Arc<PathBuf>>,
}

impl std::fmt::Debug for EntryRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntryRow")
            .field("title", &self.title)
            .field("icon_1", &self.icon_1)
            .field("icon_2", &self.icon_2)
            .finish()
    }
}
// self.disable_click

impl Component for EntryRow {
    fn view(&self) -> Option<node::Node> {
        let opt_pathbuf = self.selected_entry.clone();
        let file_icon_option = node!(
            Div::new(),
            lay![
                size_pct: [20, Auto],
                axis_alignment: Alignment::End,
                cross_alignment:Alignment::Center,
                padding: [0. , 0., 0., 10.]
            ]
        )
        .push(node!(
            IconButton::new(self.icon_2.clone())
                .on_click(Box::new(move || Box::new(Message::OpenModal(true)))) // Open modal when icon 2 is clicked
                .icon_type(IconType::Png)
                .style(
                    "size",
                    Size {
                        width: Dimension::Px(34.0),
                        height: Dimension::Px(34.0),
                    }
                )
                .style("background_color", Color::TRANSPARENT)
                .style("border_color", Color::TRANSPARENT)
                .style("active_color", Color::rgba(85., 85., 85., 0.50))
                .style("radius", 10.),
            lay![
                size: [52, 52],
                axis_alignment: Alignment::End,
                cross_alignment: Alignment::Center,
            ]
        ));
        let mut base_node = node!(
            Div::new(),
            lay![
                //padding: [10, 10, 10, 10],
                //size_pct: [100, Auto],
                size:[440,68],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(
            node!(ClicableIconComponent {
                on_click: Some(Box::new(move || {
                    if opt_pathbuf.is_none() {
                        return msg!(Message::GoBack);
                    } else {
                        let arc_path_buf = opt_pathbuf.clone().unwrap();
                        msg!(Message::SelectEntry((*arc_path_buf).clone()).clone())
                    }
                    // let arc_path_buf = opt_pathbuf.unwrap();

                    // msg!(Message::SelectEntry((*arc_path_buf).clone()).clone())
                })),
            })
            .push(node!(
                widgets::Image::new(self.icon_1.clone()),
                lay![
                    size: [28, 28],
                    margin:[0., 10., 0., 20.],
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, Auto],
                        direction: Direction::Column,
                        axis_alignment: Alignment::Stretch,
                    ]
                )
                .push(node!(
                    Text::new(txt!(truncate(self.title.clone(), 30)))
                        .style("color", Color::WHITE)
                        .style("font", "Inter")
                        .with_class("text-2xl leading-7 font-normal"),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            ),
        );

        // Show dot icon only on when it is a file
        if self.is_file {
            base_node = base_node.push(file_icon_option);
        }

        Some(base_node)
    }
}

pub fn truncate(s: String, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}
