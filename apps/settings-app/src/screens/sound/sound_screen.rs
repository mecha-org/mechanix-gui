use crate::gui::Message;
use crate::gui::{self, Routes};
use crate::screens::sound::sound_model::SoundModel;
use crate::shared::slider::{Slider, SliderType};
use crate::{components::*, header_node, main, tab_item_node};

use super::input_device_selector::InputDeviceSelector;
use super::output_device_selector::OutputDeviceSelector;

#[derive(Debug, Clone)]
pub enum SoundScreenRoute {
    SoundScreen,
    SelectOutputDevice,
    SelectInputDevice,
}

struct SoundScreenState {
    pub route: SoundScreenRoute,
}

#[derive(Debug)]
#[component(State = "SoundScreenState")]
pub struct SoundScreen {}

impl SoundScreen {
    pub fn new() -> Self {
        SoundScreen {
            dirty: false,
            state: Some(SoundScreenState {
                route: SoundScreenRoute::SoundScreen,
            }),
        }
    }
}

#[state_component_impl(SoundScreenState)]
impl component::Component for SoundScreen {
    fn init(&mut self) {
        SoundModel::update();
        self.state_mut().route = SoundScreenRoute::SoundScreen;
    }

    fn view(&self) -> Option<Node> {
        let mut base: Node = node!(
            widgets::Div::new().bg(Color::BLACK),
            lay![
                size_pct: [100],
                direction: layout::Direction::Column,
                cross_alignment: layout::Alignment::Stretch,
                padding: [5.0, 0.0, 5.0, 0.0],
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                // size_pct: [100, 90],
                size: [440, Auto],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                margin: [10., 0., 0., 0.],
            ]
        );

        let mut scrollable_node = node!(
            Scrollable::new(size!(440, 370)),
            lay![
                size: [440, 370],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        )
        .push(node!(
            Div::new(),
            lay![
                size: [440, Auto],
                direction: Direction::Column,
                cross_alignment: Alignment::Stretch,
            ]
        ));

        let output_slider = node!(
            Slider::new()
                .value((*SoundModel::get().output_volume.get()).ceil() as u8)
                .slider_type(SliderType::Line)
                .active_color(Color::rgba(226., 102., 0., 1.))
                .on_slide_end(Box::new(|value| {
                    SoundModel::set_output_volume(value.into());
                    Box::new(())
                }))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[5., 15., 12., 10.]]
        );
        let input_slider = node!(
            Slider::new()
                .value((*SoundModel::get().input_volume.get()) as u8)
                .slider_type(SliderType::Line)
                .active_color(Color::rgba(156., 53., 240., 1.))
                .on_slide_end(Box::new(|value| {
                    SoundModel::set_input_volume(value.into());
                    Box::new(())
                }))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[5., 15., 12., 10.]]
        );

        // let current_output_device = SoundModel::get().current_output_device.get();
        // let current_input_device = SoundModel::get().current_input_device.get();

        let current_output_device = "On-Device Speaker";
        let current_input_device = "On-Device Mic";

        main_node = main_node.push(node!(
            widgets::Div::new(),
            lay![
                size: [440, 65],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                padding: [0., 0., 0., 12.],
            ]
        )
        .push(
            tab_item_node!(
                    [text_node("Output")],
                    [text_bold_node(current_output_device)],
                    on_click: Some(Box::new(move || msg!(Message::ChangeSoundScreenRoute { route: SoundScreenRoute::SelectOutputDevice }))),
                )
            )
        );
        main_node = main_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));
        main_node = main_node.push(
            node!(
                Div::new(),
                lay![
                    margin: [0., 10., 0., 8.]
                ]
            )
            .push(sub_header_node("Output Volume")),
        );
        main_node = main_node.push(output_slider);
        main_node = main_node.push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [10., 0., 0., 0.]
            ]
        ));

        main_node = main_node.push(node!(
            widgets::Div::new(),
            lay![
                size: [440, 65],
                direction: Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
                padding: [0., 0., 0., 12.],
            ]
        )
        .push(
            tab_item_node!(
                    [text_node("Input")],
                    [text_bold_node(current_input_device)],
                    on_click: Some(Box::new(move || msg!(Message::ChangeSoundScreenRoute { route: SoundScreenRoute::SelectInputDevice }))),
                )
            )
        );
        main_node = main_node.push(node!(HDivider {
            size: 0.8,
            color: Color::rgba(83., 83., 83., 1.)
        }));
        main_node = main_node.push(
            node!(
                Div::new(),
                lay![
                    margin: [0., 10., 0., 8.]
                ]
            )
            .push(sub_header_node("Input Volume")),
        );
        main_node = main_node.push(input_slider);
        main_node = main_node.push(node!(
            HDivider {
                size: 0.8,
                color: Color::rgba(83., 83., 83., 1.)
            },
            lay![
                margin: [10., 0., 0., 0.]
            ]
        ));

        scrollable_node = scrollable_node.push(main_node);

        base = base.push(header_node!(
            "Sound",
            if let SoundScreenRoute::SoundScreen = self.state_ref().route {
                Box::new(|| {
                    msg!(Message::ChangeRoute {
                        route: Routes::SettingsList,
                    })
                })
            } else {
                Box::new(|| {
                    msg!(Message::ChangeSoundScreenRoute {
                        route: SoundScreenRoute::SoundScreen,
                    })
                })
            }
        ));
        match self.state_ref().route {
            SoundScreenRoute::SelectOutputDevice => {
                base = base.push(node!(OutputDeviceSelector {}))
            }
            SoundScreenRoute::SelectInputDevice => base = base.push(node!(InputDeviceSelector {})),
            SoundScreenRoute::SoundScreen => {
                base = base.push(scrollable_node);
            }
        }

        Some(base)
    }

    fn update(&mut self, msg: prelude::Message) -> Vec<prelude::Message> {
        if let Some(msg) = msg.downcast_ref::<Message>() {
            match msg {
                Message::ChangeSoundScreenRoute { route } => {
                    self.state_mut().route = route.clone();
                }
                _ => (),
            }
        }
        vec![msg]
    }
}
