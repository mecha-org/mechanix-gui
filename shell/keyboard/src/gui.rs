use crate::settings::{self, KeyboardSettings};
use crate::{action, AppMessage};
use mctk_core::component::RootComponent;
use mctk_core::event::Event;
use mctk_core::layout::{Alignment, Dimension, Size};
use mctk_core::reexports::femtovg::{Align, CompositeOperation};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::style::{FontWeight, HorizontalPosition, Styled};
use mctk_core::widgets::{Button, IconButton};
use mctk_core::{component, layout, txt, Color, Point, Pos, Scale, AABB};
use mctk_core::{
    component::Component,
    lay, msg, node, rect,
    renderables::{
        image::InstanceBuilder as ImageInstanceBuilder,
        rect::InstanceBuilder as RectInstanceBuilder, svg::InstanceBuilder as SvgInstanceBuilder,
        text::InstanceBuilder as TextInstanceBuilder, Image, Rect, Renderable, Svg, Text,
    },
    size, size_pct, state_component_impl,
    widgets::Div,
    Node,
};
use std::any::Any;
use std::collections::HashMap;
use std::ffi::CString;
use std::ops::Neg;

pub enum IconType {
    Png,
    Svg,
}

#[derive(Debug, Clone)]
pub enum SettingNames {
    Wireless,
    Bluetooth,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    KeyboardKeyClicked(crate::action::Action),
    Show,
    Hide,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug)]
pub struct KeyboardState {
    settings: KeyboardSettings,
    layout: crate::layout::ParsedLayout,
    current_view: String,
}

#[component(State = "KeyboardState")]
#[derive(Debug, Default)]
pub struct Keyboard {}

impl Keyboard {}

#[state_component_impl(KeyboardState)]
impl Component for Keyboard {
    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => KeyboardSettings::default(),
        };

        let layout_path = settings.layouts.default.clone();
        println!("layout_path ============> : {:?} ", layout_path);

        let layout = match crate::layout::Layout::from_file(layout_path) {
            Ok(layout) => layout,
            Err(e) => {
                println!("Error parsing layout {:?}", e);
                panic!("");
            }
        };

        let parsed_layout = layout.clone().build().unwrap();

        println!("layout is {:?}", parsed_layout);

        self.state = Some(KeyboardState {
            settings,
            layout: parsed_layout,
            current_view: String::from("base"),
        });
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::KeyboardKeyClicked(action)) => match action {
                action::Action::SetView(view) => {
                    self.state_mut().current_view = view.clone();
                }
                action::Action::LockView {
                    lock,
                    unlock,
                    latches,
                    looks_locked_from,
                } => {
                    if self.state_ref().current_view == lock.clone() {
                        self.state_mut().current_view = unlock.clone();
                    } else {
                        self.state_mut().current_view = lock.clone();
                    }
                }
                action::Action::ApplyModifier(_) => {}
                action::Action::Submit { text, keys } => {}
                action::Action::Erase => {}
                action::Action::ShowPreferences => {}
            },
            Some(Message::Show) => {}
            Some(Message::Hide) => {}
            _ => (),
        }
        vec![]
    }

    // fn render(&mut self, context: component::RenderContext) -> Option<Vec<Renderable>> {
    //     let width = context.aabb.width();
    //     let height = context.aabb.height();
    //     let mut pos = context.aabb.pos;
    //     let mut rs = vec![];

    //     //Background
    //     let background = RectInstanceBuilder::default()
    //         .pos(pos)
    //         .scale(Scale { width, height })
    //         .color(Color::rgba(5., 7., 10., 0.76))
    //         .build()
    //         .unwrap();

    //     rs.push(Renderable::Rect(Rect::from_instance_data(background)));\
    //     rs
    // }

    fn view(&self) -> Option<Node> {
        let mut main_div = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100, 100],
                direction: layout::Direction::Column,
                cross_alignment: Alignment::Center,
            ]
        );

        //Render view from layout

        let layout = self.state_ref().layout.clone();
        let current_view = self.state_ref().current_view.clone();
        let view = layout.views.get(&current_view).unwrap();
        println!("rows {:?}", view.rows);
        for (i, row) in view.rows.clone().into_iter().enumerate() {
            let mut row_div = node!(
                Div::new(),
                lay![
                    margin: [10, 0,0,0],
                    cross_alignment: Alignment::Stretch,
                ]
            );
            // let cols: Vec<&str> = row.split(" ").filter(|c| c.len() > 0).collect();

            for (j, col) in row.buttons.into_iter().enumerate() {
                let action = col.action.clone();
                let col_div = match col.label.clone() {
                    crate::layout::Label::Text(text) => node!(
                        Button::new(txt!(text))
                            .on_click(Box::new(move || msg!(Message::KeyboardKeyClicked(
                                action.clone()
                            ))))
                            .style("h_alignment", HorizontalPosition::Center)
                            .style("radius", 4.6)
                            .style("text_color", Color::WHITE)
                            .style("font_size", 20.)
                            .style("active_color", Color::rgba(255., 255., 255., 0.50))
                            .style("background_color", Color::rgb(42., 42., 44.)),
                        lay![
                            size: [col.size.0, col.size.1],
                            margin: [0., 2.5, 0., 2.5],
                        ],
                    ),
                    crate::layout::Label::Icon(icon) => node!(
                        IconButton::new(icon)
                            .on_click(Box::new(move || msg!(Message::KeyboardKeyClicked(
                                action.clone()
                            ))))
                            .style("background_color", Color::rgb(42., 42., 44.))
                            .style("active_color", Color::rgba(255., 255., 255., 0.50))
                            .style("padding", 8.)
                            .style("radius", 12.),
                        lay![
                            size: [col.size.0, col.size.1],
                            margin: [0., 2.5, 0., 2.5],
                        ]
                    ),
                };

                row_div = row_div.push(col_div.key(j as u64));
            }
            main_div = main_div.push(row_div.key(i as u64));
        }

        Some(main_div)
    }
}

impl RootComponent<AppMessage> for Keyboard {
    fn root(&mut self, window: &dyn Any, app_channel: Option<Sender<AppMessage>>) {}
}
