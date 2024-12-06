use crate::gui::Message;
use crate::gui::Routes;
use crate::shared::h_divider::HDivider;
use crate::shared::slider::Slider;
use crate::shared::slider::SliderType;
use crate::{components::*, tab_item_node};

#[derive(Debug)]
pub struct BatteryScreen {}
impl Component for BatteryScreen {
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

        main_node = main_node.push(header_node("Battery"));

        let battery_percentage = node!(
            Slider::new()
                .value(50)
                .slider_type(SliderType::Line)
                .active_color(Color::rgb(102., 226., 0.))
                // .on_slide(Box::new(|value| Box::new(())))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[10., 10., 50., 10.]]
        );

        main_node = main_node.push(text_node(format!("BATTERY {}%", 50).as_str()));
        main_node = main_node.push(battery_percentage);
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [text_node("Mode")],
            [text_bold_node("Balenced"), icon_node("right_arrow_icon")],
            route: Routes::PerformanceModes
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));

        // base = base.push(footer_node!(Routes::SettingsList));
        base = base.push(main_node);
        Some(base)
    }
}
