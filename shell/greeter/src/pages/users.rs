use mctk_core::{
    component::{Component, RenderContext},
    lay,
    layout::{Alignment, Direction},
    msg, node, rect,
    renderables::{rect::InstanceBuilder as RectInstanceBuilder, Rect, Renderable},
    size, size_pct,
    style::Styled,
    widgets::{Carousel, Div, IconButton, Svg},
    Color, Node, Pos, Scale, AABB,
};

use crate::{
    components::user_card::UserCard,
    gui::{Message, PasswordAuthMessage, PasswordAuthRoute, Routes},
    users::User,
};

#[derive(Debug)]
pub struct Users {
    pub users: Vec<User>,
}

impl Component for Users {
    // fn render(&mut self, context: RenderContext) -> Option<Vec<Renderable>> {
    //     let AABB { pos, .. } = context.aabb;
    //     let mut rs = vec![];

    //     //Left slide
    //     let pos = Pos::from([0., 0.]);
    //     let scale = Scale::default() + 100.;
    //     let left_slide = RectInstanceBuilder::default()
    //         .pos(pos)
    //         .scale(scale)
    //         .color(Color::rgb(26., 28., 33.))
    //         .radius((40., 40., 40., 40.))
    //         .build()
    //         .unwrap();

    //     //Content slide

    //     //Right slide

    //     rs.push(Renderable::Rect(Rect::from_instance_data(left_slide)));

    //     Some(rs)
    // }

    fn view(&self) -> Option<Node> {
        let footer = node!(
            Div::new(),
            lay![
                position_type: Absolute,
                position: [Auto, 0.0, 0.0, 0.0],
                size: [Auto, 80],
                cross_alignment: Alignment::Center,
                padding: [9, 18, 9, 18]
            ]
        )
        .push(node!(
            IconButton::new("power_icon")
                .on_click(Box::new(|| msg!(Message::ChangeRoute(
                    Routes::PowerOptions
                ))))
                .style("background_color", Color::rgb(21., 23., 29.))
                .style("active_color", Color::rgba(29., 23., 29., 0.5))
                .style("padding", 8.)
                .style("radius", 12.),
            lay![
                size: [60, 60],
            ]
        ));

        let mut users_list = node!(
            Carousel::new().scroll_x(),
            lay![
                padding: [14, 24, 18, 0],
                margin: [20, 0, 0, 0],
                size_pct: [100, Auto],
                direction: Row,
            ]
        );
        // .push(node!(UserCard::new("First", "card")))
        // .push(node!(UserCard::new("Second", "card")));
        // .push(node!(Div::new().bg(Color::RED), [size: [100], margin: [10]]))
        // .push(node!(Div::new().bg(Color::GREEN), [size: [100]]));

        for (i, user) in self.users.clone().into_iter().enumerate() {
            users_list = users_list.push(node!(UserCard::new(
                user.name.clone().unwrap(),
                user.username.clone()
            )
            .on_click(Box::new(move || { msg!(PasswordAuthMessage::Submit) })),));
        }

        Some(
            node!(
                Div::new().bg(Color::rgba(0., 0., 0., 0.80)),
                lay![
                    size_pct: [100],
                    // cross_alignment: Stretch,
                    direction: Direction::Column,
                ]
            )
            .push(users_list), // .push(footer),
        )
    }
}
