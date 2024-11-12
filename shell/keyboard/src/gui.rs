use crate::components::touch_panel::TouchPanel;
use crate::settings::{self, KeyboardSettings};
use crate::{action, AppMessage, AppParams};
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, PositionType};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::style::{HorizontalPosition, Styled};
use mctk_core::widgets::{Button, IconButton, IconType};
use mctk_core::{component, layout, txt, Color};
use mctk_core::{
    component::Component, lay, msg, node, rect, size, size_pct, state_component_impl, widgets::Div,
    Node,
};
use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use wayland_protocols_async::zwp_input_method_v2::handler::ContentPurpose;

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    KeyClicked(crate::action::Action, Vec<crate::layout::KeyCode>),
    SuggestionClicked(String),
    UpdateSuggestions {
        suggestions: Vec<String>,
        suggested_for: String,
        next_char_prob: HashMap<String, f64>,
    },
    ContentInfo {
        purpose: ContentPurpose,
    },
    Reset,
    // UpdateKeyboardWindow {
    //     keyboard_window: KeyboardWindow,
    // },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KeyboardWindow {
    Maximized,
    Minimized,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug)]
pub struct KeyboardState {
    settings: KeyboardSettings,
    layouts: HashMap<ContentPurpose, crate::layout::ParsedLayout>,
    purpose: ContentPurpose,
    current_view: String,
    app_channel: Option<Sender<AppMessage>>,
    suggestions: Vec<String>,
    suggested_for: String,
    next_char_prob: HashMap<String, f64>,
    keyboard_window: KeyboardWindow,
}

#[component(State = "KeyboardState")]
#[derive(Debug, Default)]
pub struct Keyboard {}

impl Keyboard {}

#[state_component_impl(KeyboardState)]
impl Component for Keyboard {
    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        self.state_ref().keyboard_window.hash(hasher);
    }

    fn init(&mut self) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => KeyboardSettings::default(),
        };

        let mut layouts = HashMap::new();

        match crate::layout::Layout::from_file(settings.layouts.default.clone()) {
            Ok(layout) => {
                let parsed_layout = layout.clone().build().unwrap();
                layouts.insert(ContentPurpose::Normal, parsed_layout)
            }
            Err(e) => {
                println!("Error parsing default layout {:?}", e);
                panic!("");
            }
        };

        match crate::layout::Layout::from_file(settings.layouts.terminal.clone()) {
            Ok(layout) => {
                let parsed_layout = layout.clone().build().unwrap();
                layouts.insert(ContentPurpose::Terminal, parsed_layout);
                println!("inserted terminal layout");
            }
            Err(e) => {
                println!("Error parsing default layout {:?}", e);
                panic!("");
            }
        };

        // println!("layout is {:?}", parsed_layout);

        self.state = Some(KeyboardState {
            settings,
            layouts: layouts,
            current_view: String::from("base"),
            app_channel: None,
            suggestions: vec![],
            suggested_for: String::new(),
            next_char_prob: HashMap::new(),
            purpose: ContentPurpose::Normal,
            keyboard_window: KeyboardWindow::Maximized,
        });
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        match message.downcast_ref::<Message>() {
            Some(Message::KeyClicked(action, keycodes)) => match action {
                action::Action::SetView(view) => {
                    self.state_mut().current_view = view.clone();
                }
                action::Action::LockView {
                    lock,
                    unlock,
                    latches,
                    looks_locked_from,
                } => {
                    if self.state_ref().current_view == lock.clone() {
                        self.state_mut().current_view = unlock.clone();
                    } else {
                        self.state_mut().current_view = lock.clone();
                    }
                }
                action::Action::ApplyModifier(m) => {}
                action::Action::Submit { text, keys } => {
                    // println!("text {:?} keys {:?}", text, keys);
                    if let Some(app_channel) = &self.state_ref().app_channel {
                        let _ = app_channel.send(AppMessage::TextkeyPressed {
                            keycode: keycodes[0].clone(),
                        });
                    };
                    if self.state_ref().current_view == "upper".to_string() {
                        self.state_mut().current_view = "base".to_string()
                    }
                }
                action::Action::Erase => {
                    if let Some(app_channel) = &self.state_ref().app_channel {
                        let _ = app_channel.send(AppMessage::Erase);
                    };
                }
                action::Action::ShowPreferences => {}
                action::Action::Minimize => {
                    self.state_mut().keyboard_window = KeyboardWindow::Minimized;
                    self.state_mut().current_view = "minimize".to_string();
                    if let Some(app_channel) = &self.state_ref().app_channel {
                        let _ = app_channel.send(AppMessage::Minimize);
                    }
                }
                action::Action::Maximize => {
                    self.state_mut().keyboard_window = KeyboardWindow::Maximized;
                    self.state_mut().current_view = "base".to_string();
                    if let Some(app_channel) = &self.state_ref().app_channel {
                        let _ = app_channel.send(AppMessage::Maximize);
                    }
                }
            },
            Some(Message::UpdateSuggestions {
                suggestions,
                suggested_for,
                next_char_prob,
            }) => {
                self.state_mut().suggestions = suggestions.clone();
                self.state_mut().suggested_for = suggested_for.clone();
                self.state_mut().next_char_prob = next_char_prob.clone();
            }
            Some(Message::SuggestionClicked(suggestion)) => {
                // println!("Suggestion clicked: {}", suggestion);
                if let Some(app_channel) = &self.state_ref().app_channel {
                    let _ = app_channel.send(AppMessage::SuggestionPressed {
                        suggestion: suggestion.clone(),
                        suggested_for: self.state_ref().suggested_for.clone(),
                    });
                }
            }
            Some(Message::ContentInfo { purpose }) => {
                self.state_mut().purpose = purpose.clone();
            }
            // Some(Message::UpdateKeyboardWindow { keyboard_window }) => {
            //     self.state_mut().keyboard_window = keyboard_window.clone();
            // }
            Some(Message::Reset) => {
                self.state_mut().keyboard_window = KeyboardWindow::Maximized;
                self.state_mut().current_view = String::from("base");
            }
            _ => (),
        }
        vec![]
    }

    fn view(&self) -> Option<Node> {
        //Render view from layout
        let purpose = self.state_ref().purpose;
        let keyboard_window = self.state_ref().keyboard_window;
        let layout = self.state_ref().layouts.get(&purpose).unwrap().clone();
        let current_view = self.state_ref().current_view.clone();
        let view = layout.views.get(&current_view).unwrap();
        let suggestions = self.state_ref().suggestions.clone();
        let next_char_prob = self.state_ref().next_char_prob.clone();
        let click_area = self.state_ref().settings.click_area.clone();
        let mut main_div = node!(
            Div::new().bg(if keyboard_window == KeyboardWindow::Maximized {
                Color::BLACK
            } else {
                Color::TRANSPARENT
            }),
            lay![
                size_pct: [100 , 100],
                direction: layout::Direction::Column,
                cross_alignment: Alignment::Stretch,
                axis_alignment: Alignment::Stretch,
            ]
        );

        let mut suggestion_row = node!(
            Div::new(),
            lay![
                size: [Auto, 48],
                direction: layout::Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center
            ]
        );

        for (i, suggestion) in suggestions.into_iter().enumerate() {
            suggestion_row = suggestion_row.push(
                node!(
                    Button::new(txt!(suggestion.clone()))
                        .on_click(Box::new(move || msg!(Message::SuggestionClicked(
                            suggestion.to_string()
                        ))))
                        .style("h_alignment", HorizontalPosition::Center)
                        .style("radius", 4.6)
                        .style("text_color", Color::WHITE)
                        .style("font_size", 16.)
                        .style("active_color", Color::rgba(255., 255., 255., 0.50))
                        .style("background_color", Color::TRANSPARENT),
                    lay![
                        margin: [0., 2.5, 0., 2.5],
                    ],
                )
                .key(i as u64),
            );
        }

        if purpose == ContentPurpose::Normal {
            main_div = main_div.push(suggestion_row);
        }

        main_div = main_div.push(node!(
            TouchPanel::new(
                view.clone(),
                next_char_prob,
                self.state_ref().current_view.clone(),
                click_area,
                purpose,
            ),
            lay![ margin: [8., if purpose == ContentPurpose::Terminal { 12. } else { 0. }, 0., 0.]]
        ));

        Some(main_div)
    }
}

impl RootComponent<AppParams> for Keyboard {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
    }
}
