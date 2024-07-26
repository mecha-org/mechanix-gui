use std::hash::Hash;

use crate::{components::app::App as AppComponent, gui};
use desktop_entries::DesktopEntry;
use mctk_core::{
    component::{Component, Message},
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::Styled,
    widgets::{Div, IconButton, IconType},
    Color,
};

#[derive(Debug)]
pub enum AppListMessage {
    Home,
}

#[derive(Debug)]
pub struct AppList {
    pub apps: Vec<DesktopEntry>,
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
        match msg.downcast_ref::<AppListMessage>() {
            Some(AppListMessage::Home) => {
                std::process::abort();
            }
            None => (),
        }
        match msg.downcast_ref::<gui::Message>() {
            Some(_) => bubble_msgs.push(msg),
            None => (),
        }

        bubble_msgs
    }

    fn view(&self) -> Option<mctk_core::Node> {
        let mut apps_list_node = node!(
            Div::new()
                .scroll_y()
                .style("bar_width", 0.)
                .style("bar_color", Color::TRANSPARENT)
                .style("bar_background_color", Color::TRANSPARENT),
            lay![
                padding: [8, 16, 0, 0],
                size_pct: [100, 81],
                axis_alignment: Alignment::Start,
                direction: Row,
                wrap: true,
            ]
        );

        for (i, app) in self.apps.clone().into_iter().enumerate() {
            let cloned_app = app.clone();
            let mut app_component =
                AppComponent::new(cloned_app.name.clone()).on_click(Box::new(move || {
                    msg!(gui::Message::RunApp {
                        name: cloned_app.name.clone(),
                        exec: cloned_app.exec.clone()
                    })
                }));

            if let Some(path) = app.icon_path {
                let icon_type = match path.extension().and_then(|ext| ext.to_str()) {
                    Some("png") => IconType::Png,
                    Some("svg") => IconType::Svg,
                    _ => IconType::Png,
                };
                app_component = app_component.icon((app.name.clone(), icon_type))
            }

            apps_list_node = apps_list_node.push(node!(app_component).key(i as u64));
        }

        let footer = node!(
            Div::new().bg(Color::rgba(5., 7., 10., 1.)),
            lay![
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                size: [Auto, 84],
                // cross_alignment: Alignment::Center,
                padding: [14, 14, 14, 14],

            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, Auto]
                ]
            )
            .push(node!(
                IconButton::new("home_icon")
                    .on_click(Box::new(|| msg!(AppListMessage::Home)))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                ]
            ))
            .push(node!(
                IconButton::new("search_icon")
                    .on_click(Box::new(|| msg!(gui::Message::ChangeRoute {
                        route: gui::Routes::Search
                    })))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                    position_type: Absolute,
                    position: [0.0, Auto, 0.0, 0.0],
                ]
            )),
        );

        let mut c_node = node!(
            Div::new().bg(Color::rgb(5., 7., 10.)),
            lay![
                size_pct: [100]
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                // axis_alignment: Alignment::Stretch,
                // padding: [ 30, 0, 0, 0 ]
            ]
        );

        c_node = c_node.push(apps_list_node);
        c_node = c_node.push(footer);

        Some(c_node)
    }
}
