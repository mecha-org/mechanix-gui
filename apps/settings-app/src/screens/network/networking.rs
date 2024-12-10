use super::component::NetworkRowComponent;
use super::wireless_model::WirelessModel;
use crate::AppMessage;
use crate::{
    components::{header_node, text_node, ComponentHasher},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};
use std::hash::Hash;

use mctk_core::event;
use mctk_core::reexports::femtovg::img::save_buffer;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::Image;
use mctk_core::widgets::Scrollable;
use mctk_core::{
    component::{self, Component},
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{self, Div, IconButton, IconType, Text, Toggle},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};

use mechanix_system_dbus_client::wireless::WirelessInfoResponse;
use zbus::message;

enum NetworkingMessage {
    handleClickOnMore,
    handleClickOnBack,
}

pub struct ClicableIconComponent {
    pub on_click: Option<Box<dyn Fn() -> Box<Message> + Send + Sync>>,
}

impl std::fmt::Debug for ClicableIconComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClicableIconComponent").finish()
    }
}

fn truncate(s: String, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

impl Component for ClicableIconComponent {
    fn on_click(&mut self, event: &mut event::Event<event::Click>) {
        if let Some(f) = &self.on_click {
            event.emit(f());
        }
    }

    fn container(&self) -> Option<Vec<usize>> {
        Some(vec![0])
    }

    fn view(&self) -> Option<Node> {
        let base = node!(
            Div::new(),
            lay![
                size_pct: [80, Auto],
                axis_alignment: Alignment::Start,
            ]
        );
        Some(base)
    }
}

#[derive(Debug)]
pub struct NetworkScreenState {
    // pub loading: bool,
    pub status: bool,
    pub name: String,
    pub more_clicked: bool,
    // pub app_channel: Option<Sender<AppMessage>>,
    // pub connected_network: Option<WirelessDetailsItem>,
}

#[derive(Debug)]
#[component(State = "NetworkScreenState")]
pub struct NetworkingScreen {}

impl NetworkingScreen {
    // pub fn new(status: bool, connected_network: Option<WirelessDetailsItem>) -> Self {
    pub fn new(status: bool, name: String) -> Self {
        Self {
            state: Some(NetworkScreenState {
                status,
                name,
                more_clicked: false,
                // connected_network: connected_network,
            }),
            dirty: false,
        }
    }
}

impl Component for NetworkingScreen {
    fn init(&mut self) {
        WirelessModel::start_streaming();
        WirelessModel::update();
        WirelessModel::scan();
    }

    fn render_hash(&self, hasher: &mut ComponentHasher) {
        WirelessModel::get()
            .scan_result
            .get()
            .wireless_network
            .len()
            .hash(hasher);

        WirelessModel::get()
            .known_networks
            .get()
            .known_network
            .len()
            .hash(hasher);

        if WirelessModel::get()
            .connected_network
            .get()
            .clone()
            .is_some()
        {
            1_i32.hash(hasher);
        } else {
            0_i32.hash(hasher);
        }

        self.props_hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let status: bool = *WirelessModel::get().is_enabled.get();

        let mut base: Node = node!(
            Div::new(),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let text_node = node!(
            Text::new(txt!("Networking"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 20.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, Auto],
                margin:[2.0, 5.0, 2.0, 5.0],
            ]
        );

        let header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 10],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                margin: [0., 0., 5., 0.],
                // position_type: Absolute,
                position: [0., 0., Auto, 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [60, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::SettingsList
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    padding: [0, 0, 0, 20.],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            ))
            .push(text_node),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [40, Auto],
                    axis_alignment: Alignment::End,
                    padding: [0, 0, 0, 0.],
                ]
            )
            .push(node!(
                IconButton::new("add_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::AddNetwork {
                                ssid: "".to_string()
                            }
                        }
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(40.0),
                            height: Dimension::Px(40.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            ))
            .push(node!(
                IconButton::new("wireless_settings")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkSettings
                        }
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        // toggle row
        let toggle_row = node!(
            Div::new(),
            lay![
                size: [480, 50],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment:Alignment::Center,
                padding: [5., 0., 15., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size: [350, 50],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Text::new(txt!("Wireless"))
                    .style("color", Color::WHITE)
                    .style("size", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new().bg(Color::TRANSPARENT),
                lay![
                    size_pct: [20, 40],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Toggle::new(status).on_change(Box::new(|value| {
                    WirelessModel::toggle_wireless();
                    Box::new(())
                })),
                lay![]
            )),
        );

        // let toggle_node = node!(
        //     Div::new(),
        //     lay![
        //         size: [350, 50],
        //         direction: Direction::Column,
        //         cross_alignment: Alignment::Stretch,
        //     ]
        // )
        // .push(toggle_row);

        let mut connected_network_name = "    ".to_string();
        if let Some(connected_network) = WirelessModel::get().connected_network.get().clone() {
            connected_network_name = connected_network.name.clone();
        }
        connected_network_name = truncate(connected_network_name, 30);

        let connected_status = match *WirelessModel::get().state.get() {
            super::wireless_model::WifiState::Connecting => "Connecting...",
            super::wireless_model::WifiState::Connected => "Connected",
            _ => "Disconnected",
        };

        let connected_network_row = node!(
            Div::new(),
            lay![
                size: [440, 50],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                // padding: [5., 0., 12., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                widgets::Image::new("wireless_good"),
                lay![
                    size: [24, 24],
                    margin:[0., 0., 0., 20.],
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, Auto],
                        direction: Direction::Column,
                        axis_alignment: Alignment::Stretch,
                    ]
                )
                .push(node!(
                    Text::new(txt!(connected_network_name.clone()))
                        .style("color", Color::WHITE)
                        .style("size", 18.0)
                        .style("line_height", 20.0)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                ))
                .push(node!(
                    // mini status
                    Text::new(txt!(connected_status))
                        .style("color", Color::WHITE)
                        .style("size", 14.0)
                        .style("line_height", 18.)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            ),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End,
                    cross_alignment:Alignment::Center,
                    padding: [0. , 0., 0., 10.]
                ]
            )
            .push(node!(
                IconButton::new("info_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkDetails
                        }
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        let mut saved_available_networks = vec![];
        let mut unsaved_available_networks = vec![];
        let available_networks = WirelessModel::get()
            .scan_result
            .get()
            .wireless_network
            .clone();

        let known_networks = WirelessModel::get()
            .known_networks
            .get()
            .known_network
            .clone();

        for network in available_networks {
            let mut is_known = false;
            let mut is_current = false;
            let mut network_id = "".to_string();
            for known_network in known_networks.iter() {
                if known_network.ssid == network.name {
                    if known_network.flags.contains("[CURRENT]") {
                        is_current = true;
                    }
                    network_id = known_network.network_id.clone();
                    is_known = true;
                    break;
                }
            }
            if connected_network_name.clone() == network.name {
                is_current = true;
            }
            if is_current {
                continue;
            }
            if is_known {
                saved_available_networks.push((network, network_id));
            } else {
                unsaved_available_networks.push(network);
            }
        }

        let available_network_text = node!(
            Text::new(txt!("Available Networks"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 16.0)
                .style("line_height", 18.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [30.0, 0.0, 10.0, 0.0],
            ]
        );

        let saved_network_row_component = |network: WirelessInfoResponse| {
            let ssid = network.name.clone();
            let mut icon = if network.flags.contains("WPA") {
                "secured_wireless_strong".to_string()
            } else {
                "wireless_strong".to_string()
            };
            if let Ok(signal) = network.signal.parse::<u32>() {
                if signal < 70 {
                    icon = icon.replace("strong", "weak");
                } else if signal < 30 {
                    icon = icon.replace("strong", "low");
                }
            }
            node!(
                Div::new(),
                lay![
                    size: [440, 50],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                    // padding: [5., 0., 12., 0.],
                ],
            )
            .push(
                node!(ClicableIconComponent {
                    on_click: Some(Box::new(move || {
                        WirelessModel::connect_to_saved_network(ssid.clone());
                        msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::Networking
                            }
                        })
                    }))
                })
                .push(node!(
                    widgets::Image::new(icon),
                    lay![
                        size: [24, 24],
                        margin:[0., 0., 0., 20.],
                    ]
                ))
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [100, Auto],
                            direction: Direction::Column,
                            axis_alignment: Alignment::Stretch,
                        ]
                    )
                    .push(node!(
                        Text::new(txt!(truncate(network.name.clone(), 30)))
                            .style("color", Color::WHITE)
                            .style("size", 18.0)
                            .style("line_height", 20.0)
                            .style("font", "Space Grotesk")
                            .style("font_weight", FontWeight::Normal),
                        lay![
                            direction: Direction::Row,
                            axis_alignment: Alignment::Start,
                            cross_alignment: Alignment::Center,
                        ]
                    ))
                    .push(node!(
                        // mini status
                        Text::new(txt!("Saved"))
                            .style("color", Color::WHITE)
                            .style("size", 14.0)
                            .style("line_height", 18.)
                            .style("font", "Space Grotesk")
                            .style("font_weight", FontWeight::Normal),
                        lay![
                            direction: Direction::Row,
                            axis_alignment: Alignment::Start,
                            cross_alignment: Alignment::Center,
                        ]
                    )),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [20, Auto],
                        axis_alignment: Alignment::End,
                        cross_alignment:Alignment::Center,
                        padding: [0. , 0., 0., 10.]
                    ]
                )
                .push(node!(
                    IconButton::new("info_icon")
                        .on_click(Box::new(move || msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::SavedNetworkDetails {
                                    mac: network.mac.clone()
                                }
                            }
                        })))
                        .icon_type(IconType::Png)
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(34.0),
                                height: Dimension::Px(34.0),
                            }
                        )
                        .style("background_color", Color::TRANSPARENT)
                        .style("border_color", Color::TRANSPARENT)
                        .style("active_color", Color::rgba(85., 85., 85., 0.50))
                        .style("radius", 10.),
                    lay![
                        size: [52, 52],
                        axis_alignment: Alignment::End,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            )
        };

        let unsaved_available_network_row_component = |network: WirelessInfoResponse| {
            let ssid = network.name.clone();
            let icon = if network.flags.contains("WPA") {
                "secured_wireless_strong".to_string()
            } else {
                "wireless_strong".to_string()
            };
            node!(
                Div::new(),
                lay![
                    size: [440, 50],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Center,
                    // padding: [5., 0., 12., 0.],
                ]
            )
            .push(
                node!(ClicableIconComponent {
                    on_click: Some(Box::new(move || msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::AddNetwork { ssid: ssid.clone() }
                        }
                    })))
                },)
                .push(node!(
                    widgets::Image::new(icon),
                    lay![
                        size: [24, 24],
                        margin:[0., 0., 0., 20.],
                    ]
                ))
                .push(
                    node!(
                        Div::new(),
                        lay![
                            size_pct: [100, Auto],
                            direction: Direction::Column,
                            axis_alignment: Alignment::Stretch,
                        ]
                    )
                    .push(node!(
                        Text::new(txt!(truncate(network.name.clone(), 28)))
                            .style("color", Color::WHITE)
                            .style("size", 18.0)
                            .style("line_height", 20.0)
                            .style("font", "Space Grotesk")
                            .style("font_weight", FontWeight::Normal),
                        lay![
                            direction: Direction::Row,
                            axis_alignment: Alignment::Start,
                        ]
                    )),
                ),
            )
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [20, Auto],
                        axis_alignment: Alignment::End,
                        cross_alignment:Alignment::Center,
                        padding: [0. , 0., 0., 10.]
                    ]
                )
                .push(node!(
                    IconButton::new("info_icon")
                        .on_click(Box::new(move || msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::UnknownNetworkDetails {
                                    mac: network.mac.clone()
                                }
                            }
                        })))
                        .icon_type(IconType::Png)
                        .style(
                            "size",
                            Size {
                                width: Dimension::Px(34.0),
                                height: Dimension::Px(34.0),
                            }
                        )
                        .style("background_color", Color::TRANSPARENT)
                        .style("border_color", Color::TRANSPARENT)
                        .style("active_color", Color::rgba(85., 85., 85., 0.50))
                        .style("radius", 10.),
                    lay![
                        size: [52, 52],
                        axis_alignment: Alignment::End,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            )
        };

        let available_network_row_3 = node!(
            Div::new(),
            lay![
                size_pct: [100, 12],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                // padding: [5., 0., 12., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                widgets::Image::new("wireless_good"),
                lay![
                    size: [24, 24],
                    margin:[0., 0., 0., 20.],
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [100, Auto],
                        direction: Direction::Column,
                        axis_alignment: Alignment::Stretch,
                    ]
                )
                .push(node!(
                    Text::new(txt!("Mecha Guest"))
                        .style("color", Color::WHITE)
                        .style("size", 18.0)
                        .style("line_height", 20.0)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            ),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End,
                    cross_alignment:Alignment::Center,
                    padding: [0. , 0., 0., 10.]
                ]
            )
            .push(node!(
                IconButton::new("info_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::NetworkDetails
                        }
                    })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        let view_all_text: Node = node!(
            Text::new(txt!("view all"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 10.0)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [2.0, 0.0, 2.0, 0.0],
                axis_alignment: Alignment::Start
            ]
        );

        // // Advanced
        let advanced_nextwork_text = node!(
            Text::new(txt!("Advanced"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 16.0)
                .style("line_height", 18.)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                margin: [20.0, 0.0, 10.0, 0.0],
            ]
        );

        let advanced_network_row = node!(
            Div::new(),
            lay![
                size: [440, 50],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                // padding: [5., 0., 12., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            )
            .push(node!(
                Text::new(txt!("Add Network"))
                    .style("color", Color::rgba(45., 138., 225., 1.))
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    direction: Direction::Row,
                    axis_alignment: Alignment::Start,
                ]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End,
                    padding: [0. , 0., 0., 5.]
                ]
            )
            .push(node!(
                IconButton::new("right_arrow_icon")
                    // .on_click(Box::new(|| msg!(Message::ChangeRoute {
                    //     route: Routes::Network {
                    //         screen: NetworkScreenRoutes::AddNetwork
                    //     }
                    // })))
                    .icon_type(IconType::Png)
                    .style(
                        "size",
                        Size {
                            width: Dimension::Px(34.0),
                            height: Dimension::Px(34.0),
                        }
                    )
                    .style("background_color", Color::TRANSPARENT)
                    .style("border_color", Color::TRANSPARENT)
                    .style("active_color", Color::rgba(85., 85., 85., 0.50))
                    .style("radius", 10.),
                lay![
                    size: [52, 52],
                    axis_alignment: Alignment::End,
                    cross_alignment: Alignment::Center,
                ]
            )),
        );

        content_node = content_node.push(toggle_row);
        content_node = content_node.push(node!(HDivider { size: 1. }));
        // content_node = content_node.push(node!(HDivider { size: 1. }));

        // content_node = content_node.push(available_network_text);
        //
        // content_node = content_node.push(node!(HDivider { size: 1. }));
        let mut scrollable_section = node!(
            Scrollable::new(),
            lay![
                size: [440, 320],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        ));

        if WirelessModel::get().connected_network.get().is_some() && status.clone() == true {
            scrollable_section = scrollable_section.push(connected_network_row);
        }
        let mut key = 0;

        scrollable_section = scrollable_section.push(
            node!(
                Div::new().border(Color::rgb(132., 132., 132.), 0.5, (0., 0., 0., 0.)),
                lay![
                    direction: Direction::Row,
                    size: [480, Auto],
                    cross_alignment: Alignment::Stretch
                ]
            )
            .push(node!(
                Div::new(),
                lay![
                    size: [ 480, 1 ]
                ]
            )),
        );
        for (network, network_id) in saved_available_networks.iter() {
            if network.name.clone().len() > 0 {
                scrollable_section =
                    scrollable_section.push(saved_network_row_component(network.clone()).key(key));
                key += 1;
                scrollable_section = scrollable_section
                    .push(
                        node!(
                            Div::new().border(Color::rgb(132., 132., 132.), 0.5, (0., 0., 0., 0.)),
                            lay![
                                direction: Direction::Row,
                                size: [480, Auto],
                                cross_alignment: Alignment::Stretch
                            ]
                        )
                        .push(node!(
                            Div::new(),
                            lay![
                                size: [ 480, 1 ]
                            ]
                        )),
                    )
                    .key(key);
                key += 1;
            }
        }
        for network in unsaved_available_networks.iter() {
            if network.name.clone().len() > 0 {
                key += 1;

                scrollable_section = scrollable_section
                    .push(unsaved_available_network_row_component(network.clone()).key(key));
                scrollable_section = scrollable_section
                    .push(
                        node!(
                            Div::new().border(Color::rgb(132., 132., 132.), 0.5, (0., 0., 0., 0.)),
                            lay![
                                direction: Direction::Row,
                                size: [480, Auto],
                                cross_alignment: Alignment::Stretch
                            ]
                        )
                        .push(node!(
                            Div::new(),
                            lay![
                                size: [ 480, 1 ]
                            ]
                        )),
                    )
                    .key(key);
            }
        }
        content_node = content_node.push(scrollable_section);
        content_node = content_node.push(node!(HDivider { size: 1. }));

        // content_node = content_node.push(view_all_text);
        // content_node = content_node.push(advanced_nextwork_text);

        // content_node = content_node.push(node!(HDivider { size: 1. }));
        // content_node = content_node.push(advanced_network_row);
        // content_node = content_node.push(node!(HDivider { size: 1. }));

        base = base.push(header_node);
        base = base.push(content_node);
        // base = base.push(node!(Scrollable::new(), lay![size: [440, 380]]).push(content_node));
        Some(base)
    }
}
