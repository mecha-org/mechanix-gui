use std::hash::Hash;

use crate::contexts::camera;
use crate::contexts::state;
use mctk_core::prelude::*;

#[derive(Debug)]
pub struct Footer;
impl Footer {
    pub fn new() -> Self {
        Self
    }
}

impl Component for Footer {
    fn render_hash(&self, hasher: &mut ComponentHasher) {
        state::State::get_settings_state().hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let height = camera::Camera::get_height() as usize;
        let width = camera::Camera::get_width() as usize;
        let aspect_ratio = width as f32 / height as f32;

        let mut base = node!(
            Div::new().bg(if aspect_ratio > (480.0 / 400.0) {
                Color::BLACK
            } else {
                Color::TRANSPARENT
            }),
            lay![
                size: size!(480.0, 480.0 - 480.0 / aspect_ratio),
                position_type: Absolute,
                position: [480.0 / aspect_ratio, Auto, Auto, -240.0],
                direction: Direction::Row,
                axis_alignment: Alignment::Start,
                cross_alignment: Alignment::Center,
            ]
        );

        let shutter_button = node!(
            Div::new().border(Color::WHITE, 2.0, (0.0, 0.0, 0.0, 0.0)),
            lay![
                size: size!(50.0, 50.0),
                direction: Direction::Row,
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Center,
                margin: [0.0, 0.0, 0.0, 0.0]
            ]
        )
        .push(node!(
            Button::new(txt!("")).on_click(Box::new(|| {
                camera::Camera::save_frame();
                msg!(())
            })),
            lay![size: size!(40.0, 40.0)]
        ));

        let settings_icon = node!(
            IconButton::new("settings_icon")
                .icon_type(IconType::Png)
                .style(
                    "size",
                    Size {
                        width: Dimension::Px(40.0),
                        height: Dimension::Px(40.0),
                    }
                )
                .style("text_color", Color::WHITE)
                .style("background_color", Color::BLACK)
                .on_click(Box::new(|| {
                    state::State::set_settings_state(true);
                    msg!(())
                })),
            lay![
                size: size!(50.0, 50.0),
                direction: Direction::Row,
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Center,
                margin: [0.0, 10.0, 0.0, 240.0 - (50.0/2.0) - 50.0 -  10.0]
            ]
        );

        base = base.push(settings_icon);
        base = base.push(shutter_button);
        Some(base)
    }
}
