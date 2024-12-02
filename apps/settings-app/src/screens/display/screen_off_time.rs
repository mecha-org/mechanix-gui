use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, footer_node, tab_item_node};

#[derive(Debug)]
pub struct ScreenOffTime {}
impl Component for ScreenOffTime {
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
                size_pct: [100, 100],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [0.0, 10.0, 0.0, 10.0],
            ]
        );

        main_node = main_node.push(header_node("Screen off time"));
        // // NOTE :  less option as footer back won't work due to overlapping issue
        // main_node = main_node.push(radio_node(vec!["10s", "20s", "30s", "60s", "5m", "Never"]));
        main_node = main_node.push(radio_node(vec!["10s", "20s", "30s"]));
        base = base.push(footer_node!(
            Routes::DisplayScreen,
            "tick_icon",
            Box::new(|| msg!(Message::ChangeRoute {
                route: Routes::DisplayScreen,
            }))
        ));
        base = base.push(main_node);
        Some(base)
    }
}
