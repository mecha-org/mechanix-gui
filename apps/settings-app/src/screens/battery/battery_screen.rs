use crate::gui::Message;
use crate::gui::Routes;
use crate::header_node;
use crate::shared::h_divider::HDivider;
use crate::shared::slider::Slider;
use crate::shared::slider::SliderType;
use crate::{components::*, tab_item_node};

use super::battery_model::BatteryModel;
use super::performance_mode::PerformanceMode;

#[derive(Debug, Clone)]
pub enum BatteryScreenRoute {
    BatteryScreen,
    PerformanceMode,
}

struct BatteryScreenState {
    pub route: BatteryScreenRoute,
}

#[derive(Debug)]
#[component(State = "BatteryScreenState")]
pub struct BatteryScreen {}

impl BatteryScreen {
    pub fn new() -> Self {
        BatteryScreen {
            dirty: false,
            state: Some(BatteryScreenState {
                route: BatteryScreenRoute::BatteryScreen,
            }),
        }
    }
}

#[state_component_impl(BatteryScreenState)]
impl Component for BatteryScreen {
    fn init(&mut self) {
        BatteryModel::update();
        self.state_mut().route = BatteryScreenRoute::BatteryScreen;
    }
    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                padding: [5.0, 0.0, 5.0, 0.0],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100, 80],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                padding: [5.0, 0.0, 0.0, 0.0],
            ]
        );

        // TODO : update view
        let battery_percentage = node!(
            Slider::new()
                .value(*BatteryModel::get().battery_percentage.get() as u8)
                .slider_type(SliderType::Line)
                .active_color(Color::rgb(102., 226., 0.))
                // .on_slide(Box::new(|value| Box::new(())))
                .col_spacing(8.)
                .col_width(3.75)
                .disabled(true),
            lay![size: [Auto, 45], margin:[10., 10., 50., 10.]]
        );

        main_node = main_node.push(text_node(
            format!(" {}%", *BatteryModel::get().battery_percentage.get() as u8).as_str(),
        ));
        main_node = main_node.push(battery_percentage);
        main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(tab_item_node!(
            [text_node("Mode")],
            [text_bold_node("Balenced"), icon_node("right_arrow_icon")],
            on_click: Some(Box::new(move || msg!(Message::ChangeBatteryScreenRoute { route: BatteryScreenRoute::PerformanceMode } ))),
        ));
        main_node = main_node.push(node!(HDivider { size: 1. }));

        base = base.push(header_node!(
            "Battery",
            Box::new(|| {
                msg!(Message::ChangeRoute {
                    route: Routes::SettingsList
                })
            })
        ));

        match self.state_ref().route {
            BatteryScreenRoute::BatteryScreen => {
                base = base.push(main_node);
            }
            BatteryScreenRoute::PerformanceMode => base = base.push(node!(PerformanceMode {})),
        }

        Some(base)
    }

    fn update(&mut self, msg: prelude::Message) -> Vec<prelude::Message> {
        if let Some(msg) = msg.downcast_ref::<Message>() {
            match msg {
                Message::ChangeBatteryScreenRoute { route } => {
                    self.state_mut().route = route.clone();
                }
                _ => (),
            }
        }
        vec![msg]
    }
}
