use std::hash::Hash;

use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::Styled;
use mctk_core::widgets::{IconButton, RoundedRect};
use mctk_core::Color;
use mctk_core::{component::Component, lay, node, rect, size, size_pct, widgets::Div, Node};

use crate::modules::settings_panel::brightness::component::Brightness;
use crate::modules::settings_panel::closer::Closer;
use crate::modules::settings_panel::sound::component::Sound;
use crate::shared::h_divider::HDivider;
use crate::shared::v_divider::VDivider;

#[derive(Debug)]
pub struct SettingsPanel {
    pub swipe: i32,
    pub sound: u8,
    pub brightness: u8,
}

impl Component for SettingsPanel {
    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.swipe.hash(hasher);
        self.sound.hash(hasher);
        self.brightness.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let swipe = self.swipe;
        let sound = self.sound;
        let brightness = self.brightness;
        // println!("view() swipe {:?}", swipe);
        let width = 480;
        let height = 480;

        let mut p1 = node!(
            Div::new().bg(Color::rgba(0., 0., 0., 0.85)),
            lay![
                size: [width, height],
                // padding: [20., 20., 20., 20.],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column
            ]
        );

        let mut rows = node!(
            Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100, Auto],
                // size: [480, 434]
                padding: [20., 22., 20., 22.],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column
            ]
        );

        let mut row_1 = node!(
            Div::new(),
            lay![
                size: [Auto, 88],
                direction: Direction::Row,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Center,
            ]
        );
        row_1 = row_1.push(node!(
            IconButton::new("WirelessConnected(Strong)")
                // .on_click(Box::new(move || msg!(message.clone())))
                .style("background_color", Color::TRANSPARENT)
                .style("active_color", Color::rgba(255., 255., 255., 0.50)),
            lay![
                size: [80, 88],
                padding: [24, 13., 24, 26.],
            ]
        ));
        row_1 = row_1.push(node!(VDivider { size: 1.5 }));
        row_1 = row_1.push(node!(
            IconButton::new("BluetoothOn")
                // .on_click(Box::new(move || msg!(message.clone())))
                .style("background_color", Color::TRANSPARENT)
                .style("active_color", Color::rgba(255., 255., 255., 0.50)),
            lay![
                size: [92, 88],
                padding: [24., 26., 24., 26.]
            ]
        ));
        row_1 = row_1.push(node!(VDivider { size: 1.5 }));
        row_1 = row_1.push(node!(
            IconButton::new("RotationPortrait")
                // .on_click(Box::new(move || msg!(message.clone())))
                .style("background_color", Color::TRANSPARENT)
                .style("active_color", Color::rgba(255., 255., 255., 0.50)),
            lay![
                size: [92, 88],
                padding: [24., 26., 24., 26.]
            ]
        ));
        row_1 = row_1.push(node!(VDivider { size: 1.5 }));
        row_1 = row_1.push(node!(
            Div::new(),
            lay![
                size: [92, 88],
            ]
        ));
        row_1 = row_1.push(node!(VDivider { size: 1.5 }));
        row_1 = row_1.push(node!(
            IconButton::new("power_icon")
                // .on_click(Box::new(move || msg!(message.clone())))
                .style("background_color", Color::TRANSPARENT)
                .style("active_color", Color::rgba(255., 255., 255., 0.50)),
            lay![
                size: [80, 88],
                padding: [24., 26., 24., 13.]
            ]
        ));

        let sc1 = node!(
            RoundedRect {
                scissor: Some(true),
                background_color: Color::TRANSPARENT,
                border_color: Color::TRANSPARENT,
                border_width: 0.,
                radius: (0., 0., 0., 0.),
            },
            lay![
                size_pct: [100],
                position_type: Absolute,
                //Settings panel
                // position: [Auto, Auto , 80., 0.],
                position: [swipe - height, 0., Auto, Auto]

                //Notifications
                // position: [0., Auto, Auto, 80.],
                // position: [0., swipe - width, Auto, Auto],

                //App drawer
                // position: [100., 0., Auto, Auto],
                // position: [Auto, Auto, swipe - height, 0.],

                //Running Apps
                // position: [0, 70., Auto, Auto],
                // position: [Auto, Auto , 0., swipe - width],
                z_index_increment: 1000.
            ]
        )
        .key(swipe as u64);

        let sc2 = node!(
            RoundedRect {
                scissor: Some(false),
                background_color: Color::TRANSPARENT,
                border_color: Color::TRANSPARENT,
                border_width: 0.,
                radius: (0., 0., 0., 0.)
            },
            lay![position_type: Absolute,]
        );
        rows = rows.push(node!(HDivider { size: 1. }));
        rows = rows.push(row_1);
        rows = rows.push(node!(
            HDivider { size: 1. },
            lay![margin: [0., 0., 12., 0.]]
        ));
        rows = rows.push(node!(Sound { value: sound }, lay![ size_pct: [100, Auto] ]));
        rows = rows.push(node!(
            HDivider { size: 1. },
            lay![margin: [12., 0., 12., 0.]]
        ));
        rows = rows.push(node!(
            Brightness { value: brightness },
            lay![ size_pct: [100, Auto] ]
        ));
        rows = rows.push(node!(
            HDivider { size: 1. },
            lay![margin: [12., 0., 20., 0.]]
        ));
        rows = rows.push(node!(
            Closer {},
            lay![ size: [Auto, 20], margin: [0., 0., 4., 0.] ]
        ));
        p1 = p1.push(sc2);
        p1 = p1.push(rows);
        p1 = p1.push(sc1);

        Some(p1)
    }

    fn on_drag_start(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::DragStart>) {
        if self.swipe > 10 {
            event.stop_bubbling()
        }
    }

    fn on_click(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Click>) {
        event.stop_bubbling();
    }
}
