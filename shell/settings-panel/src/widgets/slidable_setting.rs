use mctk_core::{
    component::{Component, Message},
    lay, msg, node, rect, size, size_pct,
    style::{FontWeight, Styled, VerticalPosition},
    txt,
    widgets::{Div, RoundedRect, Slider, Svg, Text},
    Color, Node,
};

enum SlidableSettingMessage {
    ValueChanged(i32),
}

pub struct SlidableSetting {
    pub icon: String,
    pub text: String,
    pub value: i32,
    pub on_slide: Option<Box<dyn Fn(i32) -> Message + Send + Sync>>,
}

impl std::fmt::Debug for SlidableSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SlidableSetting")
            .field("text", &self.text)
            .finish()
    }
}

impl SlidableSetting {
    pub fn new(icon: String, text: String, value: i32) -> Self {
        Self {
            icon,
            text,
            value,
            on_slide: None,
        }
    }

    pub fn on_slide(mut self, on_slide: Box<dyn Fn(i32) -> Message + Send + Sync>) -> Self {
        self.on_slide = Some(on_slide);
        self
    }
}

impl Component for SlidableSetting {
    fn update(&mut self, msg: Message) -> Vec<Message> {
        let mut m: Vec<Message> = vec![];
        match msg.downcast_ref::<SlidableSettingMessage>() {
            Some(SlidableSettingMessage::ValueChanged(value)) => {
                if let Some(slide_fn) = &self.on_slide {
                    m.push(slide_fn(*value));
                }
            }
            None => panic!(),
        }
        m
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                RoundedRect{
                    background_color: Color::rgba(22., 23., 23., 0.90),
                    border_color: Color::TRANSPARENT,
                    border_width: 1.,
                    radius: (8., 8., 8. ,8.)
                }
                ,
                [
                    size: [214, 108],
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
            .push(node!(Slider::new(self.value).on_slide(Box::new(|value| msg!(SlidableSettingMessage::ValueChanged(value)))), [
                size_pct: [100, Auto],
                margin: [6, 0, 18, 0],
            ]))
            .push(node!(Text::new(txt!(self.text.clone()))
                .style("color", Color::rgb(197., 200., 207.))
                .style("size", 12.0)
                .style("font_weight", FontWeight::Medium)
               ,)),
        )
    }
}
