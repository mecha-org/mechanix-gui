use mctk_core::component::Component;
use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::Styled;
use mctk_core::widgets::{IconButton, IconType, TextBox};
use mctk_core::Color;
use mctk_core::{lay, msg, node, rect, size, size_pct, widgets::Div};

use crate::components::filter_app::FilterApp;
use crate::gui;
use crate::utils::desktop_entries::DesktopEntry;

#[derive(Debug)]
pub struct Search {
    pub apps: Vec<DesktopEntry>,
}

impl Component for Search {
    fn view(&self) -> Option<node::Node> {
        let mut filtered_apps = node!(
            Div::new()
                .scroll_y()
                .style("bar_width", 0.)
                .style("bar_color", Color::TRANSPARENT)
                .style("bar_background_color", Color::TRANSPARENT),
            lay![
                padding: [10, 20, 0, 0],
                direction: Direction::Column,
                size_pct: [100, 62],
                axis_alignment: Alignment::Start,
            ]
        );

        for (i, app) in self.apps.clone().into_iter().enumerate() {
            let cloned_app = app.clone();
            let mut app_component =
                FilterApp::new(cloned_app.name.clone()).on_click(Box::new(move || {
                    msg!(gui::Message::RunApp {
                        name: cloned_app.name.clone(),
                        exec: cloned_app.exec.clone()
                    })
                }));
            if let Some(path) = cloned_app.icon_path {
                let icon_type = match path.extension().and_then(|ext| ext.to_str()) {
                    Some("png") => IconType::Png,
                    Some("svg") => IconType::Svg,
                    _ => IconType::Png,
                };
                app_component = app_component.icon((app.name.clone(), icon_type))
            }

            filtered_apps = filtered_apps.push(node!(app_component).key(i as u64));
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
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(gui::Message::ChangeRoute {
                        route: gui::Routes::Apps
                    })))
                    .style("background_color", Color::rgb(42., 42., 44.))
                    .style("active_color", Color::rgba(255., 255., 255., 0.50))
                    .style("padding", 8.)
                    .style("radius", 12.),
                lay![
                    size: [60, 60],
                ]
            )),
        );

        Some(
            node!(
                Div::new().bg(Color::rgb(5., 7., 10.)),
                lay![
                    size_pct: [100]
                    direction: Direction::Column,
                    cross_alignment: Alignment::Stretch,
                    //axis_alignment: Alignment::Stretch,
                    // padding: [ 30, 0, 0, 0 ]
                ]
            )
            .push(node!(
                TextBox::new(Some("".to_string()))
                .style("background_color", Color::TRANSPARENT)
                .style("font_size", 32.)
                .style("text_color", Color::WHITE)
                .style("border_width", 0.)
                .style("cursor_color", Color::WHITE)
                .style("placeholder_color", Color::rgb(111., 107., 107.))
                    .placeholder("Search apps")
                    .on_change(Box::new(|s| msg!(gui::Message::SearchTextChanged(s.to_string()))))
                    ,
                [
                    size: [Auto],
                    margin: [20.]
                ]
            ))
            .push(filtered_apps)
            .push(footer),
        )
    }
}
