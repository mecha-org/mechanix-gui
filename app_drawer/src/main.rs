use std::fs;
use font_loader::system_fonts;
use iced::{executor, widget::{image, container, Scrollable, text, row, column}, window, Application, Element, Settings, Background, Color, Alignment, Font, Renderer, Padding};
use iced::{Command, Length, Subscription, Theme};
use iced::widget::text_input;
use iced::widget::scrollable::{Direction, Properties};
use settings::AppDrawerSettings;
use iced_style::container::Appearance;
use iced_aw::wrap::Wrap;
use iced_style::core::BorderRadius;

mod settings;
mod widgets;
mod theme;
use tracing::info;
use crate::settings::{App, Modules};
use crate::theme::AppDrawerTheme;
use crate::widgets::styled_container::StyledContainer;
use crate::widgets::styled_text_input::StyledTextInput;

pub mod errors;


/// Initialize the application with settings, and starts
pub fn main() -> iced::Result {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_app_drawer=trace")
        .with_thread_names(true)
        .init();

    let settings = match settings::read_settings_yml() {
        Ok(settings) => settings,
        Err(_) => {
            AppDrawerSettings::default()
        }
    };

    info!(task = "init_settings", "settings initialized for app drawer {:?}", settings);

    let custom_theme = match theme::read_theme_yml() {
        Ok(theme) => theme,
        Err(_) => {
            AppDrawerTheme::default()
        }
    };

    info!(task = "init_theme", "theme initialized for app drawer {:?}", custom_theme);

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

    AppDrawer::run(Settings {
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
        //try_opengles_first: app_settings.try_opengles_first,
        default_font,
        ..Settings::default()
    })
}

/// # AppDrawer State
/// 
/// This struct is the state definition of the entire application
struct AppDrawer {
    settings: AppDrawerSettings,
    custom_theme: AppDrawerTheme,
    search_text: String
}

/// ## Message
/// 
/// These are the events (or messages) that update state. 
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    SearchTextChanged(String),
    SearchTextSearched
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
impl Application for AppDrawer {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => {
                AppDrawerSettings::default()
            }
        };
        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => {
                AppDrawerTheme::default()
            }
        };
    
        (AppDrawer {
            settings,
            custom_theme,
            search_text: String::from("")
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from(&self.settings.title)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SearchTextChanged(term) => {
                self.search_text = term;
                Command::none()
            },
            Message::SearchTextSearched => {
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let search_input = text_input("Search application", &self.search_text).padding(14).on_input(Message::SearchTextChanged)
            .on_submit(Message::SearchTextSearched).style(iced::theme::TextInput::Custom(Box::new(StyledTextInput::new())));
        let app_row = generate_apps_grid(self.settings.modules.apps.clone(), self.search_text.clone());

        let search_icon_container = container(image("src/assets/pngs/search_icon.png"))
            .padding(Padding::from([16,0,16,16]))
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance{
                background: Option::from(Background::Color(Color::from_rgba8(21, 23, 29, 1.0))),
                border_radius: BorderRadius::from([8.0, 0.0, 0.0, 8.0]),
                ..Default::default()
            }))));

        let background_color = self.custom_theme.background.default.clone().unwrap().color;

        container(column![row![column![search_icon_container],search_input].align_items(Alignment::Center), app_row].spacing(12))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(12)
            .style(iced::theme::Container::Custom(Box::new(StyledContainer::new(Appearance{
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

fn generate_apps_ui(apps: App) -> Element<'static, Message, Renderer> {
    let app_name = text(&apps.name).size(11);
    column![ image(&apps.icon).height(90).width(98), app_name].spacing(12).align_items(Alignment::Center).into()
}
fn generate_apps_grid<'a>(
    apps: Vec<App>,
    search_text: String
) -> Element<'static, Message, Renderer> {
    let wrapped_apps =
        Wrap::with_elements(apps.into_iter().filter(|app| app.name.to_lowercase().starts_with(&search_text)).map(|app| generate_apps_ui(app)).collect())
            .line_spacing(16.0)
            .spacing(14.0);
            // .padding(5.0);;
    let scrollable_apps = Scrollable::new(wrapped_apps).direction(Direction::Vertical(Properties::new().scroller_width(0).width(0)));
    column!(scrollable_apps)
        .width(Length::Fill)
        .padding(Padding::from([10, 0]))
        .into()
}
