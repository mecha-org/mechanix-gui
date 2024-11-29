use std::{fmt::Debug, hash::Hash};

use mctk_core::{
    component::{Component, Message},
    event, lay,
    layout::{self, Alignment},
    node, rect, size,
    style::{self, FontWeight, Styled},
    txt,
    widgets::{Div, Image, Text},
    Color,
};
use mctk_macros::{component, state_component_impl};

#[derive(Debug, Default)]
struct CloseButtonState {
    pressed: bool,
}

#[component(State = "CloseButtonState")]
pub struct CloseButton {
    on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl Debug for CloseButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CloseButton").finish()
    }
}

impl CloseButton {
    pub fn new() -> Self {
        Self {
            on_click: None,
            state: Some(CloseButtonState::default()),
            dirty: false,
        }
    }
    pub fn on_click(mut self, f: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_click = Some(f);
        self
    }
}

#[state_component_impl(CloseButtonState)]
impl Component for CloseButton {
    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.state_ref().pressed.hash(hasher);
    }

    fn on_mouse_down(&mut self, event: &mut event::Event<event::MouseDown>) {
        self.state_mut().pressed = true;
    }

    fn on_mouse_up(&mut self, event: &mut event::Event<event::MouseUp>) {
        self.state_mut().pressed = false;
    }

    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn on_touch_down(&mut self, event: &mut event::Event<event::TouchDown>) {
        self.state_mut().pressed = true;
    }

    fn on_touch_up(&mut self, event: &mut event::Event<event::TouchUp>) {
        self.state_mut().pressed = false;
    }
    fn view(&self) -> Option<mctk_core::Node> {
        let pressed = self.state_ref().pressed;
        Some(
            node!(
                Div::new().bg(if pressed {
                    Color::rgb(43., 43., 43.)
                } else {
                    Color::TRANSPARENT
                }),
                lay![
                    direction: layout::Direction::Row,
                    cross_alignment: Alignment::Center,
                    padding: [10.]
                ]
            )
            .push(node!(
                Image::new("close_icon"),
                lay![
                    size: [30, 30],
                    margin: [0., 0., 0., 6.]
                ],
            ))
            .push(node!(Text::new(txt!("Close"))
                .with_class("text-white text-xl font-space-grotesk font-medium")
                .style("color", Color::WHITE)
                .style("line_height", 22.0),)),
        )
    }
}
