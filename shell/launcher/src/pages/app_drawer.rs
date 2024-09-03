use std::hash::{Hash, Hasher};

use command::spawn_command;
use desktop_entries::DesktopEntry;
use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::Styled,
    widgets::{Div, Image, TextBox},
    Color, Point,
};

use crate::{
    gui::{self, Message, Swipe, SwipeDirection, SwipeState},
    modules::installed_apps::{
        app,
        component::{AppList, AppListMessage},
    },
    pages::app_info::AppInfoMessage,
    shared::h_divider::HDivider,
};
use mctk_core::{component, state_component_impl};

use super::app_info::AppInfo;

#[derive(Debug, Default)]
pub struct AppDrawerState {
    pub app_info_app: Option<DesktopEntry>,
}

#[component(State = "AppDrawerState", Internal)]
#[derive(Debug)]
pub struct AppDrawer {
    pub apps: Vec<DesktopEntry>,
    pub swipe: i32,
}

impl AppDrawer {
    pub fn new(apps: Vec<DesktopEntry>, swipe: i32) -> Self {
        Self {
            apps,
            swipe,
            state: Some(AppDrawerState::default()),
            dirty: false,
        }
    }

    fn handle_on_drag(&self, delta: Point) -> Option<mctk_core::component::Message> {
        println!("AppDrawer delta.y {:?}", delta.y);
        if delta.y > 20. {
            let swipe = Swipe {
                dy: delta.y as i32,
                min_dy: 0,
                max_dy: 480,
                threshold_dy: 0,
                direction: SwipeDirection::Down,
                state: SwipeState::UserSwiping,
                is_closer: true,
                ..Default::default()
            };

            return Some(msg!(Message::Swipe { swipe }));
        }

        None
    }
}

#[state_component_impl(AppDrawerState)]
impl Component for AppDrawer {
    fn props_hash(&self, hasher: &mut component::ComponentHasher) {
        self.swipe.hash(hasher);
    }

    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        self.state_ref().app_info_app.is_some().hash(hasher);
    }

    fn update(&mut self, msg: mctk_core::component::Message) -> Vec<mctk_core::component::Message> {
        let mut bubble_msgs: Vec<mctk_core::component::Message> = vec![];
        if let Some(message) = msg.downcast_ref::<AppListMessage>() {
            match message {
                AppListMessage::AppClicked { app } => {
                    if !app.exec.is_empty() {
                        let mut args: Vec<String> = vec!["-c".to_string()];
                        args.push(app.exec.clone());
                        let _ = spawn_command("sh".to_string(), args);
                        let swipe = Swipe {
                            dy: 0 as i32,
                            min_dy: 0,
                            max_dy: 480,
                            threshold_dy: 0,
                            direction: SwipeDirection::Down,
                            state: SwipeState::CompletingSwipe,
                            is_closer: true,
                            ..Default::default()
                        };
                        bubble_msgs.push(msg!(gui::Message::Swipe { swipe }));
                    }
                }
                AppListMessage::AppLongClicked { app } => {
                    println!("App Drawer app long clicked");
                    self.state_mut().app_info_app = Some(app.clone());
                }
            }
        }

        if let Some(message) = msg.downcast_ref::<AppInfoMessage>() {
            match message {
                AppInfoMessage::Launch { app } => {
                    if !app.exec.is_empty() {
                        let mut args: Vec<String> = vec!["-c".to_string()];
                        args.push(app.exec.clone());
                        let _ = spawn_command("sh".to_string(), args);
                    }
                    self.state_mut().app_info_app = None;
                }
                AppInfoMessage::CheckSystemUsage { app } => {
                    println!("Checking system usage for app: {}", app.name);
                }
                AppInfoMessage::Delete { app } => {
                    println!("Deleting app: {}", app.name);
                }
            }
        }
        bubble_msgs
    }

    fn view(&self) -> Option<mctk_core::Node> {
        let app_info_app = self.state_ref().app_info_app.clone();
        let app_info_app_exists = app_info_app.is_some();

        let mut start_node = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column,
                position_type: Absolute,
                position: [self.swipe, 0., 0., 0.],
            ]
        )
        .key(self.swipe as u64);

        if let Some(app) = app_info_app {
            start_node = start_node.push(node!(
                AppInfo { app },
                lay![
                    size_pct: [100],
                    position_type: mctk_core::layout::PositionType::Absolute,
                    position: [0., 0., 0., 0.],
                ]
            ));
        }

        start_node = start_node.push(node!(
            HDivider { size: 1. },
            lay! [ margin: [10., 20., 8., 20.] ]
        ));

        start_node = start_node.push(
            node!(
                Div::new(),
                lay![
                    margin: [0., 20., 0., 20.]
                ]
            )
            .push(node!(
                Image::new("search_icon"),
                lay![
                    size: [24, 24],
                    margin: [0., 0., 0., 10.]
                ],
            ))
            .push(node!(
                TextBox::new(Some("".to_string()))
                .style("background_color", Color::TRANSPARENT)
                .style("font_size", 18.)
                .style("text_color", Color::WHITE)
                .style("border_width", 0.)
                .style("cursor_color", Color::WHITE)
                .style("placeholder_color",  Color::rgb(168., 168., 168.))
                    .placeholder(if !app_info_app_exists {"Search" } else { "" } )
                    .on_change(Box::new(|s| msg!(gui::Message::SearchTextChanged(s.to_string()))))
                    ,
                [
                    size: [420, Auto],
                ]
            )),
        );

        start_node = start_node.push(node!(
            HDivider { size: 1. },
            lay! [ margin: [8., 20., 20., 20.] ]
        ));

        start_node = start_node.push(node!(
            AppList::new(self.apps.clone()),
            lay![
                size: [Auto],
                margin: [0., 20., 0., 20.]
            ]
        ));

        Some(start_node)
    }

    fn on_drag_start(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragStart>) {
        event.stop_bubbling();
    }

    fn on_touch_drag_start(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragStart>,
    ) {
        event.stop_bubbling();
    }

    fn on_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Drag>) {
        event.stop_bubbling();
        println!("AppDrawer::drag()");
        if let Some(msg) = self.handle_on_drag(event.logical_delta()) {
            event.emit(msg);
        };
    }

    fn on_touch_drag(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::TouchDrag>) {
        event.stop_bubbling();
        println!("AppDrawer::drag()");
        if let Some(msg) = self.handle_on_drag(event.logical_delta()) {
            event.emit(msg);
        };
    }

    fn on_drag_end(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragEnd>) {
        event.emit(msg!(Message::SwipeEnd));
    }

    fn on_touch_drag_end(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragEnd>,
    ) {
        event.emit(msg!(Message::SwipeEnd));
    }
}
