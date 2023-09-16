use std::fs;

use font_loader::system_fonts;
use iced::{Alignment, Application, Background, Color, Element, executor, Settings, widget::{container, image}, window, Font};
use iced::{Command, Length, Subscription, Theme};
use iced::widget::{Column, MouseArea, row, Text};
use iced_style::container::Appearance;
use tracing::info;

use settings::ActionBarSettings;

use crate::theme::ActionBarTheme;
use crate::widgets::styled_container::StyledContainer;

mod settings;
mod widgets;
mod theme;

pub mod errors;


/// Initialize the application with settings, and starts
pub fn main() -> iced::Result {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_action_bar=trace")
        .with_thread_names(true)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => {
            ActionBarSettings::default()
        }
    };

    info!(task = "init_settings", "settings initialized for action bar {:?}", settings);

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => {
            ActionBarTheme::default()
        }
    };

    info!(task = "init_theme", "theme initialized for action bar {:?}", custom_theme);

    let window_settings = settings.window;
    let app_settings = settings.app;
    let position = window_settings.position;

    let mut default_font: Font = Font {
        family: iced::font::Family::SansSerif,
        ..Default::default()
    };

    //Check if there is defaut font added in theme.yml then make that default
    match custom_theme.font.default {
        Some(font) => match font.name {
            Some(font_name) => match font_name.len() > 0 {
                true => {
                    let property = system_fonts::FontPropertyBuilder::new().family(&font_name).build();
                    match system_fonts::get(&property){
                        Some(_) => {
                            default_font = Font {
                                family: iced::font::Family::Name(Box::leak(font_name.into_boxed_str())),
                                weight: iced::font::Weight::Light,
                                stretch: iced::font::Stretch::Normal,
                                monospaced: false,
                            };
                        },
                        None => (),
                    }
                },
                false => ()
            },
            None => (),
        },
        None => (),
    };

    ActionBar::run(Settings {
        window: window::Settings {
            size: window_settings.size.default,
            position: window::Position::Specific(position.0, position.1),
            min_size: window_settings.min_size,
            max_size: window_settings.max_size,
            visible: window_settings.visible,
            resizable: window_settings.resizable,
            decorations: window_settings.decorations,
            transparent: window_settings.transparent,
            // always_on_top: window_settings.always_on_top,
            ..Default::default()
        },
        id: app_settings.id,
        //text_multithreading: app_settings.text_multithreading,
        antialiasing: app_settings.antialiasing,
        //try_opengles_first: app_settings.try_opengles_first,
        default_font,
        ..Settings::default()
    })
}

/// # Action Bar State
///
/// This struct is the state definition of the entire application
struct ActionBar {
    settings: ActionBarSettings,
    custom_theme: ActionBarTheme,
}

/// ## Message
///
/// These are the events (or messages) that update state. 
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    SettingsPressed,
    HomePressed,
    KeyBoardPressed,
}

#[derive(Debug, Clone)]
enum DockState {
    MINIMIZED,
    MAXIMIZED,
    HOME,
}

/// # Top-level Application implementation for Iced
///
/// Primary components includes -
/// - `new()` - initializes the state
/// - `update()` - maps all the actions corresponding to 'Message'
/// - `view()` - the view definition
impl Application for ActionBar {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => {
                ActionBarSettings::default()
            }
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => {
                ActionBarTheme::default()
            }
        };

        (ActionBar {
            settings,
            custom_theme,
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from(&self.settings.title)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        info!(task = "update", "message: {:?}", message);
        match message {
            Message::SettingsPressed => {
                Command::none()
            }
            Message::HomePressed => {
                Command::none()
            }
            Message::KeyBoardPressed => {
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let background_color = self.custom_theme.background.default.clone().unwrap().color;

        let modules = self.settings.modules.clone();

        let layout = self.settings.layout.clone();

        let items: Element<_> =
            row([layout.left.clone(), layout.center.clone(), layout.right.clone()]
                .concat()
                .into_iter()
                .map(|item| {
                    let mut element: Element<_> = Text::new("").into();
                
                    if item.to_lowercase() == modules.settings.title.to_lowercase() {
                        match modules.settings.icon.clone() {
                            Some(icon) => {
                                let settings_button = Column::new()
                                .push(
                                    MouseArea::new(
                                        image(icon
                                        )).on_release(Message::SettingsPressed)
                                )
                                .width(Length::FillPortion(1));
                                element = settings_button.into()
                            },
                            None => (),
                        }
                    } else if item == modules.home.title {
                        match modules.home.icon.clone() {
                            Some(icon) => {
                                let home_button = Column::new()
                                .push(MouseArea::new(image(icon)).on_release(Message::HomePressed))
                                .width(Length::FillPortion(1));
                            element = home_button.into()
                            },
                            None => ()
                        } 
                    } else if item == modules.keyboard.title {
                        match modules.keyboard.icon.clone() {
                            Some(icon) => {
                                let keyboard_button = Column::new()
                                .push(MouseArea::new(image(icon)).on_release(Message::KeyBoardPressed));
                                element =  keyboard_button.into()
                            },
                            None => ()
                        }
                    } 
                    element
                }).collect()
            ).into();

        container(items)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([10, 24])
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance {
                background: Option::from(Background::Color(Color::from_rgba8(background_color.0, background_color.1, background_color.2, background_color.3))),
                ..Default::default()
            }))))
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

