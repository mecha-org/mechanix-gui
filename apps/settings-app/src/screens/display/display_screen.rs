use crate::shared::slider::{Slider, SliderType};
use crate::{components::*, header_node, tab_item_node};

use crate::gui::{Message, Routes};

use super::brightness_model::BrightnessModel;
use super::screen_off_time::ScreenOffTime;

#[derive(Debug, Clone)]
pub enum DisplayScreenRoute {
    DisplayScreen,
    ScreenOffTime,
}

struct DisplayScreenState {
    pub route: DisplayScreenRoute,
}

#[derive(Debug)]
#[component(State = "DisplayScreenState")]
pub struct DisplayScreen {}

impl DisplayScreen {
    pub fn new() -> Self {
        DisplayScreen {
            dirty: false,
            state: Some(DisplayScreenState {
                route: DisplayScreenRoute::DisplayScreen,
            }),
        }
    }
}

#[state_component_impl(DisplayScreenState)]
impl Component for DisplayScreen {
    fn init(&mut self) {
        BrightnessModel::update();
        self.state_mut().route = DisplayScreenRoute::DisplayScreen;
    }

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
                size_pct: [100, 90],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                margin: [10., 0., 0., 0.],
                padding: [0., 8., 0., 8.]
            ]
        );

        let slider = node!(
            Slider::new()
                .value(*BrightnessModel::get().brightness_percentage.get() as u8)
                .slider_type(SliderType::Box)
                .on_slide_end(Box::new(|value| {
                    BrightnessModel::set_brightness(value as f64);
                    Box::new(())
                }))
                .active_color(Color::rgb(15., 168., 255.))
                .on_slide(Box::new(|_| { Box::new(()) }))
                .col_spacing(7.75)
                .row_spacing(7.75)
                .col_width(4.),
            lay![size: [Auto, 45], margin:[5., 5., 35., 5.]]
        );

        let screen_off_time = tab_item_node!(
            [text_node("Screen Time")],
            [text_bold_node("30s"), get_icon("white_right_arrow", IconType::Svg, rect![0., 0., 0., 10.])],
            on_click: Some(Box::new(move || msg!(Message::ChangeDisplayScreenRoute { route: DisplayScreenRoute::ScreenOffTime } ))),
        );

        main_node = main_node.push(sub_header_node("Brightness"));

        main_node = main_node.push(slider);

        main_node = main_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));
        main_node = main_node.push(screen_off_time);
        main_node = main_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));

        base = base.push(header_node!(
            "Display",
            if let DisplayScreenRoute::DisplayScreen = self.state_ref().route {
                Box::new(|| {
                    msg!(Message::ChangeRoute {
                        route: Routes::SettingsList,
                    })
                })
            } else {
                Box::new(|| {
                    msg!(Message::ChangeDisplayScreenRoute {
                        route: DisplayScreenRoute::DisplayScreen,
                    })
                })
            }
        ));

        match self.state_ref().route {
            DisplayScreenRoute::DisplayScreen => {
                base = base.push(main_node);
            }
            DisplayScreenRoute::ScreenOffTime => base = base.push(node!(ScreenOffTime {})),
        }

        Some(base)
    }

    fn update(&mut self, msg: prelude::Message) -> Vec<prelude::Message> {
        if let Some(msg) = msg.downcast_ref::<Message>() {
            match msg {
                Message::ChangeDisplayScreenRoute { route } => {
                    self.state_mut().route = route.clone();
                }
                _ => (),
            }
        }
        vec![msg]
    }
}
