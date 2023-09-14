#![deny(clippy::all)]
use std::{fs, time::SystemTime};
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
use tracing::{info, error};
use crate::settings::Modules;
use crate::theme::StatusBarTheme;
use crate::widgets::styled_container::StyledContainer;

pub mod errors;

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 1,
    }
}


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
    wifi_state: WifiState,
    bluetooth_state: BluetoothState,
    battery_state: BatteryState
}

#[derive(Debug, Clone, Copy)]
pub enum WifiConnectedState {
    Low,
    Weak,
    Good,
    Strong
}

#[derive(Debug, Clone, Copy)]
pub enum WifiState {
    On,
    Off,
    Connected(WifiConnectedState)
}

#[derive(Debug, Clone, Copy)]
pub enum BluetoothState {
    On,
    Off,
    Connected
}

#[derive(Debug, Clone, Copy)]
pub enum BatteryState {
    Level0,
    Level10,
    Level20,
    Level30,
    Level40,
    Level50,
    Level60,
    Level70,
    Level80,
    Level90,
    Level100

}

/// ## Message
/// 
/// These are the events (or messages) that update state. 
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone, Copy)]
pub enum Message {
    TimeTick(OffsetDateTime),
    WifiStateUpdate(WifiState),
    BluetoothStateUpdate(BluetoothState),
    BatteryStatusUpdate(BatteryState)
}

#[derive(Debug, Clone)]
pub struct GenerateLayout {
    pub modules: Modules,
    pub current_time: OffsetDateTime,
    pub wifi_state: WifiState,
    pub bluetooth_state: BluetoothState,
    pub battery_state: BatteryState
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
            wifi_state: WifiState::Off,
            bluetooth_state: BluetoothState::Off,
            battery_state: BatteryState::Level0
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
            Message::WifiStateUpdate(strength) => {
                self.wifi_state = strength;
                Command::none()
            }
            Message::BluetoothStateUpdate(state) => {
                self.bluetooth_state = state;
                Command::none()
            }
            Message::BatteryStatusUpdate(state) => {
                self.battery_state = state;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let current_time_format = &self.settings.modules.clock.format;
        let format_description = format_description::parse(current_time_format).unwrap();
        let _formatted_time = match self.current_time.format(&format_description){
            Ok(t) => t,
            Err(e) => {
                error!("error while formatting time {}", e);
                String::from("")
            },
        };
        let left_row = generate_modules(GenerateLayout{
            modules: self.settings.modules.clone(),
            current_time: self.current_time,
            wifi_state: self.wifi_state,
            bluetooth_state: self.bluetooth_state,
            battery_state: self.battery_state,
        }, self.settings.layout.left.clone());
        let center_row = generate_modules(GenerateLayout{
            modules: self.settings.modules.clone(),
            current_time: self.current_time,
            wifi_state: self.wifi_state,
            bluetooth_state: self.bluetooth_state,
            battery_state: self.battery_state,
        }, self.settings.layout.center.clone());
        let right_row = generate_modules(GenerateLayout{
            modules: self.settings.modules.clone(),
            current_time: self.current_time,
            wifi_state: self.wifi_state,
            bluetooth_state: self.bluetooth_state,
            battery_state: self.battery_state,
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
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance{
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
            let wifi_states = vec![WifiState::Off, WifiState::On, WifiState::Connected(WifiConnectedState::Low),WifiState::Connected(WifiConnectedState::Weak), WifiState::Connected(WifiConnectedState::Good),WifiState::Connected(WifiConnectedState::Strong) ];
            let x = get_sys_time_in_secs() as usize % wifi_states.len();
            Message::WifiStateUpdate(wifi_states[x])
        });
        let s3 = iced::time::every(std::time::Duration::from_secs(1)).map(|_| {
            let bluetooth_states = vec![BluetoothState::Off, BluetoothState::On, BluetoothState::Connected];
            let x = get_sys_time_in_secs() as usize % bluetooth_states.len();
            Message::BluetoothStateUpdate(bluetooth_states[x])
        });
        let s4 = iced::time::every(std::time::Duration::from_secs(1)).map(|_| {
            let battery_states = vec![BatteryState::Level0, BatteryState::Level10,BatteryState::Level20, BatteryState::Level30,BatteryState::Level40,BatteryState::Level50,BatteryState::Level60,BatteryState::Level70,BatteryState::Level80,BatteryState::Level90,BatteryState::Level100];
            let x = get_sys_time_in_secs() as usize % battery_states.len();
            Message::BatteryStatusUpdate(battery_states[x])
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
            "clock" => { data_row = data_row.push( Text::new(formatted_time).size(18))},
            "wifi" => {
                match layout_details.wifi_state {
                    WifiState::Off => { 
                        if let Some(icon) = layout_details.modules.wifi.icon.off.clone() {
                            info!("wifi icon path is {}", icon);
                            data_row = data_row.push(image(&icon));
                        }
                    }
                    WifiState::On => { 
                        if let Some(icon) = layout_details.modules.wifi.icon.off.clone() {
                            info!("wifi icon path is {}", icon);
                            data_row = data_row.push(image(&icon));
                        } 
                    }
                    WifiState::Connected(WifiConnectedState::Low) => { 
                        if let Some(icon) = layout_details.modules.wifi.icon.low.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                    }
                    WifiState::Connected(WifiConnectedState::Weak) => { 
                        if let Some(icon) = layout_details.modules.wifi.icon.weak.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                    }
                    WifiState::Connected(WifiConnectedState::Good) => { 
                        if let Some(icon) = layout_details.modules.wifi.icon.good.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                    }
                    WifiState::Connected(WifiConnectedState::Strong) => { 
                        if let Some(icon) = layout_details.modules.wifi.icon.strong.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                    }
                }
            }
            ,
            "bluetooth" => {
                match layout_details.bluetooth_state {
                    BluetoothState::Off => { 
                        if let Some(icon) = layout_details.modules.bluetooth.icon.off.clone() {
                            data_row = data_row.push(image(&icon)); 
                        } 
                    }
                    BluetoothState::On => { 
                        if let Some(icon) = layout_details.modules.bluetooth.icon.on.clone() {
                            data_row = data_row.push(image(&icon)); 
                        } 
                    }
                    BluetoothState::Connected => { 
                        if let Some(icon) = layout_details.modules.bluetooth.icon.connected.clone() {
                            data_row = data_row.push(image(&icon)); 
                        } 
                     }
                }
            }
            ,
            "battery" => {
                match layout_details.battery_state {
                    BatteryState::Level0 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_0.clone() {
                            data_row = data_row.push(image(&icon)); 
                        } 
                    }
                    BatteryState::Level10 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_10.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level20 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_20.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level30 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_30.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level40 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_40.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level50 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_50.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level60 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_60.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level70 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_70.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level80 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_80.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level90 => { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_90.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
                     BatteryState::Level100=> { 
                        if let Some(icon) = layout_details.modules.battery.icon.level_100.clone() {
                            data_row = data_row.push(image(&icon)); 
                        }
                     }
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
