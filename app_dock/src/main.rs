#![deny(clippy::all)]
use std::borrow::Cow;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use iced::{executor, widget::{container, Scrollable, row, column, image}, window, Application, Element, Settings, Background, Color, Alignment, Renderer, Size};
use iced::widget::mouse_area;
use iced::widget::scrollable::{Direction, Properties};
use iced::{Command, Length, Subscription, Theme, Font};
use settings::AppDockSettings;

use iced_style::container::Appearance;
use lazy_static::lazy_static;

mod settings;
mod widgets;
mod theme;
use tracing::{error, info};
use crate::settings::App;
use crate::theme::AppDockThemeTheme;
use crate::widgets::styled_container::StyledContainer;

pub mod errors;


/// Initialize the application with settings, and starts
pub fn main() -> iced::Result {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_app_dock=trace")
        .with_thread_names(true)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => {
            AppDockSettings::default()
        }
    };

    info!(task = "init_settings", "settings initialized for app dock {:?}", settings);

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => {
            AppDockThemeTheme::default()
        }
    };

    info!(task = "init_theme", "theme initialized for app dock {:?}", custom_theme);

    let window_settings = settings.window;
    let app_settings = settings.app;
    let position = window_settings.position;
    let default_font_family = match custom_theme.font.default {
        Some(font) => match font.name {
            Some(font_name) => Box::leak(font_name.into_boxed_str()),
            None => "SansSerif",
        },
        None => "SansSerif",
    };
    
    let default_font: Font = Font {
        family: iced::font::Family::Name(default_font_family),
        weight: iced::font::Weight::Light,
        stretch: iced::font::Stretch::Normal,
        monospaced: false,
    };

    AppDock::run(Settings {
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

/// # App Dock State
/// 
/// This struct is the state definition of the entire application
struct AppDock {
    settings: AppDockSettings,
    custom_theme: AppDockThemeTheme,
    drawer_launcher_pressed_at: u128,
    dock_state: DockState
}

/// ## Message
/// 
/// These are the events (or messages) that update state. 
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    AppClicked(String),
    AppClickedReleased(String),
    HomePressed,
    FontLoaded(Result<(), iced::font::Error>),
}
#[derive(Debug, Clone)]
enum DockState {
    MINIMIZED,
    HOME,
}

/// # Top-level Application implementation for Iced
/// 
/// Primary components includes -
/// - `new()` - initializes the state
/// - `update()` - maps all the actions corresponding to 'Message'
/// - `view()` - the view definition
impl Application for AppDock {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => {
                AppDockSettings::default()
            }
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => {
                AppDockThemeTheme::default()
            }
        };

        //let font_load_command = iced::font::load(get_static_cow_from_asset(include_bytes!("../src/assets/fonts/spaces-grotesk.ttf")));

        (AppDock {
            settings,
            custom_theme,
            drawer_launcher_pressed_at: 0,
            dock_state: DockState::MINIMIZED
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from(&self.settings.title)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AppClicked(state) => {
                info!(task = "update", "app clicked {}", state);
                match state.as_str() {
                    "drawer_launcher" => {
                        self.drawer_launcher_pressed_at = 0;
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("time went backwards");
                        let in_ms = since_the_epoch.as_millis();
                        self.drawer_launcher_pressed_at = in_ms;
                    },
                    _ => {}
                }
                Command::none()
            },
            Message::AppClickedReleased(state) => {
                info!(task = "update", "app clicked released {}", state);
                match state.as_str() {
                    "drawer_launcher" => {
                        let start = SystemTime::now();
                        let since_the_epoch = start
                            .duration_since(UNIX_EPOCH)
                            .expect("time went backwards");
                        let in_ms = since_the_epoch.as_millis();
                        info!(task = "dock released", "button pressed at {} released at {} diff is {}",self.drawer_launcher_pressed_at, in_ms, in_ms - self.drawer_launcher_pressed_at);
                        if in_ms - self.drawer_launcher_pressed_at > 500 {
                            self.dock_state = DockState::HOME;
                            return Command::batch([window::resize(Size::new(self.settings.window.size.other.0, self.settings.window.size.other.1))]);
                        }
                    },
                    _ => {}
                }
                Command::none()
            },
            Message::HomePressed => {
                self.dock_state = DockState::MINIMIZED;
                Command::batch([window::resize(Size::new(self.settings.window.size.minimized.0, self.settings.window.size.minimized.1))])
            },
            Message::FontLoaded(res) => {
                if res.is_err() {
                    error!(task = "load_font", success = "false", "failed to load font");
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {

        let background_color = self.custom_theme.background.default.clone().unwrap().color;

        let pinned_apps_row = generate_apps_list(self.settings.modules.pinned_apps.clone(), false, 68, 68);

        let mut list_column = column![];

        match self.dock_state {
            DockState::MINIMIZED => {
                list_column = column![pinned_apps_row];
            },
            DockState::HOME => 
                match self.settings.modules.home.icon.to_owned() {
                    Some(icon) => list_column = column![mouse_area(image(icon)).on_release(Message::HomePressed)],
                    None => (),
                }
        }

        container(list_column .spacing(16))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance{
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

fn app_to_element(app: App,icon_width: u16, icon_height: u16) -> Element<'static, Message, Renderer> {
    // Convert the app data into an Element, following your UI generation logic
    // For example:
   // let app_name = text(&app.name).size(11);
    column![mouse_area(column![image(&app.icon).height(icon_height).width(icon_width)]
        //.spacing(12)
        .align_items(Alignment::Center))
        .on_press(Message::AppClicked(app.alias.to_owned()))
        .on_release(Message::AppClickedReleased(app.alias.to_owned()))
        ]
        .into()
}

fn generate_apps_list<'a>(
    apps: Vec<App>,
    is_scrollable: bool,
    icon_width: u16,
    icon_height: u16,
) -> Element<'static, Message, Renderer> {
    let mut apps_list = row![];

    for app in apps {
        let app_element = app_to_element(app, icon_width, icon_height);
        apps_list = apps_list.push(app_element);
    }

    if !is_scrollable{
        return column!(apps_list.spacing(24))
            .width(Length::Fill)
            // .padding(Padding::from([10, 0]))
            .into();
    }

    let scrollable_apps = Scrollable::new(apps_list.spacing(16))
        .direction(Direction::Horizontal(Properties::new().scroller_width(0).width(0)));

    column!(scrollable_apps)
        .width(Length::Fill)
        .into()
}

pub fn get_static_cow_from_asset(static_asset: &'static [u8]) -> Cow<'static, [u8]> {
    Cow::Borrowed(static_asset)
}

