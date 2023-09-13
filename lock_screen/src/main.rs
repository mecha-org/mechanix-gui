use iced::{Alignment, Application, Background, Color, Element, executor, Font, Pixels, Settings, widget::{column, container, Scrollable}, window};
use iced::{Command, Length, Subscription, Theme};
use iced::font::{Family, Weight};
use iced::widget::{Column, Container, image, MouseArea, row, Row, Text, vertical_space};
use iced::widget::scrollable::{Direction, Properties};
use iced_aw::wrap::Wrap;
use iced_style::container::Appearance;
use time::OffsetDateTime;
use tracing::info;

use settings::LockScreenSettings;

use crate::settings::Modules;
use crate::theme::LockScreenTheme;
use crate::widgets::custom_container::CustomContainer;
use crate::widgets::password_key::{Message as PasswordKeyMessage, PasswordKey};
use crate::widgets::password_text_widget::{Message as PasswordTextWidgetMessage, PasswordTextWidget};
use crate::widgets::slider_widget::{Message as SliderWidgetMessage, SliderWidget};

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
        .with_env_filter("mecha_lock_screen=trace")
        .with_thread_names(true)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => {
            LockScreenSettings::default()
        }
    };

    info!(task = "init_settings", "settings initialized for lock screen {:?}", settings);

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => {
            LockScreenTheme::default()
        }
    };

    info!(task = "init_theme", "theme initialized for lock screen {:?}", custom_theme);

    let window_settings = settings.window;
    let app_settings = settings.app;
    let position = window_settings.position;

    let SPACE_GROTESK_FONT: Font = Font {
        family: iced::font::Family::Name("Space Grotesk"),
        weight: iced::font::Weight::Light,
        stretch: iced::font::Stretch::Normal,
        monospaced: false,
    };

    LockScreen::run(Settings {
        window: window::Settings {
            size: window_settings.size,
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
        default_font: SPACE_GROTESK_FONT,
        //try_opengles_first: app_settings.try_opengles_first,
        // default_font: Some(&FONT_DATA),
        ..Settings::default()
    })
}

/// # LockScreen State
///
/// This struct is the state definition of the entire application
struct LockScreen {
    settings: LockScreenSettings,
    custom_theme: LockScreenTheme,
    password_keys: Vec<PasswordKey>,
    password_text_widgets: Vec<PasswordTextWidget>,
    password: String,
    is_user_authenticating: bool,
    is_authentication_failed: bool
}

/// ## Message
///
/// These are the events (or messages) that update state. 
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    PasswordKeyPressed(PasswordKeyMessage),
    PasswordTextWidget(PasswordTextWidgetMessage),
    FontLoaded(Result<(), iced::font::Error>),
    HomePressed,
    BackSpacePressed,
    LockIconPressed
}

#[derive(Debug, Clone)]
pub struct GenerateLayout {
    pub modules: Modules,
    pub current_time: OffsetDateTime,
    pub wifi_strength: i8,
    pub bluetooth_state: i8,
    pub battery_level: u8,
}

/// # Top-level Application implementation for Iced
///
/// Primary components includes -
/// - `new()` - initializes the state
/// - `update()` - maps all the actions corresponding to 'Message'
/// - `view()` - the view definition
impl Application for LockScreen {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => {
                LockScreenSettings::default()
            }
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => {
                LockScreenTheme::default()
            }
        };

        let modules = settings.modules.clone();

        (LockScreen {
            settings,
            custom_theme,
            password_text_widgets: vec![PasswordTextWidget::new(false), PasswordTextWidget::new(false), PasswordTextWidget::new(false), PasswordTextWidget::new(false)],
            password_keys: modules.password_configs.keys_allowed.into_iter().map(|key| {
                PasswordKey::new(String::from(key))
            }).collect(),
            password: String::from(""),
            is_user_authenticating: false,
            is_authentication_failed: false,
        }, Command::batch([iced::font::load(include_bytes!("./assets/fonts/space-grotesk.ttf").as_slice())
            .map(Message::FontLoaded)]), )
    }

    fn title(&self) -> String {
        String::from(&self.settings.title)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        info!(task = "update",  "message is {:?}", message);
        let command = match message {
            Message::FontLoaded(_) => {
                info!(task = "load fond", "font loaded successfully");
                Command::none()
            }
            Message::PasswordKeyPressed(code) => {
                match code {
                    PasswordKeyMessage::WidgetClicked(password_key) => {
                        info!(task = "key presed", "key Pressed is {:?}", password_key);
                        self.is_authentication_failed = false;
                        self.password = [self.password.to_owned(), password_key].join("");
                        self.password_text_widgets[self.password.len() - 1].update(PasswordTextWidgetMessage::ToggleFilled);
                        let is_password_wrong =  String::from("1234") != self.password;
                        let is_password_length_reached = self.password.len() == self.settings.modules.password_configs.password_length;
                        if is_password_length_reached {
                            info!(task = "auth user", "password entered is {}", self.password);
                        }


                        if is_password_length_reached && is_password_wrong {
                            self.password = String::from("");
                            self.is_authentication_failed = true;
                            for num in 0..self.settings.modules.password_configs.password_length {
                                self.password_text_widgets[num].update(PasswordTextWidgetMessage::ToggleFilled);
                            }
                        }
                    }
                };
                Command::none()
            }
            Message::PasswordTextWidget(code) => {
                Command::none()
            }
            Message::HomePressed => {
                Command::none()
            }
            Message::BackSpacePressed => {
                if self.password.len() <=0 {
                    return Command::none();
                }
                self.password.pop();
                self.password_text_widgets[self.password.len()].update(PasswordTextWidgetMessage::ToggleFilled);
                Command::none()
            }
            Message::LockIconPressed => {
                self.is_user_authenticating = true;
                Command::none()
            }
        };
        command
    }

    fn view(&self) -> Element<Message> {
        // let app_row = generate_apps_grid(self.settings.modules.apps.clone(), self.search_text.clone());

        let background_color = self.custom_theme.background.default.clone().unwrap().color;

        let wrapped_keys =

            Wrap::with_elements(self.settings.layout.grid.clone().into_iter().map(|title| {
                let index_in_password_keys = self.password_keys.clone().into_iter().position(|p_key| p_key.key.to_owned() == title).map_or(-1, |i| i as isize);
                let mut element: Element<_> = Text::new("").into();
                if title == self.settings.modules.home.title {
                    match self.settings.modules.home.icon.default.clone() {
                        Some(icon) => {
                            element = Container::new( MouseArea::new(image(icon)).on_release(Message::HomePressed)).width(90).height(90).center_x().center_y().into()
                        },
                        None => (),
                    }
                }
                else if title == self.settings.modules.back_space.title {
                    match self.settings.modules.back_space.icon.default.clone() {
                        Some(icon) => {
                            element = Container::new( MouseArea::new(image(icon)).on_release(Message::BackSpacePressed)).width(90).height(90).center_x().center_y().into();
                        },
                        None => ()
                    } 
                }
                else if index_in_password_keys >= 0 {
                    element = self.password_keys[index_in_password_keys as usize].view().map(Message::PasswordKeyPressed).into()
                };
                element
            }).collect())
                .line_spacing(12.0)
                .spacing(12.0);
        // .padding(5.0);;
        let scrollable_keys = Scrollable::new(wrapped_keys).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0)));

        let password_text_widget_ui: Element<_> =
             row(self.password_text_widgets
                .iter().enumerate().map(|(i, widget)| widget.view().map(move |message| {
                Message::PasswordTextWidget(message)
            })).collect())
                .spacing(16)
                .into();

        let auth_failed_text =  Text::new( match self.is_authentication_failed {
            true => "Invalid PIN, try again".to_string(),
            false => "".to_string()
        })
            .size(12)
            .height(30);

        let mut lock_screen = Column::new()
            .push(vertical_space(210))
            .align_items(Alignment::Center).width(Length::Fill);

        match self.settings.modules.lock.icon.default.clone() {
            Some(icon) => {
                lock_screen = lock_screen.push(  Row::new().push((MouseArea::new(image(icon)).on_release(Message::LockIconPressed))))
            },
            None => ()
        };    

        let auth_screen = column![vertical_space(30), password_text_widget_ui, auth_failed_text, scrollable_keys]
            .align_items(Alignment::Center).width(Length::Fill)
            .spacing(11);

        let current_screen: Element<_> = match self.is_user_authenticating {
            true => auth_screen.into(),
            false => lock_screen.into()
        };

        container(
            current_screen
        )
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(CustomContainer::new(Appearance {
                background: Option::from(Background::Color(Color::from_rgba8(background_color[0], background_color[1], background_color[2], 1.0))),
                ..Default::default()
            }))))
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([])
    }
}


