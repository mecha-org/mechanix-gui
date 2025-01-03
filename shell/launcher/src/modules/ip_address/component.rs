use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component::Component, node, Node};
use mctk_core::{txt, Color};
use networkmanager::IpAddressModel;

#[derive(Debug)]
pub struct IpAddress {}

impl Component for IpAddress {
    fn init(&mut self) {
        IpAddressModel::start_streaming();
    }

    fn view(&self) -> Option<Node> {
        let ip_addresses = IpAddressModel::get().ip_address.get().clone();
        let mut ip_address = "".to_string();
        if let Some(ip) = ip_addresses.get("ethernet") {
            ip_address = ip.clone();
        }
        if let Some(ip) = ip_addresses.get("wireless") {
            ip_address = ip.clone();
        }

        Some(node!(Text::new(txt!(ip_address))
            .with_class("font-space-mono font-normal")
            .style("color", Color::rgb(201., 201., 201.))
            .style("size", 15.0)))
    }
}
