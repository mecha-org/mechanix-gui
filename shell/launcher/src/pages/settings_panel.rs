use std::hash::Hash;

use mctk_core::layout::{Alignment, Direction};
use mctk_core::style::Styled;
use mctk_core::widgets::{IconButton, IconType, RoundedRect};
use mctk_core::{component::Component, lay, node, rect, size, size_pct, widgets::Div, Node};
use mctk_core::{msg, Color};
use networkmanager::WirelessModel;

use crate::gui;
use crate::modules::settings_panel::brightness::component::Brightness;
use crate::modules::settings_panel::closer::Closer;
use crate::modules::settings_panel::rotation::component::RotationStatus;
use crate::modules::settings_panel::sound::component::Sound;
use crate::shared::h_divider::HDivider;
use crate::shared::v_divider::VDivider;
use crate::types::{BatteryLevel, BluetoothStatus, WirelessStatus};
use crate::utils::get_forttated_wireless_status;

#[derive(Debug)]
pub struct SettingsPanel {
    pub swipe: i32,
    pub sound: u8,
    pub brightness: u8,
    pub bluetooth_status: BluetoothStatus,
    pub rotation_status: RotationStatus,
}

impl Component for SettingsPanel {
    fn props_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.sound.hash(hasher);
        self.brightness.hash(hasher);
        self.bluetooth_status.hash(hasher);
        self.rotation_status.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let swipe = self.swipe;
        let sound = self.sound;
        let brightness = self.brightness;
        let bluetooth_status = self.bluetooth_status;
        let rotation_status = self.rotation_status;
        let wireless_status = get_forttated_wireless_status(WirelessModel::get());

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
                padding: [36., 22., 20., 22.],
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
                axis_alignment: Alignment::Start,
            ]
        );
        row_1 = row_1.push(node!(IconButton::new(format!(
            "lg{:?}",
            wireless_status.to_string()
        ))
        .icon_type(IconType::Png)
        .on_click(Box::new(|| msg!(gui::Message::SettingClicked(
            gui::SettingNames::Wireless
        ))))
        .with_class("btn-xxl border-0 p-8")
        .style("active_color", Color::rgb(43., 43., 43.))));
        row_1 = row_1.push(node!(VDivider { size: 1.5 }));
        row_1 = row_1.push(node!(IconButton::new(format!(
            "lg{:?}",
            bluetooth_status.to_string()
        ))
        .icon_type(IconType::Png)
        .on_click(Box::new(|| msg!(gui::Message::SettingClicked(
            gui::SettingNames::Bluetooth
        ))))
        .with_class("btn-xxl border-0 p-8")
        .style("active_color", Color::rgb(43., 43., 43.)),));
        row_1 = row_1.push(node!(VDivider { size: 1.5 }));
        row_1 = row_1.push(node!(
            IconButton::new(rotation_status.to_string())
                .icon_type(IconType::Png)
                // .on_click(Box::new(|| msg!(gui::Message::SettingClicked(
                //     gui::SettingNames::Rotation
                // ))))
                .with_class("btn-xxl border-0 p-8"), // .style("active_color", Color::rgb(43., 43., 43.))
        ));
        row_1 = row_1.push(node!(VDivider { size: 1.5 }));
        row_1 = row_1.push(node!(IconButton::new("terminal_icon")
            .icon_type(IconType::Png)
            .on_click(Box::new(|| msg!(gui::Message::SettingClicked(
                gui::SettingNames::Terminal
            ))))
            .with_class("btn-xxl border-0 p-8")
            .style("active_color", Color::rgb(43., 43., 43.)),));
        row_1 = row_1.push(node!(VDivider { size: 1.5 }));
        row_1 = row_1.push(node!(IconButton::new("power_icon")
            .icon_type(IconType::Png)
            .on_click(Box::new(|| msg!(gui::Message::SettingClicked(
                gui::SettingNames::Power
            ))))
            .with_class("btn-xxl border-0 p-8")
            .style("active_color", Color::rgb(43., 43., 43.)),));

        // println!("swipe - height {:?}", swipe - height);

        let sc1 = node!(
            RoundedRect {
                scissor: Some(true),
                background_color: Color::TRANSPARENT,
                border_color: Color::TRANSPARENT,
                border_width: (0., 0., 0., 0.),
                radius: (0., 0., 0., 0.),
                swipe
            },
            lay![
                size_pct: [100],
                position_type: Absolute,
                position: [swipe - height, 0., Auto, Auto]
                z_index_increment: 1000.
            ]
        );

        let sc2 = node!(
            RoundedRect {
                scissor: Some(false),
                background_color: Color::TRANSPARENT,
                border_color: Color::TRANSPARENT,
                border_width: (0., 0., 0., 0.),
                radius: (0., 0., 0., 0.),
                swipe: swipe
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
            lay![ size: [Auto, 138], margin: [0., 0., 4., 0.] ]
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

    fn on_touch_drag_start(
        &mut self,
        event: &mut mctk_core::event::Event<mctk_core::event::TouchDragStart>,
    ) {
        if self.swipe > 10 {
            event.stop_bubbling()
        }
    }

    fn on_click(&mut self, event: &mut mctk_core::event::Event<mctk_core::event::Click>) {
        event.stop_bubbling();
    }
}
