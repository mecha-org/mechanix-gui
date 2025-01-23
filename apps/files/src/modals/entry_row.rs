use std::path::PathBuf;
use std::sync::Arc;

use mctk_core::component::Component;
use mctk_core::layout::{Alignment, Dimension, Direction, Size};
use mctk_core::style::Styled;
use mctk_core::widgets::{self, Div, HDivider, IconButton, IconType, Image, Text};
use mctk_core::{event, Node};
use mctk_core::{lay, rect, size, size_pct, txt, Color};
use mctk_core::{msg, node};

use crate::gui::Message;

pub struct ClicableIconComponent {}

impl std::fmt::Debug for ClicableIconComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClicableIconComponent").finish()
    }
}

impl Component for ClicableIconComponent {
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
    pub is_modal_open: bool,
}

#[derive(Debug)]
pub struct EntryIconText {
    pub icon_1: String,
    pub title: String,
    pub is_modal_open: bool,
    pub selected_entry: Option<Arc<PathBuf>>,
}

impl Component for EntryIconText {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if self.is_modal_open {
            return;
        }

        let opt_pathbuf = self.selected_entry.clone();
        if let Some(pathbuf) = opt_pathbuf {
            println!("path is:{:?}", &pathbuf);
            event.emit(msg!(Message::SelectEntry(pathbuf.to_path_buf())));
        } else {
            event.emit(msg!(Message::GoBack));
        }
    }

    fn view(&self) -> Option<Node> {
        let icon = node!(
            widgets::Image::new(self.icon_1.clone()),
            lay![
                size: [28, 28],
                margin:[0., 10., 0., 20.],
            ]
        );
        let text = node!(Text::new(txt!(truncate(self.title.clone(), 25)))
            .style("color", Color::WHITE)
            // .style("font", "Inter")
            .with_class("text-2xl leading-7 font-normal"),);
        let mut base = node!(
            Div::new(),
            lay![
                size_pct: [100],
                direction: Direction::Row,
            ]
        );
        base = base.push(icon);
        base = base.push(text);
        Some(base)
    }
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
        let mut row = node!(
            Div::new(),
            lay![size: [440, 68],
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center
            ]
        )
        .push(node!(EntryIconText {
            icon_1: self.icon_1.clone(),
            title: self.title.clone(),
            is_modal_open: self.is_modal_open,
            selected_entry: self.selected_entry.clone()
        },));

        if self.is_file {
            let file_name = self.title.clone();
            // println!("entry row is being called,");
            row = row.push(node!(
                IconButton::new(self.icon_2.clone())
                    .on_click(Box::new(move || Box::new(Message::OpenModal(
                        true,
                        file_name.to_string()
                    )))) // Open modal when icon 2 is clicked
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
                    .style("radius", 4.),
                lay![
                    size: [52, 52],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            ));
        }

        let divider = node!(HDivider {
            size: 0.5,
            color: Color::MID_GREY
        });

        let mut base = node!(
            Div::new(),
            lay![
            direction: Direction::Column,
            cross_alignment: Alignment::Stretch
            ]
        );
        base = base.push(row);
        base = base.push(divider);

        return Some(base);
    }
}

pub fn truncate(s: String, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}
