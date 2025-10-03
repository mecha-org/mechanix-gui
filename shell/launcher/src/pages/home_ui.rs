use std::collections::VecDeque;

use crate::gui::{self, Message};
use crate::modules::applications::model::DesktopEntriesModel;
use crate::modules::clock::component::Clock;
use crate::modules::clock::model::ClockModel;
use crate::modules::controls::Controls;
use crate::modules::cpu::component::CPU;
use crate::modules::ip_address::component::IpAddress;
use crate::modules::memory::component::Memory;
use crate::modules::name::component::MachineName;
use crate::modules::networking::component::Networking;
use crate::modules::pinned_app::PinnedApp;
use crate::modules::uptime::component::Uptime;
use crate::modules::weather::component::Weather;
use crate::settings::LauncherSettings;
use crate::shared::h_divider::{self, HDivider};
use crate::shared::v_divider::VDivider;
use crate::types::{BatteryLevel, BluetoothStatus, WirelessStatus};
use desktop_entries::DesktopEntry;
use mctk_core::layout::{Alignment, Dimension, Direction, PositionType, Size};
use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::{IconButton, IconType, Image, Svg, Text};
use mctk_core::{
    component::Component,
    lay, node, rect, size, size_pct,
    widgets::{Div, SlideBar, SlideBarType},
    Node,
};
use mctk_core::{msg, txt, Color};

#[derive(Debug, Default)]
pub struct HomeUi {
    pub settings: LauncherSettings,
    pub bluetooth_status: BluetoothStatus,
    pub cpu_usage: VecDeque<u8>,
    pub uptime: String,
    pub machine_name: String,
    pub online: bool,
    pub used_memory: u64,
    pub is_lock_screen: bool,
    pub disable_activity: bool,
    pub pinned_apps: Vec<DesktopEntry>,
}

impl Component for HomeUi {
    fn init(&mut self) {
        DesktopEntriesModel::run();
    }

    fn view(&self) -> Option<Node> {
        let cpu_usage = self.cpu_usage.clone();
        let uptime = self.uptime.clone();
        let machine_name = self.machine_name.clone();
        let online = self.online.clone();
        let used_memory = self.used_memory;
        let bluetooth_status = self.bluetooth_status.clone();
        let is_lock_screen = self.is_lock_screen;
        let disable_activity = self.disable_activity;

        let avaiable_memory = ((100. - used_memory as f32) / 100.) * 4.;
        let avaiable_memory_rounded = (avaiable_memory * 10.0).round() / 10.0;

        let mut start_node = node!(
            Div::new().bg(Color::rgba(0., 0., 0., 0.64)),
            lay![
                size_pct: [100],
                // size: [480, 434]
                // padding: [8., 16., 0., 16.],
                cross_alignment: Alignment::Stretch,
                direction: Direction::Column
            ]
        );

        start_node = start_node
            .push(node!(
                Text::new(txt!("Linux phones"))
                    .with_class("font-space-mono")
                    .style("color", Color::rgb(233., 233., 233.))
                    .style("size", 18.0)
                    .style("font_weight", FontWeight::Normal),
                lay![
                    size: [200, 28],
                    position_type: PositionType::Absolute,
                    position: [14., 32., Auto, Auto], // top 14, left 32
                ]
            ))
            .push(node!(
                Text::new(txt!("are more important"))
                    .with_class("font-space-mono")
                    .style("color", Color::rgb(233., 233., 233.))
                    .style("size", 18.0)
                    .style("font_weight", FontWeight::Normal),
                lay![
                    size: [200, 28],
                    position_type: PositionType::Absolute,
                    position: [38., 32., Auto, Auto], // 14 + 24 = 38
                ]
            ))
            .push(node!(
                Text::new(txt!("now than ever"))
                    .with_class("font-space-mono")
                    .style("color", Color::rgb(233., 233., 233.))
                    .style("size", 18.0)
                    .style("font_weight", FontWeight::Normal),
                lay![
                    size: [200, 28],
                    position_type: PositionType::Absolute,
                    position: [62., 32., Auto, Auto], // 38 + 24 = 62
                ]
            ))
            .push(node!(
                Text::new(txt!("Hackernews"))
                    .with_class("font-space-mono")
                    .style("color", Color::rgb(255., 102., 0.))
                    .style("size", 16.0)
                    .style("font_weight", FontWeight::Bold),
                lay![
                    size: [28, 28],
                    position_type: PositionType::Absolute,
                    position: [222., 70., Auto, Auto],
                ]
            ))
            .push(node!(
                Image::new("hacker_news_icon"),
                lay![
                    size: [28, 28],
                    position_type: PositionType::Absolute,
                    position: [218., 32., Auto, Auto],
                ]
            ))
            .push(node!(
                Svg::new("link"),
                lay![
                    size: [28, 28],
                    position_type: PositionType::Absolute,
                    position: [220., 230., Auto, Auto],
                ]
            ))
            .push(node!(
                IconButton::new("")
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(248.0),
                            height: Dimension::Px(249.0),
                        },
                    )
                    .style("padding", 0.)
                    .style("background_color", Color::rgb(51., 51., 51.))
                    .style("active_color", Color::rgb(24., 24., 24.))
                    .style("radius", 8.)
                    .on_click(Box::new(move || Box::new(Message::AppOpen {
                        app_id: "hackernews".to_string(),
                        layer: None
                    }))),
                lay![
                    size: size!(248., 249.),
                    direction: Direction::Row,
                    position_type: PositionType::Absolute,
                    position: [8., 16., Auto, Auto]
                ],
            ))
            .push(
                node!(
                    Image::new(format!("memory_bg")),
                    lay![
                        size: [250, 274],
                        position_type: PositionType::Absolute,
                        position: [8., Auto, Auto, 16.]
                    ]
                )
                .push(node!(
                    Text::new(txt!("Memory"),)
                        .with_class("font-space-grotesk")
                        .style("font_weight", FontWeight::Normal)
                        .style("color", Color::rgb(244., 244., 244.))
                        .style("size", 16.0),
                    lay![
                        position_type: PositionType::Absolute,
                        position: [8., 12., 0., 0.]
                    ]
                ))
                .push(
                    node!(
                        Div::new().border(Color::rgb(32., 32., 32.), 0.8, (0., 0., 0., 0.)),
                        lay![
                            direction: Direction::Row,
                            position_type: PositionType::Absolute,
                            position: [35., 14., Auto, Auto],
                            size: [230, Auto]
                        ]
                    )
                    .push(node!(
                        Div::new(),
                        lay![
                            size: [ Auto, 0.8 ]
                        ]
                    )),
                )
                .push(node!(
                    Memory::new(used_memory),
                    lay![
                        position_type: PositionType::Absolute,
                        position: [35., 24., 0., 0.]
                    ]
                ))
                .push(node!(
                    Text::new(txt!(format!("{}GB", avaiable_memory_rounded)),)
                        .with_class("font-space-grotesk")
                        .style("font_weight", FontWeight::Medium)
                        .style("color", Color::rgb(233., 233., 233.))
                        .style("size", 19.0),
                    lay![
                        position_type: PositionType::Absolute,
                        position: [183., 24., 0., 0.]
                    ]
                ))
                .push(node!(
                    Text::new(txt!("Available"),)
                        .with_class("font-space-grotesk")
                        .style("font_weight", FontWeight::Normal)
                        .style("color", Color::rgb(121., 121., 121.))
                        .style("size", 12.0),
                    lay![
                        position_type: PositionType::Absolute,
                        position: [204., 24., 0., 0.]
                    ]
                ))
                .push(node!(
                    Text::new(txt!("4GB"),)
                        .with_class("font-space-grotesk")
                        .style("font_weight", FontWeight::Medium)
                        .style("color", Color::rgb(233., 233., 233.))
                        .style("size", 19.0),
                    lay![
                        position_type: PositionType::Absolute,
                        position: [225., 24., 0., 0.]
                    ]
                ))
                .push(node!(
                    Text::new(txt!("Total"),)
                        .with_class("font-space-grotesk")
                        .style("font_weight", FontWeight::Normal)
                        .style("color", Color::rgb(121., 121., 121.))
                        .style("size", 12.0),
                    lay![
                        position_type: PositionType::Absolute,
                        position: [246., 24., 0., 0.]
                    ]
                )),
            );

        start_node = start_node.push(
            node!(
                Image::new(format!("app_list_bg")),
                lay![
                    size: [508, 250],
                    position_type: PositionType::Absolute,
                    position: [266., 16., 0., 0.]
                ]
            )
            .push(node!(
                Text::new(txt!("Apps installed"),)
                    .with_class("font-space-grotesk")
                    .style("font_weight", FontWeight::Normal)
                    .style("color", Color::rgb(166., 166., 166.))
                    .style("size", 16.0),
                lay![
                    position_type: PositionType::Absolute,
                    position: [8., 12., 0., 0.]
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        position_type: PositionType::Absolute,
                        position: [40., 28., 0., 0.],
                        size: [452, 80],
                        margin: [0, 0, 26, 0]
                    ]
                )
                .push(node!(
                    Image::new("files"),
                    lay![
                        size: [80, 80],
                        position_type: PositionType::Absolute,
                        position: [0., 0., 0., 0.],
                    ]
                ))
                .push(node!(
                    IconButton::new("")
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(80.0),
                                height: Dimension::Px(80.0),
                            },
                        )
                        .style("padding", 0.)
                        .style("background_color", Color::rgb(72., 146., 241.))
                        .style("active_color", Color::rgb(96., 156., 234.))
                        .style("radius", 8.)
                        .on_click(Box::new(move || Box::new(Message::AppOpen {
                            app_id: "mechanix_files".to_string(),
                            layer: None
                        }))),
                    lay![
                        size: size!(80., 80.),
                        direction: Direction::Row,
                        margin: [0., 0., 0., 44.],
                    ],
                ))
                .push(node!(
                    Image::new("notes"),
                    lay![
                        size: [80, 80],
                        position_type: PositionType::Absolute,
                        position: [0., 124., 0., 0.],
                    ]
                ))
                .push(node!(
                    IconButton::new("")
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(80.0),
                                height: Dimension::Px(80.0),
                            },
                        )
                        .style("padding", 0.)
                        .style("background_color", Color::rgb(255., 195., 90.))
                        .style("active_color", Color::rgb(255., 216., 148.))
                        .style("radius", 8.)
                        .on_click(Box::new(move || Box::new(Message::AppOpen {
                            app_id: "mechanix_notes".to_string(),
                            layer: None
                        }))),
                    lay![
                        size: size!(80., 80.),
                        direction: Direction::Row,
                        margin: [0., 0., 0., 44.],
                    ],
                ))
                .push(node!(
                    Image::new("music"),
                    lay![
                        size: [80, 80],
                        position_type: PositionType::Absolute,
                        position: [0., 248., 0., 0.],
                    ]
                ))
                .push(node!(
                    IconButton::new("")
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(80.0),
                                height: Dimension::Px(80.0),
                            },
                        )
                        .style("padding", 0.)
                        .style("background_color", Color::rgb(255., 114., 116.))
                        .style("active_color", Color::rgb(255., 163., 165.))
                        .style("radius", 8.)
                        .on_click(Box::new(move || Box::new(Message::AppOpen {
                            app_id: "mechanix_music".to_string(),
                            layer: None
                        }))),
                    lay![
                        size: size!(80., 80.),
                        direction: Direction::Row,
                        margin: [0., 0., 0., 44.],
                    ],
                ))
                .push(node!(
                    Image::new("settings"),
                    lay![
                        size: [80, 80],
                        position_type: PositionType::Absolute,
                        position: [0., 372., 0., 0.],
                    ]
                ))
                .push(node!(
                    IconButton::new("")
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(80.0),
                                height: Dimension::Px(80.0),
                            },
                        )
                        .style("padding", 0.)
                        .style("background_color", Color::rgb(11., 102., 222.))
                        .style("active_color", Color::rgb(96., 156., 234.))
                        .style("radius", 8.)
                        .on_click(Box::new(move || Box::new(Message::AppOpen {
                            app_id: "mechanix_settings".to_string(),
                            layer: None
                        }))),
                    lay![
                        size: size!(80., 80.),
                        direction: Direction::Row,
                    ],
                )),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        position_type: PositionType::Absolute,
                        position: [146., 28., 0., 0.],
                        size: [452, 80],
                    ]
                )
                .push(node!(
                    Image::new("terminal"),
                    lay![
                        size: [80, 80],
                        position_type: PositionType::Absolute,
                        position: [0., 0., 0., 0.],
                    ]
                ))
                .push(node!(
                    IconButton::new("")
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(80.0),
                                height: Dimension::Px(80.0),
                            },
                        )
                        .style("padding", 0.)
                        .style("background_color", Color::rgb(11., 102., 222.))
                        .style("active_color", Color::rgb(96., 156., 234.))
                        .style("radius", 8.)
                        .on_click(Box::new(move || Box::new(Message::AppOpen {
                            app_id: "alacritty".to_string(),
                            layer: None
                        }))),
                    lay![
                        size: size!(80., 80.),
                        direction: Direction::Row,
                        margin: [0., 0., 0., 44.],
                    ],
                ))
                .push(node!(
                    Image::new("chromium"),
                    lay![
                        size: [80, 80],
                        position_type: PositionType::Absolute,
                        position: [0., 124., 0., 0.],
                    ]
                ))
                .push(node!(
                    IconButton::new("")
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(80.0),
                                height: Dimension::Px(80.0),
                            },
                        )
                        .style("padding", 0.)
                        .style("background_color", Color::rgb(11., 102., 222.))
                        .style("active_color", Color::rgb(96., 156., 234.))
                        .style("radius", 8.)
                        .on_click(Box::new(move || Box::new(Message::AppOpen {
                            app_id: "chromium".to_string(),
                            layer: None
                        }))),
                    lay![
                        size: size!(80., 80.),
                        direction: Direction::Row,
                        margin: [0., 0., 0., 44.],
                    ],
                ))
                .push(node!(
                    Image::new("firefox"),
                    lay![
                        size: [80, 80],
                        position_type: PositionType::Absolute,
                        position: [0., 248., 0., 0.],
                    ]
                ))
                .push(node!(
                    IconButton::new("")
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(80.0),
                                height: Dimension::Px(80.0),
                            },
                        )
                        .style("padding", 0.)
                        .style("background_color", Color::rgb(11., 102., 222.))
                        .style("active_color", Color::rgb(96., 156., 234.))
                        .style("radius", 8.)
                        .on_click(Box::new(move || Box::new(Message::AppOpen {
                            app_id: "firefox".to_string(),
                            layer: None
                        }))),
                    lay![
                        size: size!(80., 80.),
                        direction: Direction::Row,
                        margin: [0., 0., 0., 44.],
                    ],
                )),
            ),
        );

        Some(start_node)
    }
}

fn divider(size: f32) -> Node {
    node!(
        Div::new().border(Color::rgb(132., 132., 132.), size, (0., 0., 0., 0.)),
        lay![
            direction: Direction::Row,
            size_pct: [100, Auto],
            cross_alignment: Alignment::Stretch
        ]
    )
    .push(node!(
        Div::new(),
        lay![
            size: [ Auto, 1 ]
        ]
    ))
}
