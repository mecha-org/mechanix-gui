use std::hash::Hash;

use mctk_core::event::{MouseUp, TouchDown, TouchUp};
use mctk_core::layout::{Alignment, Direction, PositionType};
use mctk_core::style::{FontWeight, Styled, VerticalPosition};
use mctk_core::{
    component::{Component, Message},
    event::{Click, Event, MouseDown},
    lay, node, rect, size, size_pct, txt,
    widgets::{Div, RoundedRect, Svg, Text},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};

#[derive(Debug, Default)]
struct ClickableSettingState {
    pressing: bool,
    click_disabled: bool,
    loading: bool,
    disabled: bool,
}

#[derive(Debug, Clone, Hash)]
pub enum SettingText {
    Normal(String),
    Subscript(String, String),
    Bold(String),
}

#[component(State = "ClickableSettingState")]
pub struct ClickableSetting {
    pub icon: String,
    pub text_1: String,
    pub text_2: SettingText,
    pub on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl std::fmt::Debug for ClickableSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClickableSetting")
            .field("text", &self.text_1)
            .finish()
    }
}

impl ClickableSetting {
    pub fn new(icon: String, text_1: String, text_2: SettingText) -> Self {
        Self {
            icon,
            text_1,
            text_2,
            on_click: None,
            state: Some(ClickableSettingState::default()),
            dirty: false,
        }
    }

    pub fn on_click(mut self, on_click: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn click_disabled(mut self, click_disabled: bool) -> Self {
        self.state_mut().click_disabled = click_disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.state_mut().loading = loading;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.state_mut().disabled = disabled;
        self
    }
}

#[state_component_impl(ClickableSettingState)]
impl Component for ClickableSetting {
    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        if self.state.is_some() {
            self.state_ref().pressing.hash(hasher);
            self.state_ref().click_disabled.hash(hasher);
            self.state_ref().loading.hash(hasher);
            self.state_ref().disabled.hash(hasher);
        }
    }

    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.icon.hash(hasher);
        self.text_1.hash(hasher);
        self.text_2.hash(hasher);
    }

    fn on_click(&mut self, event: &mut Event<Click>) {
        if let Some(f) = &self.on_click {
            if !self.state_ref().click_disabled && !self.state_ref().loading {
                event.emit(f());
            }
        }
    }

    fn on_mouse_down(&mut self, _event: &mut Event<MouseDown>) {
        self.state_mut().pressing = true;
    }

    fn on_mouse_up(&mut self, _event: &mut Event<MouseUp>) {
        self.state_mut().pressing = false;
    }

    fn on_touch_down(&mut self, _event: &mut Event<TouchDown>) {
        self.state_mut().pressing = true;
    }

    fn on_touch_up(&mut self, _event: &mut Event<TouchUp>) {
        self.state_mut().pressing = false;
    }

    fn view(&self) -> Option<Node> {
        let bg_color = if self.state_ref().pressing && !self.state_ref().click_disabled {
            Color::rgba(55., 55., 56., 0.95)
        } else {
            Color::rgba(22., 23., 23., 0.90)
        };

        let mut text_2_node = node!(
            Div::new(),
            lay![
                size: [Auto, 26],
                margin: [0, 0, 0, 0],
                direction: Direction::Row,
                cross_alignment: Alignment::End,

            ]
        );

        match self.text_2.clone() {
            SettingText::Normal(text) => {
                text_2_node = text_2_node.push(node!(Text::new(txt!(text.clone()))
                    .style("color", Color::rgb(197., 200., 207.))
                    .style("size", 12.0)
                    .style("font_weight", FontWeight::Normal)));
            }
            SettingText::Subscript(text, subscript) => {
                text_2_node = text_2_node.push(node!(Text::new(txt!(text))
                    .style("color", Color::WHITE)
                    .style("size", 20.0)
                    .style("font_weight", FontWeight::Normal)));
                text_2_node = text_2_node.push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [Auto, 100],
                            cross_alignment: Alignment::End,
                            padding: [0., 2., 2., 0.]
                        ]
                    )
                    .push(node!(Text::new(txt!(subscript))
                        .style("color", Color::rgb(197., 200., 207.))
                        .style("size", 12.0)
                        .style("font_weight", FontWeight::Normal),)),
                );
            }
            SettingText::Bold(text) => {
                text_2_node = text_2_node.push(node!(Text::new(txt!(text))
                    .style("color", Color::WHITE)
                    .style("size", 20.0)
                    .style("font_weight", FontWeight::Normal)));
            }
        }

        let text_1_color = if self.state_ref().disabled {
            Color::rgba(197., 200., 207., 0.40)
        } else {
            Color::rgb(197., 200., 207.)
        };

        Some(
            node!(
                RoundedRect{
                    background_color: bg_color,
                    border_color: Color::TRANSPARENT,
                    border_width: 1.,
                    radius: (8., 8., 8. ,8.),
                    scissor: None
                }
                ,
                [
                    size: [100, 100],
                    direction: Column,
                    padding: [10]
                ]
            )
            .push(node!(
                Svg::new(self.icon.to_string()),
                lay![
                    size: [32, 32],
                    margin: [0, 0, 8, 0]
                ],
            ))
            .push(text_2_node)
            .push(node!(
                Text::new(txt!(self.text_1.clone()))
                    .style("color", text_1_color)
                    .style("size", 12.0)
                    //.style("font_weight", FontWeight::Medium)
                    .style("font_weight", FontWeight::Normal), // .style("v_alignment", VerticalPosition::Bottom)
            )),
        )
    }
}
