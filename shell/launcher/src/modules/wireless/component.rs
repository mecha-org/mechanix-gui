use crate::utils::get_forttated_wireless_status;
use mctk_core::layout::Alignment;
use mctk_core::widgets::Image;
use mctk_core::{component::Component, lay, node, size, size_pct, widgets::Div, Node};
use networkmanager::WirelessModel;
#[derive(Debug)]
pub struct Wireless {}

impl Component for Wireless {
    fn init(&mut self) {
        WirelessModel::start_streaming();
    }

    fn view(&self) -> Option<Node> {
        let wireless_status = get_forttated_wireless_status(WirelessModel::get());

        Some(
            node!(
                Div::new()
                ,
                [
                    size_pct: [100],
                    axis_alignment: Alignment::Center,
                    cross_alignment: Alignment::Center
                ],
            )
            .push(node!(
                Image::new(format!("sm{:?}", wireless_status.to_string())),
                lay![
                    size: [28, 28],
                ],
            )),
        )
    }
}
