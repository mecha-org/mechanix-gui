use mechanix_system_dbus_client::wireless::KnownNetworkListResponse;
use mechanix_system_dbus_client::wireless::KnownNetworkResponse;
use reexports::glutin::api::egl::device;

use crate::footer_node;
use crate::gui::Message;
use crate::gui::NetworkScreenRoutes;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct ManageNetworksScreen {
    pub known_networks_list: Vec<KnownNetworkResponse>,
}
impl Component for ManageNetworksScreen {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0],
            ]
        );

        let mut header_node = node!(
            Div::new(),
            lay![
                size_pct: [100, 25],
                axis_alignment: Alignment::Start,
                direction: Direction::Column
            ]
        );

        let mut header = node!(
            Div::new(),
            lay![
                size_pct: [100, 15],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                padding: [5.0, 5.0, 10.0, 10.0],
                margin: [0., 0., 20., 0.],
            ]
        );
        let header_text = node!(
            Text::new(txt!("Manage Networks"))
                .style("font", "Space Grotesk")
                .style("size", 28.)
                .style("color", Color::rgb(197.0, 197.0, 197.0))
                .style("font_weight", FontWeight::Normal),
            lay![
                margin:[2.0, 5.0, 2.0, 5.0],
                size: size!(20.0, 50.0),
                axis_alignment: Alignment::Start
            ]
        );

        header = header.push(header_text);
        header_node = header_node.push(header);

        let devices = self.known_networks_list.clone();
        main_node = main_node.push(header_node);
        main_node = main_node.push(node!(Div::new(), lay![size: [10]]));
        main_node = main_node.push(node!(HDivider { size: 1. }));

        for (i, device) in devices.into_iter().enumerate() {
            let check_current_network = device.clone().flags.contains("CURRENT");

            main_node = main_node.push(
                tab_item_node!(
                    [text_bold_node(&device.ssid)],
                    [
                        if check_current_network { icon_node("connected_icon") } else {node!(Div::new(), lay![size: [0, 0]])},
                        icon_node("secured_wifi_icon"),
                        icon_node("wifi_strength_icon"),
                        icon_node("info_icon")
                    ],
                    route: Routes::Network {
                        screen: NetworkScreenRoutes::NetworkScreen
                    }
                )
                .key((i + 1) as u64),
            );
            main_node = main_node.push(node!(HDivider { size: 1. }).key(2 * i as u64));
        }

        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(footer_node!(Routes::Network {
            screen: NetworkScreenRoutes::NetworkScreen
        }));
        base = base.push(main_node);
        Some(base)
    }
}
