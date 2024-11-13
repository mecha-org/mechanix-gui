use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct PerformanceMode {}
impl Component for PerformanceMode {
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

        main_node = main_node.push(header_node("Performance Mode"));
        main_node = main_node.push(radio_node(vec!["Low", "Balenced", "High"]));
        main_node = main_node.push(node!(Div::new(), lay![size: [20]]));
        main_node = main_node.push(text_node("Higher performance will use battery faster"));
        main_node = main_node.push(text_node("and Check ambient temperature before"));
        main_node = main_node.push(text_node("proceeding. increase the temperature of the"));
        main_node = main_node.push(text_node("device significantly."));
        base = base.push(main_node);
        base = base.push(footer_node(ScreenRoute {
            route: Routes::DisplayScreen,
        }));
        Some(base)
    }
}
