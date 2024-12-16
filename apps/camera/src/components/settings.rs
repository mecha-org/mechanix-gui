use std::hash::Hash;

use crate::contexts::camera;
use crate::contexts::state;
use mctk_core::prelude::*;

#[derive(Debug)]
pub struct Settings;
impl Settings {
    pub fn new() -> Self {
        Self
    }
}

impl Component for Settings {
    fn render_hash(&self, hasher: &mut ComponentHasher) {
        state::State::get_settings_state().hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let mut base = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size: size!(480.0, 300.0),
                position_type: Absolute,
                position: [180.0, Auto, Auto, -240.0],
                direction: Direction::Column,
                axis_alignment: Alignment::Start,
                cross_alignment: Alignment::Center,
            ]
        );

        let mut header = node!(
            Div::new(),
            lay![
                size: size!(480.0, 50.0),
                direction: Direction::Row,
                axis_alignment: Alignment::End,
                cross_alignment: Alignment::Center,
            ]
        );
        let close_button = node!(
            Button::new(txt!("X"))
                .style("font_size", 20.0)
                .style("text_color", Color::WHITE)
                .style("background_color", Color::BLACK)
                .on_click(Box::new(|| {
                    state::State::set_settings_state(false);
                    msg!(())
                })),
            lay![
                size: size!(30.0, 30.0),
                margin: [10.0]
            ]
        );

        let heading = node!(
            Text::new(txt!("Settings"))
                .style("size", 20.0)
                .style("color", Color::WHITE),
            lay![margin: [0.0, 10.0, 0.0, 340.0]]
        );

        header = header.push(heading);
        header = header.push(close_button);
        base = base.push(header);

        Some(base)
    }
}
