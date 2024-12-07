use std::hash::Hash;

use super::component::NetworkRowComponent;
use super::wireless_model::WirelessModel;
use crate::AppMessage;
use crate::{
    components::{header_node, text_node},
    gui::{Message, NetworkMessage, NetworkScreenRoutes, Routes},
    main,
    shared::h_divider::HDivider,
};

use mctk_core::prelude::cosmic_text::rustybuzz::ttf_parser::Fixed;
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::renderables::Image;
use mctk_core::widgets::Button;
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

enum NetworkDetailsMessage {
    openModel(bool),
    ForgetNetwork,
}

#[derive(Debug, Clone)]
pub struct SavedNetworkDetailsState {
    pub is_model_open: bool,
    pub mac: String,
}

#[derive(Debug)]
#[component(State = "SavedNetworkDetailsState")]
pub struct SavedNetworkDetails {}

impl SavedNetworkDetails {
    pub fn new(mac: String) -> Self {
        SavedNetworkDetails {
            dirty: false,
            state: Some(SavedNetworkDetailsState {
                is_model_open: false,
                mac,
            }),
        }
    }
}

#[state_component_impl(SavedNetworkDetailsState)]
impl Component for SavedNetworkDetails {
    fn init(&mut self) {
        WirelessModel::update();
    }

    fn render_hash(&self, hasher: &mut mctk_core::component::ComponentHasher) {
        self.state_ref().is_model_open.hash(hasher);
    }

    fn view(&self) -> Option<Node> {
        let networks = WirelessModel::get()
            .scan_result
            .get()
            .wireless_network
            .clone();
        let mut network = WirelessInfoResponse {
            mac: "".to_string(),
            name: "".to_string(),
            signal: "".to_string(),
            frequency: "".to_string(),
            flags: "".to_string(),
        };
        for n in networks {
            if self.state_ref().mac == n.mac {
                network = n;
                break;
            }
        }
        let network_status = "Saved";
        let security = network.flags.clone();
        let mut signal_strength = "-";
        if let Ok(signal_int) = network.signal.parse::<i32>() {
            if signal_int < 30_i32 {
                signal_strength = "Weak";
            } else if signal_int >= 30 && signal_int < 70 {
                signal_strength = "Good";
            } else if signal_int >= 70 {
                signal_strength = "Excellent";
            }
        };

        let is_model_open = self.state_ref().is_model_open;

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
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [20, Auto],
                    axis_alignment: Alignment::End
                ]
            )
            .push(node!(
                IconButton::new("delete_icon")
                    .on_click(Box::new(move || msg!(NetworkDetailsMessage::openModel(
                        !is_model_open
                    ))))
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
                    padding: [0., 0., 0., 2.]
                ]
            )),
        );

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
                    Text::new(txt!("Status"))
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
                    Text::new(txt!(network_status))
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
                Text::new(txt!(network.name.clone()))
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
                Text::new(txt!(network_status))
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
                Text::new(txt!(if network.frequency.starts_with("2") {
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
                Text::new(txt!(network.mac))
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

        let start_node = node!(
            Div::new(),
            lay![
                direction: Direction::Row,
                margin: [20.0, 0.0, 10.0, 0.0],
            ]
        );
        content_node = content_node.push(start_node);

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

        let modal = node!(
            Div::new()
                .bg(Color::DARK_GREY)
                .border(Color::DARK_GREY, 1., (10., 10., 10., 10.)),
            lay![
                size: [280, 180],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                position_type: Absolute,
                position: [120., 80., 0., 0.],
            ]
        )
        .push(
            node!(
                Div::new().border(Color::TRANSPARENT, 1., (10., 10., 10., 10.)),
                // Div::new(),
                lay![
                size_pct: [100, 70],
                direction: Direction::Row,
                axis_alignment: Alignment::Center,
                cross_alignment: Alignment::Center,
                padding:[0., 20., 0., 0.]
                ]
            )
            .push(node!(
                Text::new(txt!("Forget this network? "))
                    .style("color", Color::WHITE)
                    .style("size", 20.)
                    .style("line_height", 22.)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    size_pct: [100, 50],
                ],
            )),
        )
        .push(
            node!(
                Div::new().border(Color::TRANSPARENT, 1.5, (0., 10., 10., 10.)),
                lay![
                    size_pct: [100, 30],
                    direction: Direction::Row,
                    cross_alignment: Alignment::Stretch,
                    axis_alignment: Alignment::Stretch,
                    padding: [0., 0., 5., 0.]
                ]
            )
            .push(node!(
                Button::new(txt!("Cancel"))
                    .style("text_color", Color::WHITE)
                    .style("background_color", Color::DARK_GREY)
                    .style("active_color", Color::MID_GREY)
                    .style("font_size", 20.)
                    .style("line_height", 22.)
                    .on_click(Box::new(move || msg!(NetworkDetailsMessage::openModel(
                        !is_model_open
                    )))),
                lay![
                    size_pct: [48, Auto],
                ]
            ))
            .push(
                node!(
                    Div::new().bg(Color::TRANSPARENT),
                    lay![
                     size_pct: [4, Auto],
                     axis_alignment: Alignment::Center,
                     cross_alignment: Alignment::Center
                    ]
                )
                .push(node!(
                    Text::new(txt!("|"))
                        .style("color", Color::LIGHT_GREY)
                        .style("size", 20.)
                        .style("line_height", 22.)
                        .style("font", "Space Grotesk")
                        .style("font_weight", FontWeight::Normal),
                    lay![
                        cross_alignment: Alignment::Center
                    ]
                )),
            )
            .push(node!(
                Button::new(txt!("Forget"))
                    .style("text_color", Color::RED)
                    .style("background_color", Color::DARK_GREY)
                    .style("active_color", Color::MID_GREY)
                    .style("font_size", 20.)
                    .style("line_height", 22.)
                    .on_click(Box::new(move || {
                        WirelessModel::forget_saved_network(network.name.clone());
                        msg!(Message::ChangeRoute {
                            route: Routes::Network {
                                screen: NetworkScreenRoutes::Networking
                            }
                        })
                    })),
                lay![
                    size_pct: [48, Auto],
                ]
            )),
        );

        if is_model_open.clone() == true {
            base = base.push(modal);
        }
        base = base.push(header_node);
        base = base.push(content_node);
        Some(base)
    }

    fn update(&mut self, msg: mctk_core::component::Message) -> Vec<mctk_core::component::Message> {
        if let Some(message) = msg.downcast_ref::<NetworkDetailsMessage>() {
            match message {
                NetworkDetailsMessage::openModel(value) => {
                    self.state_mut().is_model_open = *value;
                }
                NetworkDetailsMessage::ForgetNetwork => {
                    self.state_mut().is_model_open = false;
                }
            }
        }
        vec![msg]
    }
}
