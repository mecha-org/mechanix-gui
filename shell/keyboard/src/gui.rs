use crate::action::Modifier;
use crate::components::scrollable::Scrollable;
use crate::components::touch_panel::TouchPanel;
use crate::layout::KeyButton;
use crate::model::KeyboardModel;
use crate::settings::{self, KeyboardSettings};
use crate::{action, AppMessage, AppParams};
use mctk_core::component::RootComponent;
use mctk_core::layout::{Alignment, Dimension, PositionType};
use mctk_core::reexports::smithay_client_toolkit::reexports::calloop::channel::Sender;
use mctk_core::style::{HorizontalPosition, Styled};
use mctk_core::widgets::{Button, IconButton, IconType};
use mctk_core::{component, layout, txt, Color};
use mctk_core::{
    component::Component, lay, msg, node, rect, size, size_pct, state_component_impl, widgets::Div,
    Node,
};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use wayland_protocols_async::zwp_input_method_v2::handler::ContentPurpose;

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    KeyPressed(KeyButton),
    KeyReleased,
    SuggestionClicked(String),
    UpdateSuggestions {
        suggestions: Vec<String>,
        suggested_for: String,
        next_char_prob: HashMap<String, f64>,
    },
    Scrolling {
        status: bool,
    },
    Maximize,
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
    app_channel: Option<Sender<AppMessage>>,
    active_mods: HashSet<Modifier>,
    is_scrolling: bool,
    key_pressed: Option<KeyButton>,
}

#[component(State = "KeyboardState")]
#[derive(Debug, Default)]
pub struct Keyboard {}

impl Keyboard {}

#[state_component_impl(KeyboardState)]
impl Component for Keyboard {
    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        self.state_ref().key_pressed.hash(hasher);
        self.state_ref().purpose.hash(hasher);
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
                println!("inserted email layout");
            }
            Err(e) => {
                println!("Error parsing default layout {:?}", e);
                panic!("");
            }
        };

        match crate::layout::Layout::from_file(settings.layouts.email.clone()) {
            Ok(layout) => {
                let parsed_layout = layout.clone().build().unwrap();
                layouts.insert(ContentPurpose::Email, parsed_layout);
                println!("inserted email layout");
            }
            Err(e) => {
                println!("Error parsing email layout {:?}", e);
                panic!("");
            }
        };

        match crate::layout::Layout::from_file(settings.layouts.url.clone()) {
            Ok(layout) => {
                let parsed_layout = layout.clone().build().unwrap();
                layouts.insert(ContentPurpose::Url, parsed_layout);
                println!("inserted url layout");
            }
            Err(e) => {
                println!("Error parsing url layout {:?}", e);
                panic!("");
            }
        };

        // println!("layout is {:?}", parsed_layout);

        self.state = Some(KeyboardState {
            settings,
            layouts: layouts,
            app_channel: None,
            purpose: ContentPurpose::Normal,
            active_mods: HashSet::new(),
            is_scrolling: false,
            key_pressed: None,
        });
    }

    fn update(&mut self, message: component::Message) -> Vec<component::Message> {
        println!("App was sent: {:?}", message.downcast_ref::<Message>());
        println!("");
        match message.downcast_ref::<Message>() {
            Some(Message::KeyPressed(key_pressed)) => {
                self.state_mut().key_pressed = Some(key_pressed.clone());
            }
            Some(Message::KeyReleased) => {
                println!("Message::KeyReleased");
                if self.state_ref().is_scrolling {
                    self.state_mut().key_pressed = None;
                    return vec![];
                }

                if let Some(key_pressed) = self.state_ref().key_pressed.clone() {
                    let KeyButton {
                        action, keycodes, ..
                    } = key_pressed;

                    match action {
                        action::Action::SetView(view) => {
                            // self.state_mut().current_view = view.clone();
                            KeyboardModel::get().current_view.set(view.clone());
                        }
                        action::Action::LockView {
                            lock,
                            unlock,
                            latches,
                            looks_locked_from,
                        } => {
                            let current_view = KeyboardModel::get().current_view.get().clone();
                            if current_view == lock.clone() {
                                KeyboardModel::get().current_view.set(unlock.clone());
                            } else {
                                KeyboardModel::get().current_view.set(lock.clone());
                            }
                        }
                        action::Action::ApplyModifier(m) => {
                            println!("modifier is {:?}", m);
                            let mut mods = self.state_ref().active_mods.clone();
                            if mods.contains(&m) {
                                mods.remove(&m);
                            } else {
                                mods.insert(m.clone());
                            }

                            self.state_mut().active_mods = mods.clone();
                            if let Some(app_channel) = &self.state_ref().app_channel {
                                let _ = app_channel.send(AppMessage::ApplyModifiers { mods });
                            };
                        }
                        action::Action::Submit { text, keys } => {
                            println!("text {:?} keys {:?}", text, keys);
                            if let Some(app_channel) = &self.state_ref().app_channel {
                                let _ = app_channel.send(AppMessage::TextkeyPressed {
                                    keycode: keycodes[0].clone(),
                                });
                            };
                            let current_view = KeyboardModel::get().current_view.get().clone();
                            if current_view == "upper".to_string() {
                                KeyboardModel::get().current_view.set("base".to_string())
                            }
                        }
                        action::Action::Erase => {
                            KeyboardModel::erase();
                        }
                        action::Action::ShowPreferences => {}
                        action::Action::Minimize => {
                            KeyboardModel::minimize();
                        }
                        action::Action::Maximize => {
                            KeyboardModel::maximize();
                        }
                    };

                    self.state_mut().key_pressed = None;
                }
            }
            Some(Message::UpdateSuggestions {
                suggestions,
                suggested_for,
                next_char_prob,
            }) => {
                // self.state_mut().suggestions = suggestions.clone();
                // self.state_mut().suggested_for = suggested_for.clone();
                // self.state_mut().next_char_prob = next_char_prob.clone();
            }
            Some(Message::SuggestionClicked(suggestion)) => {
                // println!("Suggestion clicked: {}", suggestion);
                // if let Some(app_channel) = &self.state_ref().app_channel {
                //     let _ = app_channel.send(AppMessage::SuggestionPressed {
                //         suggestion: suggestion.clone(),
                //         suggested_for: self.state_ref().suggested_for.clone(),
                //     });
                // }
                KeyboardModel::suggestion_pressed(suggestion.clone());
            }
            // Some(Message::UpdateKeyboardWindow { keyboard_window }) => {
            //     self.state_mut().keyboard_window = keyboard_window.clone();
            // }
            Some(Message::Scrolling { status }) => {
                println!("Message::Scrolling");
                if *status {
                    self.state_mut().key_pressed = None;
                }
                self.state_mut().is_scrolling = *status;
            }
            _ => (),
        }

        vec![]
    }

    fn view(&self) -> Option<Node> {
        //Render view from layout
        let purpose = KeyboardModel::get()
            .purpose
            .get()
            .clone()
            .unwrap_or(ContentPurpose::Normal);
        let keyboard_window = if KeyboardModel::get().maximize.get().clone() {
            KeyboardWindow::Maximized
        } else {
            KeyboardWindow::Minimized
        };
        let layout = self.state_ref().layouts.get(&purpose).unwrap().clone();
        let current_view = KeyboardModel::get().current_view.get().clone();
        let view = layout.views.get(&current_view).unwrap();
        let suggestions = KeyboardModel::get().suggestions.get().clone();
        let next_char_prob = KeyboardModel::get().next_char_prob.get().clone();
        let click_area = self.state_ref().settings.click_area.clone();
        let active_mods = self.state_ref().active_mods.clone();
        let is_scrolling = self.state_ref().is_scrolling.clone();
        let key = match purpose {
            ContentPurpose::Normal => 1,
            ContentPurpose::Alpha => 2,
            ContentPurpose::Digits => 3,
            ContentPurpose::Number => 4,
            ContentPurpose::Phone => 5,
            ContentPurpose::Url => 6,
            ContentPurpose::Email => 7,
            ContentPurpose::Name => 8,
            ContentPurpose::Password => 9,
            ContentPurpose::Pin => 10,
            ContentPurpose::Date => 11,
            ContentPurpose::Time => 12,
            ContentPurpose::Datetime => 13,
            ContentPurpose::Terminal => 14,
            _ => 15,
        };
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
        )
        .key(key);

        let mut suggestion_row = node!(
            Div::new(),
            lay![
                size: [Auto, 36],
                direction: layout::Direction::Row,
                axis_alignment: Alignment::Stretch,
                cross_alignment: Alignment::Center,
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
                        .style("line_height", 22.)
                        .style("active_color", Color::rgba(255., 255., 255., 0.50))
                        .style("background_color", Color::TRANSPARENT),
                    lay![
                        size: [Auto, 28],
                    ],
                )
                .key(i as u64),
            );
        }

        println!("purpose is {:?}", purpose);
        if purpose == ContentPurpose::Normal || purpose == ContentPurpose::Alpha {
            main_div = main_div.push(suggestion_row);
        }

        match keyboard_window {
            KeyboardWindow::Maximized => {
                main_div = main_div.push(
                    node!(
                        Scrollable::new(
                            size!(480, Dimension::Px(210.)),
                            if purpose == ContentPurpose::Terminal {
                                66.
                            } else {
                                0.
                            }
                        ),
                        lay![]
                    ).key(if purpose == ContentPurpose::Terminal {
                        10 as u64
                    }  else {
                        100 as u64
                    })
                    .push(
                        node!(
                            TouchPanel::new(
                                view.clone(),
                                next_char_prob,
                                current_view,
                                click_area,
                                purpose,
                                active_mods,
                                self.state_ref().key_pressed.clone()
                            ),
                            lay![
                            margin: [4., 6. , 0., 0.]
                            size: [ if purpose == ContentPurpose::Terminal {738. } else { 474. }, Dimension::Px(210.)]
                            ]
                        )
                        .key(if self.state_ref().key_pressed.is_some() {
                            10 as u64
                        }
                        else {
                            1000 as u64
                        }),
                    ),
                );
            }
            KeyboardWindow::Minimized => {
                main_div = main_div.push(node!(
                    TouchPanel::new(
                        view.clone(),
                        next_char_prob,
                        current_view,
                        click_area,
                        purpose,
                        active_mods,
                        self.state_ref().key_pressed.clone()
                    ),
                    lay![
                    margin: [4., 6. , 0., 0.]
                    size: [ if purpose == ContentPurpose::Terminal {748. } else { 474. }, Dimension::Px(210.)]
                    ]
                )
                .key(if self.state_ref().key_pressed.is_some() {
                    11 as u64
                }
                else {
                    1100 as u64
                }),);
            }
        }

        // match keyboard_window {
        //     KeyboardWindow::Maximized => {

        //     }
        //     KeyboardWindow::Minimized => {
        //         main_div = main_div.push(node!(
        //             IconButton::new("window-max")
        //                 .on_click(Box::new(|| msg!(Message::Maximize)))
        //                 .style("size", size!(60, 40)),
        //             lay![size:[80, 60]]
        //         ));
        //     }
        // }

        // main_div = main_div.push(
        //     node!(
        //         Scrollable::new(
        //             size!(480, Dimension::Px(210.)),
        //             if purpose == ContentPurpose::Terminal {
        //                 70.
        //             } else {
        //                 0.
        //             }
        //         ),
        //         lay![]
        //     ).key(if purpose == ContentPurpose::Normal {
        //         10 as u64
        //     } else {
        //         100 as u64
        //     })
        //     .push(
        //         node!(
        //             TouchPanel::new(
        //                 view.clone(),
        //                 next_char_prob,
        //                 self.state_ref().current_view.clone(),
        //                 click_area,
        //                 purpose,
        //                 active_mods,
        //                 self.state_ref().key_pressed.clone()
        //             ),
        //             lay![
        //             margin: [4., 6. , 0., 0.]
        //             size: [ if purpose == ContentPurpose::Terminal {748. } else { 474. }, Dimension::Px(210.)]
        //             ]
        //         )
        //         .key(if self.state_ref().key_pressed.is_some() {
        //             10 as u64
        //         }
        //         else {
        //             100 as u64
        //         }),
        //     ),
        // );

        // main_div = main_div.push(node!(
        //     TouchPanel::new(
        //         view.clone(),
        //         next_char_prob,
        //         self.state_ref().current_view.clone(),
        //         click_area,
        //         purpose,
        //         active_mods,
        //         self.state_ref().key_pressed.clone()
        //     ),
        //     lay![
        //         margin: [4., 6. , 0., 0.]
        //         size: [480., 244.]
        //     ]
        // ));

        Some(main_div)
    }
}

impl RootComponent<AppParams> for Keyboard {
    fn root(&mut self, window: &dyn Any, app_params: &dyn Any) {
        let app_params = app_params.downcast_ref::<AppParams>().unwrap();
        self.state_mut().app_channel = app_params.app_channel.clone();
    }
}
