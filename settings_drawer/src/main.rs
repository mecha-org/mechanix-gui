use font_loader::system_fonts;
use iced::{Application, Background, Color, Element, executor, Font, Settings, widget::{column, container, Scrollable}, window};
use iced::{Command, Length, Subscription, Theme};
use iced::widget::scrollable::{Direction, Properties};
use iced_aw::wrap::Wrap;
use iced_style::container::Appearance;
use tracing::info;

use settings::SettingsDrawerSettings;
use crate::settings::Modules;
use crate::theme::SettingsDrawerTheme;
use crate::widgets::action_widget::{Message as PercentageWidgetMessage, PercentageWidget};
use crate::widgets::styled_container::StyledContainer;
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
        .with_env_filter("mecha_settings_drawer=trace")
        .with_thread_names(true)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => {
            SettingsDrawerSettings::default()
        }
    };

    info!(task = "initalize_settings", "settings initialized for settings drawer: {:?}", settings);

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => {
            SettingsDrawerTheme::default()
        }
    };

    info!(task = "initalize_theme", "theme initialized for settings drawer: {:?}", custom_theme);

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

    Launcher::run(Settings {
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
        ..Settings::default()
    })
}

#[derive(Debug, Copy, Clone)]
pub enum WifiState{
    On,
    Off,
    Connected
}

#[derive(Debug, Copy, Clone)]
pub enum BluetoothState{
    On,
    Off,
    Connected
}


/// # Launcher State
///
/// This struct is the state definition of the entire application
struct Launcher {
    settings: SettingsDrawerSettings,
    custom_theme: SettingsDrawerTheme,
    wifi_state: WifiState,
    bluetooth_state: BluetoothState,
    battery_action: PercentageWidget,
    cpu_action: PercentageWidget,
    memory_action: PercentageWidget,
    running_apps_action: PercentageWidget,
    wifi_action: PercentageWidget,
    bluetooth_action: PercentageWidget,
    auto_rotate_action: PercentageWidget,
    settings_action: PercentageWidget,
    sound_action: SliderWidget,
    brigtness_action: SliderWidget,
}

/// ## Message
///
/// These are the events (or messages) that update state. 
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    CpuStatusChanged(PercentageWidgetMessage),
    BatteryStatusChanged(PercentageWidgetMessage),
    MemoryStatusChanged(PercentageWidgetMessage),
    RunningAppsChanged(PercentageWidgetMessage),
    WifiStatusChanged(PercentageWidgetMessage),
    BluetoothStatusChanged(PercentageWidgetMessage),
    AutoRotateStatusChanged(PercentageWidgetMessage),
    SettingsStatusChanged(PercentageWidgetMessage),
    SoundStatusChanged(SliderWidgetMessage),
    BrightnessStatusChanged(SliderWidgetMessage),
    FontLoaded(Result<(), iced::font::Error>),
}

#[derive(Debug, Clone)]
pub struct GenerateLayout {
    pub modules: Modules,
}

/// # Top-level Application implementation for Iced
///
/// Primary components includes -
/// - `new()` - initializes the state
/// - `update()` - maps all the actions corresponding to 'Message'
/// - `view()` - the view definition
impl Application for Launcher {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => {
                SettingsDrawerSettings::default()
            }
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => {
                SettingsDrawerTheme::default()
            }
        };

        let modules = settings.modules.clone();

        (Launcher {
            settings,
            custom_theme,
            battery_action: PercentageWidget::new(modules.battery.title.to_owned(), modules.battery.icon.level_70.to_owned(), Option::from(70), String::from("%")),
            cpu_action: PercentageWidget::new(modules.cpu.title.to_owned(), modules.cpu.icon.low.to_owned(), Option::from(15), String::from("%")),
            memory_action: PercentageWidget::new(modules.memory.title.to_owned(), modules.memory.icon.low.to_owned(), Option::from(32), String::from("%")),
            running_apps_action: PercentageWidget::new(modules.running_apps.title.to_owned(), modules.running_apps.icon.low.to_owned(), Option::from(7), String::from("")),
            wifi_action: PercentageWidget::new(modules.wifi.title.to_owned(), modules.wifi.icon.strong.to_owned(), None, String::from("")),
            bluetooth_action: PercentageWidget::new(modules.bluetooth.title.to_owned(), modules.bluetooth.icon.on.to_owned(), None, String::from("")),
            auto_rotate_action: PercentageWidget::new(modules.auto_rotate.title.to_owned(), modules.auto_rotate.icon.portrait.to_owned(), None, String::from("")),
            settings_action: PercentageWidget::new(modules.settings.title.to_owned(), modules.settings.icon.default.to_owned(), None, String::from("")),
            sound_action: SliderWidget::new(modules.sound.title.to_owned(), "".to_string(), 20.0),
            brigtness_action: SliderWidget::new(modules.brightness.title.to_owned(), "".to_string(), 70.0),
            wifi_state: WifiState::Off,
            bluetooth_state: BluetoothState::Off,
        }, Command::none(), )
    }

    fn title(&self) -> String {
        String::from(&self.settings.title)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        info!(task = "update", "update message is {:?}", message);
        let command = match message {
            Message::BatteryStatusChanged(code) => {
                self.battery_action.update(code).map(Message::BatteryStatusChanged)
            }
            Message::CpuStatusChanged(code) => {
                self.cpu_action.update(code).map(Message::CpuStatusChanged)
            }
            Message::MemoryStatusChanged(code) => {
                self.memory_action.update(code).map(Message::MemoryStatusChanged)
            }
            Message::RunningAppsChanged(code) => {
                self.running_apps_action.update(code).map(Message::RunningAppsChanged)
            }
            Message::FontLoaded(_) => {
                info!(task = "load_font", "fond loaded successfully");
                Command::none()
            }
            Message::WifiStatusChanged(code) => {
                match code {
                    PercentageWidgetMessage::WidgetClicked => {
                        info!(task = "wifi status changed", "wifi status change code is {:?}", code);
                        let mut updated_message: PercentageWidgetMessage;

                        match self.wifi_state {
                            WifiState::On => {
                                self.wifi_state = WifiState::Off;
                                updated_message = PercentageWidgetMessage::IconChanged(self.settings.modules.wifi.icon.off.clone());
                            },
                            WifiState::Off => {
                                self.wifi_state = WifiState::On;
                                updated_message = PercentageWidgetMessage::IconChanged(self.settings.modules.wifi.icon.on.clone());
                            },
                            WifiState::Connected => {
                                self.wifi_state = WifiState::Off;
                                updated_message = PercentageWidgetMessage::IconChanged(self.settings.modules.wifi.icon.off.clone());
                            },
                        }    
                        self.wifi_action.update(updated_message).map(Message::WifiStatusChanged)
                    }
                    _ => {
                        Command::none()
                    }
                }
            }
            Message::BluetoothStatusChanged(code) => {
                match code {
                    PercentageWidgetMessage::WidgetClicked => {
                        info!(task = "bluetooth status changed", "bluetooth status change code is {:?}", code);
                        let mut updated_message: PercentageWidgetMessage;

                        match self.bluetooth_state {
                            BluetoothState::On => {
                                self.bluetooth_state = BluetoothState::Off;
                                updated_message = PercentageWidgetMessage::IconChanged(self.settings.modules.bluetooth.icon.off.clone());
                            },
                            BluetoothState::Off => {
                                self.bluetooth_state = BluetoothState::On;
                                updated_message = PercentageWidgetMessage::IconChanged(self.settings.modules.bluetooth.icon.on.clone());
                            },
                            BluetoothState::Connected => {
                                self.bluetooth_state = BluetoothState::Off;
                                updated_message = PercentageWidgetMessage::IconChanged(self.settings.modules.bluetooth.icon.off.clone());
                            },
                        }
                        self.bluetooth_action.update(updated_message).map(Message::BluetoothStatusChanged)
                    }
                    _ => {
                        Command::none()
                    }
                }
            }
            Message::AutoRotateStatusChanged(code) => {
                self.auto_rotate_action.update(code).map(Message::AutoRotateStatusChanged)
            }
            Message::SettingsStatusChanged(code) => {
                self.settings_action.update(code).map(Message::SettingsStatusChanged)
            }
            Message::SoundStatusChanged(code) => {
                self.sound_action.update(code).map(Message::SoundStatusChanged)
            }
            Message::BrightnessStatusChanged(code) => {
                self.brigtness_action.update(code).map(Message::BrightnessStatusChanged)
            }
        };
        command
    }

    fn view(&self) -> Element<Message> {
        // let app_row = generate_apps_grid(self.settings.modules.apps.clone(), self.search_text.clone());

        let background_color = self.custom_theme.background.default.clone().unwrap().color;

        // let row_1 = row![wifi_ui, bluetooth_ui, battery_ui, auto_rotate_ui ].spacing(12);
        //
        // let row_2 = row![settings_ui, running_apps_ui, cpu_ui, memory_ui].spacing(12);
        //
        // let row_3 = row![sounds_ui, brightness_ui].spacing(12);

        let wifi_title = self.wifi_action.title.to_owned().as_str();
        let bluetooth_title = self.bluetooth_action.title.to_owned();
        let battery_title = self.battery_action.title.to_owned();
        let auto_rotate_title = self.auto_rotate_action.title.to_owned();
        let settings_title = self.settings_action.title.to_owned();
        let running_apps_title = self.running_apps_action.title.to_owned();
        let cpu_title = self.cpu_action.title.to_owned();
        let memory_title = self.memory_action.title.to_owned();
        let sound_title = self.sound_action.title.to_owned();
        let brightness_title = self.brigtness_action.title.to_owned();

        let wrapped_apps =

            Wrap::with_elements(self.settings.layout.grid.clone().into_iter().map(|title| {
                if self.wifi_action.title == title {
                    self.wifi_action.view().map(Message::WifiStatusChanged)
                } else if self.bluetooth_action.title == title {
                    self.bluetooth_action.view().map(Message::BluetoothStatusChanged)
                } else if self.battery_action.title == title {
                    self.battery_action.view().map(Message::BatteryStatusChanged)
                } else if self.auto_rotate_action.title == title {
                    self.auto_rotate_action.view().map(Message::AutoRotateStatusChanged)
                } else if self.settings_action.title == title {
                    self.settings_action.view().map(Message::SettingsStatusChanged)
                } else if self.running_apps_action.title == title {
                    self.running_apps_action.view().map(Message::RunningAppsChanged)
                } else if self.cpu_action.title == title {
                    self.cpu_action.view().map(Message::CpuStatusChanged)
                } else if self.memory_action.title == title {
                    self.memory_action.view().map(Message::MemoryStatusChanged)
                } else if self.sound_action.title == title {
                    self.sound_action.view().map(Message::SoundStatusChanged)
                } else if self.brigtness_action.title == title {
                    self.brigtness_action.view().map(Message::BrightnessStatusChanged)
                } else {
                    self.wifi_action.view().map(Message::WifiStatusChanged)
                }
            }).collect())
                .line_spacing(12.0)
                .spacing(12.0);
        // .padding(5.0);;
        let scrollable_apps = Scrollable::new(wrapped_apps).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0)));
        // column!(scrollable_apps)
        //     .width(Length::Fill)
        //     .padding(Padding::from([10, 0]))
        //     .into()

        container(column![scrollable_apps].spacing(11))
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



