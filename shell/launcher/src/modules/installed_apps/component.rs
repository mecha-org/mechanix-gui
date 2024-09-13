use std::hash::Hash;

use crate::gui;

use super::app::App as AppComponent;
use desktop_entries::DesktopEntry;
use mctk_core::{
    component::{Component, Message},
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct, state_component_impl,
    style::Styled,
    widgets::{Div, IconButton, IconType},
    Color,
};

#[derive(Debug)]
pub enum AppListMessage {
    AppClicked { app: DesktopEntry },
    AppLongClicked { app: DesktopEntry },
}

#[derive(Debug)]
pub struct AppList {
    pub apps: Vec<DesktopEntry>,
    pub disabled: bool,
}

impl AppList {
    pub fn new(apps: Vec<DesktopEntry>, disabled: bool) -> Self {
        Self { apps, disabled }
    }
}

impl Component for AppList {
    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.apps.len().hash(hasher);
    }

    fn update(&mut self, msg: Message) -> Vec<Message> {
        println!(
            "AppList was sent: {:?}",
            msg.downcast_ref::<AppListMessage>()
        );
        let mut bubble_msgs: Vec<Message> = vec![];

        bubble_msgs.push(msg);

        bubble_msgs
    }

    fn view(&self) -> Option<mctk_core::Node> {
        let mut apps_list_node = node!(
            Div::new(),
            // .scroll_y()
            // .style("bar_width", 0.)
            // .style("bar_color", Color::TRANSPARENT)
            // .style("bar_background_color", Color::TRANSPARENT),
            lay![
                // padding: [8, 16, 0, 0],
                size_pct: [100],
                axis_alignment: Alignment::Start,
                direction: Row,
                wrap: true,
            ]
        );

        for (i, app) in self.apps.clone().into_iter().enumerate() {
            apps_list_node = apps_list_node.push(
                node!(
                    AppComponent::new(app.clone(), self.disabled),
                    lay![margin: [0., 0., 0., if (i+1) % 4 == 0 { 0. } else {29.} ]]
                )
                .key(i as u64),
            );
        }

        Some(apps_list_node)
    }
}
