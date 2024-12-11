use crate::components::*;

#[derive(Debug)]
pub struct PerformanceMode {}
impl Component for PerformanceMode {
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: Direction::Column,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        );

        main_node = main_node.push(radio_node(vec!["Low", "Balenced", "High"]));
        main_node = main_node.push(node!(Div::new(), lay![size: [20]]));
        main_node = main_node.push(text_node("Higher performance will use battery faster"));
        main_node = main_node.push(text_node("and Check ambient temperature before"));
        main_node = main_node.push(text_node("proceeding. increase the temperature of the"));
        main_node = main_node.push(text_node("device significantly."));

        let sub_header = text_bold_node("Performance Mode");

        base = base.push(sub_header);
        base = base.push(main_node);
        Some(base)
    }
}
