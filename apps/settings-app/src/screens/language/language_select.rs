use crate::gui::{Message, Routes};
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct LanguageSelect {}
impl Component for LanguageSelect {
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

        let values = [
            ("English", "English-UK"),
            ("English", "English-US"),
            ("Chinese", "Chinese, simplified"),
        ];

        main_node = main_node.push(get_header_node("Language Select"));
        for (i, (language, language_type)) in values.into_iter().enumerate() {
            main_node = main_node.push(
                tab_item_node!(
                    [text_bold_node(language), text_node(language_type)],
                    [icon_node("white_right_arrow")]
                )
                .key((i + 1) as u64),
            );
            main_node = main_node.push(
                node!(HDivider {
                    size: 1.,
                    color: Color::rgba(83., 83., 83., 1.)
                })
                .key(2 * i as u64),
            );
        }
        // main_node = main_node.push(footer_node!(Routes::LanguageScreen));
        base = base.push(main_node);
        Some(base)
    }
}
