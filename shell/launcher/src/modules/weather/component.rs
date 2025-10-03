use mctk_core::style::{FontWeight, Styled};
use mctk_core::widgets::Text;
use mctk_core::{component::Component, node, Node};
use mctk_core::{txt, Color};

#[derive(Debug)]
pub struct Weather {}

impl Component for Weather {
    fn init(&mut self) {
        // WeatherModel::start_streaming();
    }

    fn view(&self) -> Option<Node> {
        let temperature = "Mostly wind,20°C";

        Some(node!(Text::new(txt!(temperature))
            .with_class("font-space-mono font-normal")
            .style("color", Color::rgb(201., 201., 201.))
            .style("size", 19.0)))
    }
}
