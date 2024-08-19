use std::collections::VecDeque;

use crate::gui::{self, Message, SwipeGestures};
use crate::modules::clock::component::Clock;
use crate::modules::controls::Controls;
use crate::modules::cpu::component::CPU;
use crate::modules::ip_address::component::IpAddress;
use crate::modules::memory::component::Memory;
use crate::modules::name::component::MachineName;
use crate::modules::networking::component::Networking;
use crate::modules::pinned_app::PinnedApp;
use crate::modules::uptime::component::Uptime;
use crate::settings::LauncherSettings;
use crate::shared::h_divider::HDivider;
use crate::shared::slider::{Slider, SliderType};
use crate::shared::v_divider::VDivider;
use crate::types::{BatteryLevel, BluetoothStatus, WirelessStatus};
use mctk_core::layout::{Alignment, Direction};
use mctk_core::{component::Component, lay, node, rect, size, size_pct, widgets::Div, Node};
use mctk_core::{msg, Color};

#[derive(Debug, Default)]
pub struct HomeUi {
    pub settings: LauncherSettings,
    pub battery_level: BatteryLevel,
    pub wireless_status: WirelessStatus,
    pub bluetooth_status: BluetoothStatus,
    pub time: String,
    pub date: String,
    pub cpu_usage: VecDeque<u8>,
    pub uptime: String,
    pub machine_name: String,
    pub ip_address: String,
    pub online: bool,
    pub used_memory: u64,
    pub swipe_gesture: Option<SwipeGestures>,
    pub is_lock_screen: bool,
}

impl HomeUi {
    // fn handle_on_drag(&mut self, logical_delta: Point) -> Option<mctk_core::component::Message> {
    //     let dx = logical_delta.x;
    //     let dy = logical_delta.y;
    //     let min_drag = 10.;

    //     if dx.abs() > min_drag || dy.abs() > min_drag {
    //         if dx > dy {
    //             if logical_delta.x > 0. {
    //                 return Some(msg!(Message::Swipe {
    //                     direction: SwipeGestures::Right(dx.abs() as i32)
    //                 }));
    //             } else {
    //                 return Some(msg!(Message::Swipe {
    //                     direction: SwipeGestures::Left(dx.abs() as i32)
    //                 }));
    //             }
    //         } else {
    //             if logical_delta.y > 0. {
    //                 return Some(msg!(Message::Swipe {
    //                     direction: SwipeGestures::Down(dy.abs() as i32)
    //                 }));
    //             } else {
    //                 return Some(msg!(Message::Swipe {
    //                     direction: SwipeGestures::Up(dy.abs() as i32)
    //                 }));
    //             }
    //         };
    //     }

    //     None
    // }
}

impl Component for HomeUi {
    fn view(&self) -> Option<Node> {
        // println!("HomeUi::view()");

        let cpu_usage = self.cpu_usage.clone();
        let uptime = self.uptime.clone();
        let machine_name = self.machine_name.clone();
        let ip_address = self.ip_address.clone();
        let online = self.online.clone();
        let used_memory = self.used_memory;
        let time = self.time.clone();
        let date = self.date.clone();
        let battery_level = self.battery_level.clone();
        let wireless_status = self.wireless_status.clone();
        let bluetooth_status = self.bluetooth_status.clone();

        let mut start_node = node!(
            Div::new().bg(Color::rgba(0., 0., 0., 0.64)),
            lay![
                size_pct: [100],
                // size: [480, 434]
                padding: [20., 22., 20., 22.],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column
            ]
        );

        let mut row_1 = node!(
            Div::new(),
            lay![
                size: [Auto, 123],
                direction: Direction::Row,
                cross_alignment: Alignment::Center
            ]
        );

        row_1 = row_1.push(node!(
            Clock { date, time },
            lay![
                size_pct: [50, Auto]
            ]
        ));

        row_1 = row_1.push(node!(
            Controls {
                battery_level: BatteryLevel::Level100,
                wireless_status,
                bluetooth_status: BluetoothStatus::Connected,
            },
            lay![
                size_pct: [50, Auto],
                axis_alignment: Alignment::End,
            ]
        ));

        let mut row_2 = node!(
            Div::new(),
            lay![
                size: [Auto, 56],
                direction: Direction::Row,
                cross_alignment: Alignment::Center
            ]
        );

        row_2 = row_2.push(node!(
            MachineName { name: machine_name },
            lay![
                size_pct: [50, Auto]
            ],
        ));

        row_2 = row_2.push(node!(
            IpAddress { ip_address },
            lay![
                size_pct: [50, Auto],
                axis_alignment: Alignment::End,
            ]
        ));

        let mut row_3 = node!(
            Div::new(),
            lay![
                size: [Auto, 142],
                direction: Direction::Row,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Center,
                // padding: [14., 0., 10., 0.]
            ]
        );

        row_3 = row_3.push(
            node!(
                Div::new(),
                lay![
                    size: [145, Auto],
                    direction: Direction::Column,
                    padding: [14., 0., 10., 0.]
                ]
            )
            .push(node!(
                Uptime {
                    time: uptime.clone()
                },
                lay![],
            ))
            .push(node!(
                Networking {
                    status: if online {
                        "Online".to_string()
                    } else {
                        "Offline".to_string()
                    }
                },
                lay![
                    margin: [10., 0., 0., 0.]
                ],
            )),
        );
        row_3 = row_3.push(node!(VDivider { size: 1. }));
        row_3 = row_3.push(
            node!(
                Div::new(),
                lay![
                    size: [145, Auto],
                    direction: Direction::Column,
                    padding: [14., 12., 10., 12.]
                ]
            )
            .push(node!(CPU { usages: cpu_usage }, lay![])),
        );
        row_3 = row_3.push(node!(VDivider { size: 1. }));
        row_3 = row_3.push(
            node!(
                Div::new(),
                lay![
                    size: [145, Auto],
                    direction: Direction::Column,
                    padding: [14., 12., 10., 12.]
                ]
            )
            .push(node!(Memory::new(used_memory), lay![])),
        );

        let mut row_4 = node!(
            Div::new(),
            lay![
                size: [Auto, 114],
                direction: Direction::Row,
                cross_alignment: Alignment::Center
            ]
        );

        for (i, app) in self.settings.modules.apps.clone().into_iter().enumerate() {
            row_4 = row_4.push(
                node!(
                    PinnedApp::new(app.app_id.clone(), app.icon.unwrap()).on_click(Box::new(
                        move || msg!(Message::AppClicked {
                            app_id: app.app_id.clone()
                        })
                    ))
                )
                .key(i as u64),
            );
        }

        // start_node = start_node.push(node!(
        //     Div::new().bg(Color::BLUE),
        //     lay![
        //         position_type: Absolute,
        //         size: [100, 100]
        //         max_size: [100, 100],
        //         position: [Auto, -80., Auto, Auto],
        //         z_index_increment: 1000.
        //     ]
        // ));

        // if let Some(swipe) = self.swipe_gesture.clone() {
        //     match swipe {
        //         SwipeGestures::Down(_) => {}
        //         SwipeGestures::Up(_) => {}
        //         SwipeGestures::Right(val) => {
        //             println!("update val {:?}", -480 + val);
        //             start_node = start_node.push(
        //                 node!(
        //                     Div::new().bg(Color::BLUE).swipe(-480 + val),
        //                     lay![
        //                         position_type: Absolute,
        //                         size: [480, 480]
        //                         position: [Auto, -480 + val, Auto, Auto],
        //                         z_index_increment: 1000.
        //                     ]
        //                 )
        //                 .push(node!(
        //                     Text::new(txt!("hello"))
        //                         .style("color", Color::rgb(230., 230., 230.))
        //                         .style("size", 72.0)
        //                         .style("font", "SpaceMono-Bold")
        //                         .style("font_weight", FontWeight::Bold),
        //                     lay![]
        //                 ))
        //                 .key(val as u64),
        //             );
        //         }
        //         SwipeGestures::Left(_) => {}
        //     }
        // }
        start_node = start_node.push(node!(HDivider { size: 1. }));
        start_node = start_node.push(row_1);
        start_node = start_node.push(node!(
            HDivider { size: 1. },
            lay! [ margin: [10., 0., 0., 0.] ]
        ));

        if !self.is_lock_screen {
            start_node = start_node.push(row_2);
            start_node = start_node.push(node!(HDivider { size: 1. }));
            start_node = start_node.push(row_3);
        } else {
            start_node = start_node.push(node!(Div::new(), lay![size:[Auto, 204]]));
        }

        start_node = start_node.push(node!(HDivider { size: 1. }));
        if !self.is_lock_screen {
            start_node = start_node.push(row_4);
        } else {
            start_node = start_node.push(node!(
                Slider::new()
                    .slider_type(SliderType::Box)
                    .on_slide_end(Box::new(|value| {
                        if value > 90 {
                            msg!(gui::Message::Unlock)
                        } else {
                            msg!("none")
                        }
                    }))
                    .fill_random_on_start(true)
                    .fill_random_on_slide(true)
                    .reset_on_slide_end(true)
                    .col_spacing(6.25)
                    .row_spacing(7.25)
                    .col_width(12.)
                    .active_color(Color::rgb(255., 255., 255.)),
                lay![size: [Auto, 98], margin:[10., 0., 0., 0.]]
            ))
        }
        start_node = start_node.push(node!(HDivider { size: 0.8 }));

        Some(start_node)
    }
}
