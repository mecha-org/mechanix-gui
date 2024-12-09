use crate::gui::Message;
use crate::gui::{self, Routes};
use crate::screens::sound::sound_model::SoundModel;
use crate::shared::slider::{Slider, SliderType};
use crate::{components::*, header_node, tab_item_node};

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
            ]
        );

        let mut main_node = node!(
            widgets::Div::new(),
            lay![
                size_pct: [100],
                cross_alignment: layout::Alignment::Stretch,
                direction: layout::Direction::Column,
                // padding: [15.0, 10.0, 15.0, 10.0],
            ]
        );

        let output_slider = node!(
            Slider::new()
                .value((*SoundModel::get().output_volume.get()).ceil() as u8)
                .slider_type(SliderType::Line)
                .active_color(Color::rgb(226., 102., 0.))
                .on_slide_end(Box::new(|value| {
                    SoundModel::set_output_volume(value.into());
                    Box::new(())
                }))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[10., 10., 0., 10.]]
        );
        let input_slider = node!(
            Slider::new()
                .value((*SoundModel::get().input_volume.get()) as u8)
                .slider_type(SliderType::Line)
                .active_color(Color::rgb(102., 226., 0.))
                .on_slide_end(Box::new(|value| {
                    SoundModel::set_input_volume(value.into());
                    Box::new(())
                }))
                .col_spacing(8.)
                .col_width(3.75),
            lay![size: [Auto, 45], margin:[10., 10., 0., 10.]]
        );

        let output_device = tab_item_node!(
            [text_node("Output Speaker")],
            [icon_node("right_arrow_icon")],
            on_click: Some(Box::new(move || msg!(Message::ChangeSoundScreenRoute { route: SoundScreenRoute::SelectOutputDevice }))),
        );

        let input_device = tab_item_node!(
            [text_node("Input Microphone")],
            [icon_node("right_arrow_icon")],
            on_click: Some(Box::new(move || msg!(Message::ChangeSoundScreenRoute { route: SoundScreenRoute::SelectInputDevice }))),
        );
        main_node = main_node.push(node!(Div::new(), lay![size: [20]]));
        main_node = main_node.push(text_bold_node("OUTPUT"));
        main_node = main_node.push(output_slider);
        main_node = main_node.push(output_device);
        // main_node = main_node.push(node!(HDivider { size: 1. }));
        main_node = main_node.push(node!(Div::new(), lay![size: [20]]));
        main_node = main_node.push(text_bold_node("INPUT"));
        main_node = main_node.push(input_slider);
        main_node = main_node.push(input_device);

        // main_node = main_node.push(footer_node!(Routes::SettingsList));

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
                base = base.push(main_node);
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
