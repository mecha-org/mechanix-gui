use mctk_core::{
    component::{Component, Message},
    event::{Click, Event},
    lay,
    layout::{Alignment, Direction},
    node, rect, size, size_pct,
    style::{FontWeight, HorizontalPosition, Styled, VerticalPosition},
    txt,
    widgets::{Div, IconButton, IconType, Image, Svg, Text},
    Color, Node,
};

pub struct FilterApp {
    name: String,
    icon: Option<(String, IconType)>,
    on_click: Option<Box<dyn Fn() -> Message + Send + Sync>>,
}

impl std::fmt::Debug for FilterApp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("App").field("name", &self.name).finish()
    }
}

impl FilterApp {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            icon: None,
            on_click: None,
        }
    }

    pub fn on_click(mut self, on_click: Box<dyn Fn() -> Message + Send + Sync>) -> Self {
        self.on_click = Some(on_click);
        self
    }

    pub fn icon(mut self, icon: (String, IconType)) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl Component for FilterApp {
    fn on_click(&mut self, event: &mut Event<Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn view(&self) -> Option<Node> {
        let mut icon = "not_found_small_icon".to_string();
        let mut icon_type = IconType::Png;

        if let Some((ic, it)) = self.icon.clone() {
            icon = ic;
            icon_type = it;
        }

        let app_icon = match icon_type {
            IconType::Svg => node!(
                Svg::new(icon),
                lay![
                    size_pct: [100],
                ],
            ),
            IconType::Png => node!(
                Image::new(icon),
                lay![
                    size_pct: [100],
                ],
            ),
        };

        let mut app_name = self.name.clone();

        if app_name.len() > 10 {
            app_name.truncate(10);
            app_name.push_str("...");
        }

        Some(
            node!(
                Div::new(),
                lay![
                    size: [Auto],
                    direction: Direction::Row,
                    margin: [0, 0, 12, 0]
                ]
            )
            .push(
                node!(
                    Div::new().bg(Color::rgb(19., 19., 20.)).border(
                        Color::rgb(21., 23., 29.),
                        2.,
                        (10.5, 10.5, 10.5, 10.5)
                    ),
                    lay![
                         size: [80, 80],
                         axis_alignment: Alignment::Center,
                         cross_alignment: Alignment::Center,
                         margin: [0, 0, 0, 20],
                         padding: [11]
                    ]
                )
                .push(app_icon),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                         size_pct: [100],
                         cross_alignment: Alignment::Center
                    ]
                )
                .push(node!(
                    Text::new(txt!(app_name))
                        .style("color", Color::rgb(197., 200., 207.))
                        .style("size", 21.0)
                        .style("font_weight", FontWeight::Normal) // .style("v_alignment", VerticalPosition::Center)
                )),
            ),
        )
    }
}
