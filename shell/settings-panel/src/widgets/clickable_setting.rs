use mctk_core::event::MouseUp;
use mctk_core::style::{Styled, VerticalPosition};
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
}

#[component(State = "ClickableSettingState")]
pub struct ClickableSetting {
    pub icon: String,
    pub text_1: String,
    pub text_2: String,
    pub font: String,
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
    pub fn new(icon: String, text_1: String, text_2: String, font: String) -> Self {
        Self {
            icon,
            text_1,
            text_2,
            font,
            on_click: None,
            state: Some(ClickableSettingState::default()),
            dirty: false,
        }
    }

    pub fn on_click(mut self, on_click: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

#[state_component_impl(ClickableSettingState)]
impl Component for ClickableSetting {
    fn on_click(&mut self, event: &mut Event<Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn on_mouse_down(&mut self, _event: &mut Event<MouseDown>) {
        self.state_mut().pressing = true;
    }

    fn on_mouse_up(&mut self, _event: &mut Event<MouseUp>) {
        self.state_mut().pressing = false;
    }

    fn view(&self) -> Option<Node> {
        let bg_color = if self.state_ref().pressing {
            Color::rgba(29., 23., 29., 0.5)
        } else {
            Color::rgb(21., 23., 29.)
        };

        Some(
            node!(
                RoundedRect{
                    background_color: bg_color,
                    border_color: Color::rgb(21., 23., 29.),
                    border_width: 1.,
                    radius: (8., 8., 8. ,8.)
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
            .push(node!(
            Text::new(txt!(self.text_2.clone()))
                .style("font", "SpaceGrotesk-Medium")
                .style("color",Color::rgb(255., 255., 255.))
                .style("size", 18.0),
            [
                size_pct: [100.0, Auto],
            ]))
            .push(node!(
            Text::new(txt!(self.text_1.clone()))
                .style("font", "SpaceGrotesk-Medium")
                .style("color",Color::rgb(197., 200., 207.))
                .style("size", 12.0)
                .style("v_alignment", VerticalPosition::Bottom),
            [
                size_pct: [100.0, Auto]
            ])),
        )
    }
}
