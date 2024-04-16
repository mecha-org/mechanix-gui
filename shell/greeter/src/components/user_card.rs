use mctk_core::{
    component::{Component, Message},
    event::{Click, Event},
    lay,
    layout::{Alignment, Direction},
    node, rect, size, size_pct,
    style::{HorizontalPosition, Styled, VerticalPosition},
    txt,
    widgets::{Div, Image, Svg, Text},
    Color, Node,
};

pub struct UserCard {
    name: String,
    username: String,
    on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl std::fmt::Debug for UserCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UserCard")
            .field("name", &self.name)
            .finish()
    }
}

impl UserCard {
    pub fn new<S: Into<String>>(name: S, username: S) -> Self {
        Self {
            name: name.into(),
            username: username.into(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, on_click: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

impl Component for UserCard {
    fn on_click(&mut self, event: &mut Event<Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn view(&self) -> Option<Node> {
        let is_custom = self.username == "custom";
        let border = if is_custom {
            Color::WHITE
        } else {
            Color::rgb(254., 221., 0.)
        };

        Some(
            node!(
                Div::new().bg(Color::rgba(22., 23., 23., 0.95)).border(
                    Color::TRANSPARENT,
                    1.,
                    (20., 20., 20., 20.)
                ),
                lay![
                     size: [272, 329],
                     margin: [0., 0., 0., 24],
                     cross_alignment: Alignment::Center,
                     axis_alignment: Alignment::Center,
                     direction: Direction::Column
                ]
            )
            .push(node!(
                Svg::new(self.username.clone()),
                lay![
                    size: [120., 120.],
                    margin: [ 0., 0., 32., 0. ]
                ],
            ))
            .push(node!(Text::new(txt!(self.name.clone()))
                .style("color", Color::rgb(197., 200., 207.))
                .style("size", 22.0)
                .style("h_alignment", HorizontalPosition::Center)
                .style("v_alignment", VerticalPosition::Center))),
        )
    }
}
