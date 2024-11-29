use std::fmt::Debug;

use desktop_entries::DesktopEntry;
use mctk_core::{
    component::{Component, Message},
    event, lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, IconType, Image, Svg, Text},
    Color,
};

use crate::{gui, shared::h_divider::HDivider};

#[derive(Debug, Clone)]
pub enum AppInfoMessage {
    Launch { app: DesktopEntry },
    CheckSystemUsage { app: DesktopEntry },
    Delete { app: DesktopEntry },
}

#[derive(Debug)]
pub struct AppInfo {
    pub app: DesktopEntry,
}

impl Component for AppInfo {
    fn update(&mut self, msg: Message) -> Vec<Message> {
        let mut bubble_events = vec![];

        bubble_events.push(msg);

        bubble_events
    }

    fn view(&self) -> Option<mctk_core::Node> {
        let name = self.app.name.clone();
        let icon_path = self.app.icon_path.clone();

        let mut start_node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column,
                padding: [0., 20., 20., 20.],
            ]
        );

        let mut row_1 = node!(
            Div::new(),
            lay![
                size: [Auto, 266],
                direction: Direction::Column,
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Center
            ]
        );

        let mut icon_box = node!(
            Div::new().border(Color::WHITE, 1.8, (0., 0., 0., 0.)),
            lay![
                 size: [120, 120],
                 axis_alignment: Alignment::Center,
                 cross_alignment: Alignment::Center,
                 padding: [21.]
                 margin: [0., 0., 20., 0.]
            ]
        );

        if let Some(path) = icon_path {
            match path.extension().and_then(|ext| ext.to_str()) {
                Some("png") => {
                    icon_box = icon_box.push(node!(
                        Image::new(name.clone()),
                        lay![
                            size_pct: [100],
                        ],
                    ));
                }
                Some("svg") => {
                    icon_box = icon_box.push(node!(
                        Svg::new(name.clone()),
                        lay![
                            size_pct: [100],
                        ],
                    ));
                }
                _ => (),
            };
        }

        row_1 = row_1.push(icon_box);

        row_1 = row_1.push(node!(
            Text::new(txt!(name))
                .with_class("font-space-grotesk font-normal text-3xl leading-7")
                .style("color", Color::rgb(197., 197., 197.)),
            lay! [ margin: [0., 20., 0, 20.] ]
        ));

        start_node = start_node.push(row_1);

        let app = self.app.clone();
        start_node = start_node.push(node!(
            AppOptions {
                title: "Launch".to_string(),
                icon: "launch_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(AppInfoMessage::Launch {
                    app: app.clone()
                }))),
            },
            lay![]
        ));

        start_node = start_node.push(node!(HDivider { size: 1. }, lay! [ margin: [14., 0.] ]));

        let app = self.app.clone();
        start_node = start_node.push(node!(
            AppOptions {
                title: "System usage".to_string(),
                icon: "settings_icon".to_string(),
                color: Color::WHITE,
                on_click: Some(Box::new(move || msg!(AppInfoMessage::CheckSystemUsage {
                    app: app.clone()
                }))),
            },
            lay![]
        ));

        start_node = start_node.push(node!(HDivider { size: 1. }, lay! [ margin: [14., 0.] ]));

        let app = self.app.clone();
        start_node = start_node.push(node!(
            AppOptions {
                title: "Delete app".to_string(),
                icon: "delete_icon".to_string(),
                color: Color::rgb(252., 64., 85.),
                on_click: Some(Box::new(move || msg!(AppInfoMessage::Delete {
                    app: app.clone()
                }))),
            },
            lay![]
        ));

        Some(start_node)
    }
}

struct AppOptions {
    pub title: String,
    pub icon: String,
    pub color: Color,
    pub on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl Debug for AppOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppOptions")
            .field("title", &self.title)
            .field("icon", &self.icon)
            .finish()
    }
}

impl Component for AppOptions {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn view(&self) -> Option<node::Node> {
        let title = self.title.clone();
        let icon = self.icon.clone();
        let color = self.color.clone();

        Some(
            node!(
                Div::new(),
                lay![
                    size_pct: [100],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch
                ]
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [50],
                        axis_alignment: Alignment::Start
                    ]
                )
                .push(node!(Text::new(txt!(title))
                    .with_class("text-l font-space-grotesk leading-5 font-bold")
                    .style("color", color))),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [50],
                        axis_alignment: Alignment::End
                    ]
                )
                .push(node!(
                    Image::new(icon),
                    lay![
                        size: [24, 24],
                    ]
                )),
            ),
        )
    }
}
