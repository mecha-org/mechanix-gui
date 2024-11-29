use std::hash::Hash;

use mctk_core::{
    component::Component,
    lay,
    layout::{self, Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{Div, IconButton, Text},
    Color,
};

use crate::{
    gui::Message,
    modules::running_apps::{running_app::RunningApp, running_apps_carousel::Carousel},
    shared::{close_button::CloseButton, h_divider::HDivider},
};

#[derive(Debug, Default)]
pub struct AppSwitcher {
    pub running_apps: Vec<RunningApp>,
}

impl Component for AppSwitcher {
    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.running_apps.len().hash(hasher);
    }

    fn view(&self) -> Option<mctk_core::Node> {
        let mut start_node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                // size: [480, 434]
                padding: [10., 0., 0., 0.],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column
            ]
        );

        let mut running_apps_list_node = if self.running_apps.len() > 0 {
            node!(
                Carousel::new(),
                lay![
                    size: [Auto, 216]
                    direction: Row,
                    cross_alignment: Alignment::Center,
                    // axis_alignment: Alignment::End
                ],
            )
        } else {
            node!(
                Div::new(),
                lay![
                    size: [Auto, 302],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ]
            )
            .push(node!(
                Text::new(txt!("No apps running".to_string()))
                    .with_class("text-l font-space-grotesk font-normal")
                    .style("color", Color::rgb(113., 113., 113.)) // .style("v_alignment", VerticalPosition::Center)
            ))
        };

        for (i, app) in self.running_apps.clone().into_iter().enumerate() {
            running_apps_list_node = running_apps_list_node.push(
                node!(
                    Div::new(),
                    lay![
                        padding: [0., 24., 0., 24.]
                    ]
                )
                .key(i as u64)
                .push(node!(
                    app,
                    lay![
                        size: [180, 216],
                    ],
                )),
            );
        }

        start_node = start_node.push(node!(
            HDivider { size: 1. },
            lay! [ margin: [0., 20., 10.5, 20.] ]
        ));

        start_node = start_node.push(node!(
            Text::new(txt!("Running apps"))
                .with_class("text-3xl leading-7 font-space-grotesk font-normal")
                .style("color", Color::rgb(197., 197., 197.)),
            lay! [ margin: [0., 20., 0, 20.] ]
        ));

        start_node = start_node.push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size: [480, 302],
                    padding: [0., 0., 0., 0.]
                    direction: Direction::Column,
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(running_apps_list_node),
        );

        start_node = start_node.push(node!(
            HDivider { size: 1. },
            lay! [ margin: [0., 20., 0., 20.] ]
        ));

        start_node = start_node.push(
            node!(
                Div::new(),
                lay![
                    size: [Auto, 78],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                    padding: [0., 20., 0., 20.]
                ]
            )
            .push(node!(
                CloseButton::new().on_click(Box::new(|| msg!(Message::RunningAppsToggle {
                    show: false
                }))),
                lay![
                    size: [114, 50],
                ]
            )),
        );

        Some(start_node)
    }
}
