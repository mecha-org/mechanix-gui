use super::wireless_model::WirelessModel;
use crate::gui::{Message, NetworkScreenRoutes, Routes};
use crate::utils::truncate;
use crate::{components::*, header_node};
use std::hash::Hash;

use mctk_core::widgets::{Button, HDivider};
use mctk_core::{
    component::{self, Component},
    lay,
    layout::{Alignment, Dimension, Direction, Size},
    msg, node, rect, size, size_pct,
    style::{FontWeight, Styled},
    txt,
    widgets::{self, Div, IconButton, IconType, Text},
    Color, Node,
};
use mctk_macros::{component, state_component_impl};
use mechanix_system_dbus_client::wireless::WirelessInfoResponse;

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
            Text::new(txt!("Network Information"))
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("size", 28.0)
                .style("line_height", 17.5)
                .style("font", "Space Grotesk")
                .style("font_weight", FontWeight::Normal),
            lay![
                size_pct: [100, Auto],
            ]
        );

        let mut content_node = node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
                margin: [0., 0., 0., 0.],
            ]
        );

        let mut scrollable_section = node!(
            Scrollable::new(size!(440, 340)),
            lay![
                size: [440, 340],
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

        let full_network_name = network.clone().name.clone();
        let truncate_network_name = truncate(network.clone().name.clone(), 20);

        let rows_node = node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Stretch,
            ],
        )
        .push(single_detail_row(DetailRow {
            key: "Name".to_string(),
            value: truncate_network_name.clone(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Status".to_string(),
            value: network_status.to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Frequency".to_string(),
            value: if network.frequency.starts_with("2") {
                "2.4 GHz"
            } else {
                "5 GHz"
            }
            .to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "Signal".to_string(),
            value: signal_strength.to_string(),
        }))
        .push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }))
        .push(single_detail_row(DetailRow {
            key: "MAC Address".to_string(),
            value: network.mac.to_string(),
        }))
        .push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ))
        .push(single_detail_row(DetailRow {
            key: "Security".to_string(),
            value: security.to_string(),
        }))
        .push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [8., 0., 8., 0.]
            ]
        ));

        scrollable_section = scrollable_section.push(rows_node);

        // note : in border with width, does not match with radius  - 1. is the border width
        let modal = node!(
            Div::new().bg(Color::rgba(29., 29., 29., 1.)).border(
                Color::rgba(127., 127., 135., 1.),
                0.,
                (10., 10., 10., 10.)
            ),
            lay![
                size: [320, 160],
                direction: Direction::Column,
                position_type: Absolute,
                position: [140., 60., 0., 0.],
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
                padding: [15., 15., 15., 10.]

            ]
        )
        .push(
            node!(
                Div::new(),
                lay![
                    size_pct: [100, 72],
                    cross_alignment: Alignment::Start,
                    axis_alignment: Alignment::Start,
                ]
            )
            .push(node!(
                Text::new(txt!("Forget this network?"))
                    .style("color", Color::WHITE)
                    .style("size", 18.)
                    .style("line_height", 20.)
                    .style("font", "Space Grotesk")
                    .style("font_weight", FontWeight::Normal),
                lay![
                    size: [Auto],
                ]
            )),
        )
        .push(
            // BUTTONS
            node!(
                Div::new(),
                lay![
                    size_pct: [100, 28],
                    direction: Direction::Row,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch,
                ]
            )
            .push(node!(
                Div::new(),
                lay![
                    size_pct: [28, 100]
                    axis_alignment: Alignment::Start,
                ]
            ))
            .push(
                node!(
                    Div::new(),
                    lay![
                        size_pct: [72, 100]
                        axis_alignment: Alignment::Stretch,
                    ]
                )
                .push(node!(
                    Button::new(txt!("Cancel"))
                        .style("text_color", Color::WHITE)
                        // .style("background_color", Color::rgba(29., 29., 29., 1.))
                        .style("background_color", Color::rgba(68., 68., 68., 1.))
                        .style("active_color", Color::rgba(82., 81., 81., 1.))
                        .style("font_size", 16.)
                        .style("line_height", 18.)
                        .style("radius", 8.)
                        // .style("border_color", Color::rgba(127., 127., 135., 1.))
                        // .style("border_width", 1.)
                        .on_click(Box::new(move || msg!(NetworkDetailsMessage::openModel(
                            !is_model_open
                        )))),
                    lay![
                        size_pct: [46, 100],
                        padding: [0., 0., 0., 12.],
                        axis_alignment: Alignment::Start,

                    ]
                ))
                .push(node!(
                    Button::new(txt!("Forget"))
                        // .style("text_color", Color::BLACK)
                        // .style("background_color", Color::rgba(29., 29., 29., 1.))
                        // .style("active_color", Color::rgba(82., 81., 81., 1.))
                        .style("text_color", Color::BLACK)
                        .style("background_color", Color::WHITE)
                        .style("active_color", Color::rgba(194., 184., 184., 1.))
                        .style("font_size", 16.)
                        .style("line_height", 18.)
                        .style("radius", 8.)
                        // .style("border_color", Color::rgba(127., 127., 135., 1.))
                        // .style("border_width", 1.)
                        .on_click(Box::new(move || {
                            WirelessModel::forget_saved_network(network.clone().name.clone());
                            msg!(Message::ChangeRoute {
                                route: Routes::Network {
                                    screen: NetworkScreenRoutes::Networking
                                }
                            })
                        })),
                    lay![
                        size_pct: [46, 100],
                        padding: [0., 12., 0., 0.],
                        axis_alignment: Alignment::End,
                    ]
                )),
            ),
        );

        if is_model_open.clone() == true {
            base = base.push(modal);
        }

        base = base.push(header_node!(
            truncate_network_name.clone(),
            Box::new(|| {
                msg!(Message::ChangeRoute {
                    route: Routes::Network {
                        screen: NetworkScreenRoutes::Networking
                    }
                })
            }),
            "delete_icon",
            Box::new(move || {
                WirelessModel::forget_saved_network(full_network_name.to_string());
                msg!(Message::ChangeRoute {
                    route: Routes::Network {
                        screen: NetworkScreenRoutes::Networking
                    }
                })
            })
        ));

        content_node = content_node.push(scrollable_section);
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
