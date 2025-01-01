use upower::BatteryStatus;

use crate::gui::Message;
use crate::gui::Routes;
use crate::header_node;
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
        let current_mode = BatteryModel::get().cureent_mode.get().clone();

        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: layout::Direction::Column,
                padding: [5.0, 0.0, 5.0, 0.0],
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100, 90],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                margin: [10., 0., 0., 0.],
                padding: [0., 8., 0., 8.]
            ]
        );

        let battery_percentage_value = *BatteryModel::get().battery_percentage.get() as u8;
        // let battery_percentage_value = 50;
        let mut charge_text = "";

        let battery_status = BatteryModel::get().battery_status_value.get();

        if *battery_status == BatteryStatus::Unknown {
            charge_text = "";
        } else if *battery_status == BatteryStatus::Charging {
            charge_text = "Charging";
        } else if *battery_status == BatteryStatus::Discharging {
            charge_text = "Discharging";
        } else {
            charge_text = "Charded";
        }

        // green
        let mut active_color = Color::rgb(52., 199., 89.);
        let mut inactive_color = Color::rgb(74., 108., 82.);

        if battery_percentage_value <= 20 {
            // red
            active_color = Color::rgb(199., 52., 52.);
            inactive_color = Color::rgb(132., 92., 92.);
        } else if battery_percentage_value <= 40 {
            // yellow
            active_color = Color::rgb(199., 160., 52.);
            inactive_color = Color::rgb(152., 144., 106.);
        }

        let battery_percentage_widget = node!(
            Slider::new()
                // .value(*BatteryModel::get().battery_percentage.get() as u8)   // OG
                .value(battery_percentage_value)
                .slider_type(SliderType::BatteryLine)
                .active_color(active_color)
                .inactive_color(inactive_color)
                .col_spacing(10.)
                .col_width(14.)
                .disabled(true),
            lay![size: [Auto, 45], margin:[5., 5., 35., 5.]]
        );

        main_node = main_node.push(sub_header_node(
            format!("{} ({}%)", charge_text, battery_percentage_value).as_str(),
        ));

        main_node = main_node.push(battery_percentage_widget);
        main_node = main_node.push(node!(
            HDivider {
                size: 1.,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                padding: [10., 0., 0., 0.]
            ]
        ));
        main_node = main_node.push(tab_item_node!(
            [text_node("Power mode")],
            [text_bold_node(&current_mode)],
            on_click: Some(Box::new(move || msg!(Message::ChangeBatteryScreenRoute { route: BatteryScreenRoute::PerformanceMode } ))),
        ));
        main_node = main_node.push(node!(HDivider {
            size: 1.,
            color: Color::rgba(83., 83., 83., 1.)
        }));

        base = base.push(header_node!(
            "Battery",
            if let BatteryScreenRoute::BatteryScreen = self.state_ref().route {
                Box::new(|| {
                    msg!(Message::ChangeRoute {
                        route: Routes::SettingsList,
                    })
                })
            } else {
                Box::new(|| {
                    msg!(Message::ChangeBatteryScreenRoute {
                        route: BatteryScreenRoute::BatteryScreen,
                    })
                })
            }
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
