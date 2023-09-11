#![deny(clippy::all)]
use std::fs;
use iced::{executor, widget::{container, row, Text, column, text}, window, Application, Element, Settings, Background, Color, Alignment, Renderer};

use iced_native::{Command, Length, Subscription, Theme};
use settings::StatusBarSettings;
use iced_native::widget::image;
use iced_style::container::Appearance;
use lazy_static::lazy_static;


use time::{format_description, OffsetDateTime};
mod settings;
mod widgets;
mod theme;
use tracing::info;
use crate::settings::Modules;
use crate::theme::StatusBarTheme;
use crate::widgets::custom_container::CustomContainer;

pub mod errors;


/// Initialize the application with settings, and starts
pub fn main() -> iced::Result {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_status_bar=trace")
        .with_thread_names(true)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => {
            StatusBarSettings::default()
        }
    };

    info!(task = "initalize_settings", "settings initialized for status bar: {:?}", settings);

    let _custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => {
            StatusBarTheme::default()
        }
    };

    info!(task = "initalize_theme", "theme initialized for status bar: {:?}", _custom_theme);

    let window_settings = settings.window;
    let app_settings = settings.app;
    let position = window_settings.position;
    lazy_static! {
        static ref FONT_DATA: Vec<u8> = fs::read("src/assets/fonts/spaces-grotesk.ttf").expect("Failed to read font data");
    }

    StatusBar::run(Settings {
        window: window::Settings {
            size: window_settings.size,
            position: window::Position::Specific(position.0, position.1),
            min_size: window_settings.min_size,
            max_size: window_settings.max_size,
            visible: window_settings.visible,
            resizable: window_settings.resizable,
            decorations: window_settings.decorations,
            transparent: window_settings.transparent,
            always_on_top: window_settings.always_on_top,
            ..Default::default()
        },
        id: app_settings.id,
        text_multithreading: app_settings.text_multithreading,
        antialiasing: app_settings.antialiasing,
        try_opengles_first: app_settings.try_opengles_first,
        default_font: Some(&FONT_DATA),
        ..Settings::default()
    })
}

/// # Status Bar state
/// 
/// This struct is the state definition of the entire application
struct StatusBar {
    settings: StatusBarSettings,
    current_time: OffsetDateTime,
    custom_theme: StatusBarTheme,
    wifi_strength: i8,
    bluetooth_state: i8,
    battery_level: u8
}

/// ## Message
/// 
/// These are the events (or messages) that update state. 
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone, Copy)]
pub enum Message {
    TimeTick(OffsetDateTime),
    WifiStrengthUpdate(i8),
    BluetoothStatusUpdate(i8),
    BatteryStatusUpdate(u8)
}

#[derive(Debug, Clone)]
pub struct GenerateLayout {
    pub modules: Modules,
    pub current_time: OffsetDateTime,
    pub wifi_strength: i8,
    pub bluetooth_state: i8,
    pub battery_level: u8
}

/// # Top-level Application implementation for Iced
/// 
/// Primary components includes -
/// - `new()` - initializes the state
/// - `update()` - maps all the actions corresponding to 'Message'
/// - `view()` - the view definition
impl Application for StatusBar {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => {
                StatusBarSettings::default()
            }
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => {
                StatusBarTheme::default()
            }
        };
    
        (StatusBar {
            settings,
            custom_theme,
            current_time: OffsetDateTime::now_local()
            .unwrap_or_else(|_| OffsetDateTime::now_utc()),
            wifi_strength: -1,
            bluetooth_state: -1,
            battery_level: 0
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from(&self.settings.title)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TimeTick(local_time) => {
                let now: OffsetDateTime = local_time;

                if now != self.current_time {
                    self.current_time = now;
                }
                Command::none()
            }
            Message::WifiStrengthUpdate(strength) => {
                self.wifi_strength = strength;
                Command::none()
            }
            Message::BluetoothStatusUpdate(state) => {
                self.bluetooth_state = state;
                Command::none()
            }
            Message::BatteryStatusUpdate(state) => {
                self.battery_level = state;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let current_time_format = &self.settings.modules.clock.format;
        let format_description = format_description::parse(current_time_format).unwrap();
        let _formatted_time = self.current_time.format(&format_description).unwrap();
        let left_row = generate_modules(GenerateLayout{
            modules: self.settings.modules.clone(),
            current_time: self.current_time,
            wifi_strength: self.wifi_strength,
            bluetooth_state: self.bluetooth_state,
            battery_level: self.battery_level,
        }, self.settings.layout.left.clone());
        let center_row = generate_modules(GenerateLayout{
            modules: self.settings.modules.clone(),
            current_time: self.current_time,
            wifi_strength: self.wifi_strength,
            bluetooth_state: self.bluetooth_state,
            battery_level: self.battery_level,
        }, self.settings.layout.center.clone());
        let right_row = generate_modules(GenerateLayout{
            modules: self.settings.modules.clone(),
            current_time: self.current_time,
            wifi_strength: self.wifi_strength,
            bluetooth_state: self.bluetooth_state,
            battery_level: self.battery_level,
        }, self.settings.layout.right.clone());

        let row_content = row![
                    column![left_row.spacing(10)].width(Length::Fill),
                    column![center_row].width(Length::Fill),
                    column![right_row.spacing(28).align_items(Alignment::Center)]
                    .width(Length::Fill)
                    .align_items(Alignment::End)
        ].width(Length::Fill).align_items(Alignment::Center);

        let background_color = self.custom_theme.background.default.clone().unwrap().color;

        container(row_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .style(iced::theme::Container::Custom(Box::new(CustomContainer::new(Appearance{
                background: Option::from(Background::Color(Color::from_rgb8(background_color[0], background_color[1], background_color[2]))),
                ..Default::default()
            }))))
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {

        let s1 = iced::time::every(std::time::Duration::from_millis(500)).map(move |_instant| {
            Message::TimeTick(
                time::OffsetDateTime::now_local()
                    .unwrap_or_else(|_| time::OffsetDateTime::now_utc()),
            )
        });
        let s2 = iced::time::every(std::time::Duration::from_secs(1)).map(|_| {
            let wifi_strengths: Vec<i8> = vec![-1, 0, 11, 21, 41, 81];
            let x = match time::OffsetDateTime::now_local() {
                Ok(time) => {
                    time.second() % 6
                },
                Err(_) => {
                    0
                }
            };
            Message::WifiStrengthUpdate(wifi_strengths[x as usize],)
        });
        let s3 = iced::time::every(std::time::Duration::from_secs(1)).map(|_| {
            let bluetooth_states: Vec<i8> = vec![-1, 0, 1];
            let x = match time::OffsetDateTime::now_local() {
                Ok(time) => {
                    time.second() % 3
                },
                Err(_) => {
                    0
                }
            };
            Message::BluetoothStatusUpdate(bluetooth_states[x as usize],)
        });
        let s4 = iced::time::every(std::time::Duration::from_secs(1)).map(|_| {
            let states: Vec<i8> = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
            let x = match time::OffsetDateTime::now_local() {
                Ok(time) => {
                    time.second() % 10
                },
                Err(_) => {
                    0
                }
            };
            Message::BatteryStatusUpdate(states[x as usize] as u8,)
        });
        Subscription::batch([s1, s2, s3, s4])
    }
}

fn generate_modules(layout_details: GenerateLayout, layout: Vec<String>) -> iced::widget::Row<'static, Message, Renderer> {
    let mut data_row = row![];
    for layout_item in layout.iter() {
        let current_time_format = &layout_details.modules.clock.format;
        let format_description = format_description::parse(current_time_format).unwrap();
        let formatted_time = layout_details.current_time.format(&format_description).unwrap();
        match layout_item.as_str() {
            "clock" => { data_row = data_row.push( Text::new(formatted_time.to_owned()).size(18))},
            "wifi" => {
                match layout_details.wifi_strength {
                    -1 => { data_row = data_row.push(image( &layout_details.modules.wifi.icon.off)); }
                    0 => { data_row = data_row.push(image(&layout_details.modules.wifi.icon.off)); }
                    1..=20 => { data_row = data_row.push(image(&layout_details.modules.wifi.icon.low)); }
                    21..=40 => { data_row = data_row.push(image(&layout_details.modules.wifi.icon.weak)); }
                    41..=80 => { data_row = data_row.push(image(&layout_details.modules.wifi.icon.good)); }
                    81..=100 => { data_row = data_row.push(image(&layout_details.modules.wifi.icon.strong)); }
                    _ => {  }
                }
            },
            "bluetooth" => {
                match layout_details.bluetooth_state {
                    -1 => { data_row = data_row.push(image(&layout_details.modules.bluetooth.icon.off)); }
                    0 => { data_row = data_row.push(image(&layout_details.modules.bluetooth.icon.on)); }
                    1 => { data_row = data_row.push(image(&layout_details.modules.bluetooth.icon.connected)); }
                    _ => {}
                }
            },
            "battery" => {
                match layout_details.battery_level {
                    0..=9 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_0)); }
                    10..=19 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_10)); }
                    20..=29 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_20)); }
                    30..=39 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_30)); }
                    40..=49 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_40)); }
                    50..=59 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_50)); }
                    60..=69 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_60)); }
                    70..=79 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_70)); }
                    80..=89 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_80)); }
                    90..=99 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_90)); }
                    100 => { data_row = data_row.push(image(&layout_details.modules.battery.icon.level_100)); }
                    _ => {}
                }
            },
            "window_title" => {
                data_row = data_row.push(text("App title"));
            },
            _ => println!("Component not found for: {}", layout_item),
        };
    }
    data_row
}
