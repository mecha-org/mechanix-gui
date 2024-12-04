use super::component::NetworkRowComponent;
use super::wireless_model::WirelessModel;
use crate::AppMessage;
use crate::{
    components::{header_node, text_node},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};

use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::Image;
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

use mechanix_status_bar_components::types::WirelessStatus;
use mechanix_system_dbus_client::wireless::WirelessInfoResponse;
use zbus::message;

enum NetworkingMessage {
    handleClickOnMore,
    handleClickOnBack,
}

#[derive(Debug)]
pub struct NetworkDetailsState {
    // pub loading: bool,
    // list
}

#[derive(Debug)]
// #[component(State = "NetworkDetailsState")]
pub struct UnknownNetworkDetails {
    mac: String,
}

impl UnknownNetworkDetails {
    pub fn new(mac: String) -> Self {
        Self { mac }
    }

    fn get_ip_address(&self) -> Option<String> {
        let networks = sysinfo::Networks::new_with_refreshed_list();
        for (interface, info) in &networks {
            if interface.starts_with("wl") {
                for network in info.ip_networks().iter() {
                    if network.addr.is_ipv4() {
                        return Some(network.addr.to_string());
                    }
                }
            }
        }
        None
    }
}

impl Component for UnknownNetworkDetails {
    fn init(&mut self) {
        WirelessModel::update();
    }

    fn view(&self) -> Option<Node> {
        let ip_address = if let Some(ip_address) = self.get_ip_address() {
            ip_address
        } else {
            "-".to_string()
        };
        let mut text_color = Color::WHITE;
        let connected_network_option = WirelessModel::get()
            .scan_result
            .get()
            .wireless_network
            .clone()
            .into_iter()
            .find(|network| network.mac == self.mac)
            .clone();
        let mut security = "-";
        let mut signal_strength = "-";
        let connected_network = match connected_network_option {
            Some(connected_network_option) => {
                for security_match in &["WPA-PSK", "WPA2-PSK", "WPA3-PSK"] {
                    if connected_network_option.flags.contains(security_match) {
                        security = security_match;
                        break;
                    }
                }
                if let Ok(signal_int) = connected_network_option.signal.parse::<i32>() {
                    if signal_int < 0_i32 && signal_int > -30_i32 {
                        signal_strength = "Excellent";
                    } else if signal_int <= -30 && signal_int > -70 {
                        signal_strength = "Good";
                    } else if signal_int <= -70 {
                        signal_strength = "Weak";
                    }
                };
                connected_network_option
            }
            None => WirelessInfoResponse {
                name: "-".to_string(),
                mac: "-".to_string(),
                flags: "-".to_string(),
                frequency: "-".to_string(),
                signal: "-".to_string(),
            },
        };

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
            Text::new(txt!("Network Details"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 17.5)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, Auto],
            ]
        );

        // // TODO:
        // 1. show selected network name in header; for long name , add suffix ".."
        // 2. show forget-(delete_icon) only when it is saved network
        let header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 10],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                margin: [0., 0., 5., 0.],
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [80, Auto],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ],
            )
            .push(node!(
                IconButton::new("back_icon")
                    .on_click(Box::new(|| msg!(Message::ChangeRoute {
                        route: Routes::Network {
                            screen: NetworkScreenRoutes::Networking
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
                    padding: [0, 0, 0, 20.],
                    axis_alignment: Alignment::Start,
                    cross_alignment: Alignment::Center,
                ]
            ))
            .push(text_node),
        )
        .push(node!(
            Div::new(),
            lay![
                size_pct: [20, Auto],
                axis_alignment: Alignment::End
            ]
        ));

        let mut content_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 90],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        let selected_network_row = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                padding: [5., 0., 15., 0.],
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
                widgets::Image::new("wifi_icon"),
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
                    Text::new(txt!("Signal"))
                        .style("color", Color::WHITE)
                        .style("size", 15.0)
                        .style("line_height", 17.50)
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
                    Text::new(txt!(signal_strength))
                        .style("color", Color::WHITE)
                        .style("size", 14.0)
                        .style("line_height", 20.0)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Bold),
                    lay![
                        direction: Direction::Row,
                        axis_alignment: Alignment::Start,
                        cross_alignment: Alignment::Center,
                    ]
                )),
            ),
        );

        let selected_network_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(selected_network_row);

        // details text
        let details_text = node!(
            Text::new(txt!("Details"))
                .style("color", Color::rgba(197., 197., 197., 1.))
                .style("size", 18.0)
                .style("line_height", 20.0)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Bold),
            lay![
                direction: Direction::Row,
                margin: [20.0, 0.0, 10.0, 0.0],
            ]
        );

        // NOTE: removed subnet and gatway to mac address and Name
        // status, passphrase - security
        let details_row_1 = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Name"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(connected_network.name.clone()))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Status"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!("Not Connected"))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        );

        let details_row_2 = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Frequency"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(if connected_network.frequency.starts_with("2") {
                    "2.4 GHz"
                } else {
                    "5 GHz"
                }))
                .style("color", Color::WHITE)
                .style("size", 18.0)
                .style("line_height", 20.0)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Signal"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(signal_strength))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        );

        let details_row_3 = node!(
            Div::new(),
            lay![
                size_pct: [100, Auto],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("MAC Address"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(connected_network.mac.clone()))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [50, Auto],
                    axis_alignment: Alignment::Start,
                    direction: Direction::Column,
                ]
            )
            .push(node!(
                Text::new(txt!("Security"))
                    .style("color", Color::rgba(197., 197., 197., 1.))
                    .style("size", 15.0)
                    .style("line_height", 17.5)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    margin: [0.0, 0.0, 4.0, 0.0],
                ]
            ))
            .push(node!(
                Text::new(txt!(security))
                    .style("color", Color::WHITE)
                    .style("size", 18.0)
                    .style("line_height", 20.0)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Bold),
                lay![]
            )),
        );

        content_node = content_node.push(details_text);

        // content_node = content_node.push(selected_network_node);
        // content_node = content_node.push(node!(HDivider { size: 1. }, lay![
        //     margin: [0.0, 0.0, 30.0, 0.0],
        // ]));

        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [0., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_1);
        content_node = content_node.push(node!(
            HDivider { size: 0.5 },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_2);
        content_node = content_node.push(node!(
            HDivider { size: 0.5 },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));
        content_node = content_node.push(details_row_3);
        content_node = content_node.push(node!(
            HDivider { size: 1. },
            lay![
                margin: [10., 0., 10., 0.]
            ]
        ));

        base = base.push(header_node);
        base = base.push(content_node);

        Some(base)
    }
}
