use std::fmt::Debug;

use crate::sctk::layer_app::LockMessage;
use crate::settings::{LockScreenSettings, Modules};
use crate::theme::LockScreenTheme;
use crate::widgets::custom_container::CustomContainer;
use crate::widgets::modal::Modal;
use crate::widgets::password_key::{Message as PasswordKeyMessage, PasswordKey};
use crate::widgets::password_text_widget::{
    Message as PasswordTextWidgetMessage, PasswordTextWidget,
};
use crate::widgets::slider_widget::{Message as SliderWidgetMessage, SliderWidget};
use iced::font::{Family, Weight};
use iced::widget::canvas::{Cache, Geometry, Path};
use iced::widget::scrollable::{Direction, Properties};
use iced::widget::{
    canvas, column, image, row, vertical_space, Canvas, Column, Container, MouseArea, Row, Text,
};
use iced::{
    executor, mouse,
    widget::{button, container, mouse_area, text, Scrollable},
    window, Alignment, Application, Background, Color, Command, Element, Font, Length, Pixels,
    Point, Rectangle, Settings, Size, Subscription, Theme, Vector,
};
use iced_aw::Wrap;
use iced_wgpu::core::Widget;
use smithay_client_toolkit::reexports::calloop;
// use iced_aw::wrap::Wrap;
use crate::settings;
use iced_runtime::Program;
use iced_style::container::Appearance;
use time::OffsetDateTime;
use tracing::info;

/// # LockScreen State
///
/// This struct is the state definition of the entire application
pub struct LockScreen {
    pub settings: LockScreenSettings,
    pub custom_theme: LockScreenTheme,
    pub password_keys: Vec<PasswordKey>,
    pub password_text_widgets: Vec<PasswordTextWidget>,
    pub password: String,
    pub is_user_authenticating: bool,
    pub is_authentication_failed: bool,
    pub now: time::OffsetDateTime,
    pub canvas_layer: Cache,
    pub unlock_button_pressing: bool,
    pub unlock_button_pressing_time: f32,
    pub lock_channel: Option<calloop::channel::Sender<LockMessage>>,
}

impl Default for LockScreen {
    fn default() -> Self {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => LockScreenSettings::default(),
        };

        Self {
            settings: settings,
            custom_theme: LockScreenTheme::default(),
            password_text_widgets: vec![],
            password_keys: vec![],
            password: String::from(""),
            is_user_authenticating: false,
            is_authentication_failed: false,
            now: time::OffsetDateTime::now_local()
                .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            canvas_layer: Default::default(),
            unlock_button_pressing: false,
            unlock_button_pressing_time: 0.0,
            lock_channel: None,
        }
    }
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
    LockIconPressed,
    Tick(time::OffsetDateTime),
    Pressed,
    Released,
}

#[derive(Debug, Clone)]
pub struct GenerateLayout {
    pub modules: Modules,
    pub current_time: OffsetDateTime,
    pub wifi_strength: i8,
    pub bluetooth_state: i8,
    pub battery_level: u8,
}

impl Program for LockScreen {
    type Renderer = iced::Renderer;
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> iced_runtime::Command<Self::Message> {
        info!(task = "update", "message is {:?}", message);
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
                        let _ = self.password_text_widgets[self.password.len() - 1]
                            .update(PasswordTextWidgetMessage::ToggleFilled);
                        let is_password_wrong = String::from("1234") != self.password;
                        let is_password_length_reached = self.password.len()
                            == self.settings.modules.password_configs.password_length;
                        if is_password_length_reached {
                            info!(task = "auth user", "password entered is {}", self.password);
                        }

                        if is_password_length_reached && is_password_wrong {
                            self.password = String::from("");
                            self.is_authentication_failed = true;
                            for num in 0..self.settings.modules.password_configs.password_length {
                                let _ = self.password_text_widgets[num]
                                    .update(PasswordTextWidgetMessage::ToggleFilled);
                            }
                        }
                    }
                };
                Command::none()
            }
            Message::PasswordTextWidget(code) => Command::none(),
            Message::HomePressed => Command::none(),
            Message::BackSpacePressed => {
                if self.password.len() <= 0 {
                    return Command::none();
                }
                self.password.pop();
                let _ = self.password_text_widgets[self.password.len()]
                    .update(PasswordTextWidgetMessage::ToggleFilled);
                Command::none()
            }
            Message::LockIconPressed => {
                self.is_user_authenticating = true;
                Command::none()
            }
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                    self.canvas_layer.clear();

                    if self.unlock_button_pressing {
                        self.unlock_button_pressing_time = self.unlock_button_pressing_time + 1.0;
                    }
                };
                Command::none()
            }
            Message::Pressed => {
                println!("received message pressed");
                self.unlock_button_pressing = true;

                self.unlock_button_pressing_time = 0.0;
                Command::none()
            }
            Message::Released => {
                println!(
                    "received message released {}",
                    self.unlock_button_pressing_time
                );

                let unlock_pressing_time = self.unlock_button_pressing_time;
                self.unlock_button_pressing = false;
                self.unlock_button_pressing_time = 0.0;

                if unlock_pressing_time > 15.0 {
                    let _ = self
                        .lock_channel
                        .as_ref()
                        .unwrap()
                        .send(LockMessage::Unlock);
                }

                Command::none()
            }
        };
        command
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Renderer> {
        // let app_row =
        //     generate_apps_grid(self.settings.modules.apps.clone(), self.search_text.clone());

        let background_color = self.custom_theme.background.default.clone().unwrap().color;

        let wrapped_keys = Wrap::with_elements(
            self.settings
                .layout
                .grid
                .clone()
                .into_iter()
                .map(|title| {
                    let index_in_password_keys = self
                        .password_keys
                        .clone()
                        .into_iter()
                        .position(|p_key| p_key.key.to_owned() == title)
                        .map_or(-1, |i| i as isize);
                    let mut element: Element<_> = Text::new("").into();
                    if title == self.settings.modules.home.title {
                        match self.settings.modules.home.icon.default.clone() {
                            Some(icon) => {
                                element = Container::new(
                                    MouseArea::new(image(icon)).on_release(Message::HomePressed),
                                )
                                .width(90)
                                .height(90)
                                .center_x()
                                .center_y()
                                .into()
                            }
                            None => (),
                        }
                    } else if title == self.settings.modules.back_space.title {
                        match self.settings.modules.back_space.icon.default.clone() {
                            Some(icon) => {
                                element = Container::new(
                                    MouseArea::new(image(icon))
                                        .on_release(Message::BackSpacePressed),
                                )
                                .width(90)
                                .height(90)
                                .center_x()
                                .center_y()
                                .into();
                            }
                            None => (),
                        }
                    } else if index_in_password_keys >= 0 {
                        element = self.password_keys[index_in_password_keys as usize]
                            .view()
                            .map(Message::PasswordKeyPressed)
                            .into()
                    };
                    element
                })
                .collect(),
        )
        .line_spacing(12.0)
        .spacing(12.0);
        // .padding(5.0);;
        let scrollable_keys = Scrollable::new(wrapped_keys).direction(Direction::Vertical(
            Properties::new().scroller_width(0).width(0),
        ));

        let password_text_widget_ui: Element<_> = row(self
            .password_text_widgets
            .iter()
            .enumerate()
            .map(|(i, widget)| {
                widget
                    .view()
                    .map(move |message| Message::PasswordTextWidget(message))
            })
            .collect())
        .spacing(16)
        .into();

        let auth_failed_text = Text::new(match self.is_authentication_failed {
            true => "Invalid PIN, try again".to_string(),
            false => "".to_string(),
        })
        .size(12)
        .height(30);

        let mut lock_screen = Column::new()
            .push(vertical_space(210))
            .align_items(Alignment::Center)
            .width(Length::Fill);

        match self.settings.modules.lock.icon.default.clone() {
            Some(icon) => {
                lock_screen = lock_screen.push(
                    Row::new()
                        .push(MouseArea::new(image(icon)).on_release(Message::LockIconPressed)),
                )
            }
            None => (),
        };

        let auth_screen = column![
            vertical_space(30),
            password_text_widget_ui,
            auth_failed_text,
            scrollable_keys
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .spacing(11);

        let current_screen: Element<_> = match self.is_user_authenticating {
            true => auth_screen.into(),
            false => lock_screen.into(),
        };
        let pin_screen_conatiner = container(current_screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(
                CustomContainer::new(Appearance {
                    background: Option::from(Background::Color(Color::from_rgba8(
                        background_color[0],
                        background_color[1],
                        background_color[2],
                        1.0,
                    ))),
                    ..Default::default()
                }),
            )));

        let canvas: Canvas<&LockScreen, Message> = canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill);

        let canvas_conatiner = container(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(
                StyledContainer::new(container::Appearance {
                    background: Option::from(Background::Color(Color::from_rgba8(0, 0, 0, 0.0))),
                    ..Default::default()
                }),
            )));

        let modal = container(column![column![mouse_area(image(self.settings.modules.lock.icon.default.clone().unwrap()))
            .on_press(Message::Pressed)
            .on_release(Message::Released),]
        .spacing(10)])
        // .padding(10)
        // .style(theme::Container::Box)
        ;

        Modal::new(canvas_conatiner, modal)
            // .on_blur(Message::Animate)
            .into()
    }
}

//canvas impl start
impl<Message> canvas::Program<Message, iced::Renderer> for LockScreen {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let drawing = self.canvas_layer.draw(renderer, bounds.size(), |frame| {
            let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::rectangle(
                Point::new(frame.width(), 0.0),
                Size::new(-frame.width(), frame.height()),
            );

            let center_point = frame.center();
            let circle = Path::circle(
                center_point,
                frame
                    .width()
                    .min(frame.height())
                    .min((self.unlock_button_pressing_time * 40.0) / 2.0),
            );

            //frame.fill(&background, Color::from_rgba8(0, 0, 0, 0.50));
            // frame.fill(&animate_button, Color::from_rgba8(1, 1, 1, 1.0));
            frame.fill(&circle, Color::from_rgb8(0x12, 0x93, 0xD8));
            frame.with_save(|frame| {
                frame.translate(center);
            });
        });

        vec![drawing]
    }
}
//canvas impl end

//Styled container start
pub struct StyledContainer {
    pub(crate) appearance: container::Appearance,
}

impl StyledContainer {
    pub fn new(params: container::Appearance) -> Self {
        Self { appearance: params }
    }
}

impl container::StyleSheet for StyledContainer {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Theme::Light => container::Appearance {
                text_color: None,
                background: Some(Background::Color(iced::Color::TRANSPARENT)),
                border_radius: 4.0.into(),
                border_width: 12.0,
                border_color: Color::TRANSPARENT,
            },
            _ => self.appearance,
        }
    }
}
//Styled container end
