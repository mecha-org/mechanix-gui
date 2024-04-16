use mctk_core::{
    component::Component,
    lay,
    layout::{Alignment, Direction},
    msg, node, rect, size, size_pct,
    style::Styled,
    widgets::{Carousel, Div, IconButton, Svg},
    Color, Node,
};

use crate::{
    components::user_card::UserCard,
    gui::{Message, Routes},
    users::User,
};

#[derive(Debug)]
pub struct Users {
    pub users: Vec<User>,
}

impl Component for Users {
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
                margin: [40, 0, 0, 0],
                size_pct: [100, Auto],
                direction: Row,
            ]
        );

        for (i, user) in self.users.clone().into_iter().enumerate() {
            users_list = users_list.push(
                node!(
                    UserCard::new(user.name.clone().unwrap(), user.username.clone()).on_click(
                        Box::new(move || msg!(if user.pin_enabled {
                            Message::ChangeRoute(Routes::Pin)
                        } else {
                            Message::ChangeRoute(Routes::Password)
                        }))
                    )
                )
                .key(i as u64),
            );
        }

        Some(
            node!(
                Div::new().bg(Color::BLACK),
                lay![
                    size_pct: [100],
                    cross_alignment: Stretch,
                    direction: Direction::Column,
                ]
            )
            .push(users_list)
            .push(footer),
        )
    }
}
