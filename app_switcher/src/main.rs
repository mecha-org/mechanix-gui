use iced::{Alignment, Application, Background, Color, Element, executor, Font, Settings, widget::container, window};
use iced::{Command, Length, Subscription, Theme};
use iced::font::Weight;
use iced::widget::{Column, column, image, MouseArea, Row, row, Scrollable, Text, vertical_space};
use iced::widget::scrollable::{Direction, Properties};
use iced_style::container::Appearance;
use time::OffsetDateTime;
use tracing::info;

use settings::AppSwitcherSettings;

use crate::settings::Modules;
use crate::theme::AppSwitcherTheme;
use crate::widgets::app_widget::{AppWidget, Message as AppWidgetMessage};
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
        .with_env_filter("mecha_app_switcher=trace")
        .with_thread_names(true)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => {
            AppSwitcherSettings::default()
        }
    };

    info!(task = "read_settings", "settings initialized for app switcher {:?}", settings);

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => {
            AppSwitcherTheme::default()
        }
    };

    info!(task = "read_theme", "theme initialized for app switcher {:?}", custom_theme);

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
                    default_font = Font {
                        family: iced::font::Family::Name(Box::leak(font_name.into_boxed_str())),
                        weight: iced::font::Weight::Light,
                        stretch: iced::font::Stretch::Normal,
                        monospaced: false,
                    };
                },
                false => ()
            },
            None => (),
        },
        None => (),
    };

    AppSwitcher::run(Settings {
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
        default_font,
        //try_opengles_first: app_settings.try_opengles_first,
        // default_font: Some(&FONT_DATA),
        ..Settings::default()
    })
}

// #[derive(Debug, Deserialize, Clone, Serialize)]
// pub struct RunningApps {
//     pub app_id: String,
//     pub alias: String,
//     pub name: String,
//     pub instance_name: String,
//     pub icon: String
// }

/// # App Switcher State
///
/// This struct is the state definition of the entire application
struct AppSwitcher {
    settings: AppSwitcherSettings,
    custom_theme: AppSwitcherTheme,
    running_apps: Vec<AppWidget>,
    cpu_usage: i8,
    memory_usage: f32,
}

/// ## Message
///
/// These are the events (or messages) that update state. 
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    CpuUsageChanged(i8),
    MemoryUsageChanged(f32),
    ChildAppWidgetMessages(AppWidgetMessage),
    FontLoaded(Result<(), iced::font::Error>),
    HomePressed,
    CloseAllPressed,
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
impl Application for AppSwitcher {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => {
                AppSwitcherSettings::default()
            }
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => {
                AppSwitcherTheme::default()
            }
        };

        let modules = settings.modules.clone();

        (AppSwitcher {
            settings,
            custom_theme,
            running_apps: vec![
                AppWidget::new(String::from("app_1"), String::from("app_1"), String::from("Chromium"), String::from("Mecha instance"), String::from("./src/assets/pngs/app_chrome.png"), modules.close.icon.default.clone()),
                AppWidget::new(String::from("app_2"), String::from("app_2"), String::from("Mecha Connect"), String::from("Mecha instance"), String::from("./src/assets/pngs/app_mecha_connect.png"),modules.close.icon.default.clone()),
                AppWidget::new(String::from("app_3"), String::from("app_3"), String::from("Chromium"), String::from("Mecha instance"), String::from("./src/assets/pngs/app_chrome.png"),modules.close.icon.default.clone()),
                AppWidget::new(String::from("app_4"), String::from("app_4"), String::from("Mecha Connect"), String::from("Mecha instance"), String::from("./src/assets/pngs/app_mecha_connect.png"),modules.close.icon.default.clone()),
                AppWidget::new(String::from("app_5"), String::from("app_5"), String::from("Chromium"), String::from("Mecha instance"), String::from("./src/assets/pngs/app_chrome.png"),modules.close.icon.default.clone()),
            ],
            cpu_usage: 15,
            memory_usage: 1.2,
        }, Command::none() )
    }

    fn title(&self) -> String {
        String::from(&self.settings.title)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        info!(task = "update", "update message is {:?}", message);
        let command = match message {
            Message::CpuUsageChanged(_) => {
                Command::none()
            }
            Message::MemoryUsageChanged(_) => {
                Command::none()
            }
            Message::ChildAppWidgetMessages(msg) => {
                match msg {
                    AppWidgetMessage::AppCloseClicked(app_id) => {
                        info!(task = "app close clicked", "closing app {}", app_id);
                        let updated_apps = self.running_apps.clone().into_iter().filter(|app| app.app_id != app_id).collect();
                        self.running_apps = updated_apps;
                        info!(task = "app close clicked",  "closed app {}", app_id);
                    }
                    AppWidgetMessage::AppClicked(app_id) => {
                        info!(task = "app clicked", "app clicked {}", app_id);
                    }
                }
                Command::none()
            }
            Message::FontLoaded(_) => {
                Command::none()
            }
            Message::HomePressed => {
                Command::none()
            }
            Message::CloseAllPressed => {
                self.running_apps = vec![];
                Command::none()
            }
        };
        command
    }

    fn view(&self) -> Element<Message> {
        let background_color = self.custom_theme.background.default.clone().unwrap().color;

        let mut cpu_usage_row = Row::new();

        match self.settings.modules.cpu.icon.low.clone()  {
            Some(icon) => {
                cpu_usage_row = cpu_usage_row.push(image(icon));
            },
            None => (),
        }

        cpu_usage_row = cpu_usage_row
        .push(Text::new(" CPU ").style(iced::theme::Text::Color(Color::from_rgb8(132, 141, 166))))
        .push(Text::new(format!("{}%", self.cpu_usage))
            .font(Font {
                weight: Weight::Bold,
                ..Default::default()
            }));

        let cpu_usage = Column::new()
            .width(Length::FillPortion(1))
            .push(
                cpu_usage_row
            );

        let mut memory_row =     Row::new();

        match self.settings.modules.memory.icon.low.clone() {
            Some(icon) => {
                memory_row = memory_row.push(image(icon));
            },
            None => (),
        }

        memory_row = memory_row
                    .push(Text::new(" MEM ").style(iced::theme::Text::Color(Color::from_rgb8(132, 141, 166))))
                    .push(Text::new(format!("{}/{}", self.memory_usage, self.settings.modules.memory.total.to_owned()))
                        .font(Font {
                            weight: Weight::Bold,
                            ..Default::default()
                        }));

        let memory_usage = Column::new()
            .width(Length::FillPortion(1))
            .push(
                memory_row
            );

        let usage_row = Row::new()
            .width(Length::Fill)
            .push(cpu_usage)
            .push(memory_usage)
            ;

        //let run_app: Element<Message> = self.running_apps.view().map(Message::AppRemoved);
        // let running_apps_row: Vec<Element<Message>> = self.running_apps
        //     .iter()
        //     .map(|app| app.clone().view().map(Message::AppRemoved))
        //     .collect();

        let running_apps_row: Element<_> = row(self.running_apps.iter().map(|app| { app.view().map(Message::ChildAppWidgetMessages) }).collect())
            .spacing(16)
            .into();

        let mut footer = Row::new()
            .width(Length::Fill);

        match self.settings.modules.home.icon.default.clone() {
            Some(icon) => {
                let home_button = MouseArea::new(Column::new()
                .width(Length::FillPortion(1))
                .push(
                    image(icon)
                )).on_release(Message::HomePressed);
                footer = footer.push(home_button);
            },
            None => (),
        };

        match self.settings.modules.close_all.icon.default.clone() {
            Some(icon) => {
                let close_all_button = MouseArea::new(Column::new()
                .push(
                    image(icon)
                )).on_release(Message::CloseAllPressed);
                footer = footer.push(close_all_button);
            },
            None => todo!(),
        };

        let scrollable_apps: Element<_> = match self.running_apps.len() > 0 {
            true => Scrollable::new(running_apps_row).direction(Direction::Horizontal(Properties::new().scroller_width(0).width(0))).into(),
            false => {
                let mut no_apps_col = Column::new();

                match self.settings.modules.no_apps.icon.default.clone() {
                    Some(icon) => {
                        no_apps_col = no_apps_col
                            .push(image(icon))
                    },
                    None => (),
                }

                no_apps_col = no_apps_col
                .push(Text::new("No apps running on device"))
                .width(480)
                .align_items(Alignment::Center);
                
                Row::new().push(no_apps_col)
                .align_items(Alignment::Center)
                .width(Length::FillPortion(1))
                .height(261)
                .into()
            }
        };

        container(column![usage_row, vertical_space(30), scrollable_apps, vertical_space(30), footer])
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance {
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



