use std::borrow::BorrowMut;
use std::fmt;

use greetd_ipc::Response;
use gtk::{glib::clone, prelude::GtkWindowExt};
use handlers::login::handler::{LoginHandler, LoginHandlerMessage};
use relm4::component::{
    AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController,
};
use relm4::gtk::LayoutManager;
use relm4::{
    async_trait::async_trait, gtk, AsyncComponentSender, ComponentParts, ComponentSender, RelmApp,
    SimpleComponent,
};
use relm4::{Component, ComponentController, Controller};

mod pages;
mod settings;
mod theme;
mod users;
mod widgets;
use pages::{
    home::{HomePage, Message as HomePageMessage, Settings as HomePageSettings},
    password_authentication::{
        Message as PasswordAuthenticationMessage, PasswordAuthentication,
        Settings as PasswordAuthenticationSettings,
    },
    pin_authentication::{
        Message as PinAuthenticationMessage, PinAuthentication,
        Settings as PinAuthenticationSettings,
    },
    power_options::{
        Message as PowerOptionsMessage, PowerOptions, Settings as PowerOptionsSettings,
    },
};
use settings::{LayoutSettings, Modules};
use tokio::sync::mpsc::{self, Sender};
use tracing::{error, info};
pub mod errors;
mod handlers;

use crate::settings::GreeterSettings;
use crate::theme::GreeterTheme;

/// # Greeter State
///
/// This struct is the state definition of the entire application
pub struct Greeter {
    settings: GreeterSettings,
    custom_theme: GreeterTheme,
    current_screen: Screens,
    home_page: Controller<HomePage>,
    pin_authentication_page: AsyncController<PinAuthentication>,
    password_authentication_page: AsyncController<PasswordAuthentication>,
    power_options: Controller<PowerOptions>,
    screens_stack: gtk::Stack,
    login_handler_sender: Option<mpsc::Sender<LoginHandlerMessage>>,
}

#[derive(Debug, Clone)]
pub enum Screens {
    HomeScreen,
    PasswordScreen,
    PinScreen { username: Option<String> },
    PowerOptions,
}

impl fmt::Display for Screens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Screens::HomeScreen => write!(f, "home_screen"),
            Screens::PasswordScreen => write!(f, "password_screen"),
            Screens::PinScreen { username } => write!(f, "pin_screen"),
            Screens::PowerOptions => write!(f, "power_options"),
        }
    }
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    ChangeScreen(Screens),
    Dummy,
}

pub struct AppWidgets {
    screens_stack: gtk::Stack,
}

#[cfg(not(feature = "layer-shell"))]
fn init_window(settings: GreeterSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();
    window
}

#[cfg(feature = "layer-shell")]
fn init_window(settings: GreeterSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();

    gtk4_layer_shell::init_for_window(&window);

    // Display above normal windows
    gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);
    gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);

    gtk4_layer_shell::set_keyboard_mode(&window, gtk4_layer_shell::KeyboardMode::OnDemand);

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (gtk4_layer_shell::Edge::Left, true),
        (gtk4_layer_shell::Edge::Right, true),
        (gtk4_layer_shell::Edge::Top, true),
        (gtk4_layer_shell::Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        gtk4_layer_shell::set_anchor(&window, anchor, state);
    }

    window
}

#[derive(Debug)]
pub enum Prompt {
    Captcha { message: String },
    Password { message: String },
}

#[derive(Debug)]
pub enum CommandMsg {
    ShowErr(String),
    ClearErr,
    HandleGreetdResponse(Response),
    Prompts(Prompt),
    AuthError,
}

#[async_trait(?Send)]
impl AsyncComponent for Greeter {
    /// The type of the messages that this component can receive.
    type Input = Message;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    type CommandOutput = CommandMsg;

    fn init_root() -> Self::Root {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => GreeterSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for Lock Screen: {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => GreeterTheme::default(),
        };

        info!(
            task = "initalize_theme",
            "theme initialized for Lock Screen: {:?}", custom_theme
        );

        let window = init_window(settings);
        window
    }

    /// Initialize the UI and model.
    async fn init(
        _: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let icon_theme = gtk::IconTheme::builder().build();
        info!("icon paths are {:?}", icon_theme.resource_path());
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => GreeterSettings::default(),
        };

        let css = settings.css.clone();
        relm4::set_global_css_from_file(css.default);

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => GreeterTheme::default(),
        };

        let modules = settings.modules.clone();
        let layout = settings.layout.clone();

        //Stack used to render different screens
        //At a time one screen will be rendered
        let screens_stack = gtk::Stack::builder().build();

        let home_page = HomePage::builder()
            .launch(HomePageSettings {
                lock_icon: modules.lock.icon.default.to_owned(),
                unlock_icon: modules.unlock.icon.default.to_owned(),
                password_icon: modules.power.icon.default.to_owned(),
            })
            .forward(
                sender.input_sender(),
                clone!(@strong modules => move|msg| match msg {
                    HomePageMessage::ChangeScreen(screen) => Message::ChangeScreen(screen),
                    _ => Message::Dummy
                }),
            );

        let power_options = create_power_options(
            modules.clone(),
            layout.clone(),
            sender.input_sender().clone(),
        );

        let pin_authentication_page = create_pin_authentication_page(
            modules.clone(),
            layout.clone(),
            sender.input_sender().clone(),
            None,
            None,
        );

        let password_authentication_page = create_password_authentication_page(
            modules.clone(),
            layout.clone(),
            sender.input_sender().clone(),
            None,
        );

        //Adding home page screeen in stack
        screens_stack.add_named(
            home_page.widget(),
            Option::from(Screens::HomeScreen.to_string().as_str()),
        );

        screens_stack.add_named(
            power_options.widget(),
            Option::from(Screens::PowerOptions.to_string().as_str()),
        );

        //Adding auth screeen in stack
        screens_stack.add_named(
            pin_authentication_page.widget(),
            Option::from(Screens::PinScreen { username: None }.to_string().as_str()),
        );

        //Adding password screeen in stack
        screens_stack.add_named(
            password_authentication_page.widget(),
            Option::from(Screens::PasswordScreen.to_string().as_str()),
        );

        let current_screen = Screens::HomeScreen;

        //Setting current active screen in stack
        screens_stack.set_visible_child_name(&current_screen.to_string());
        screens_stack.set_transition_type(gtk::StackTransitionType::Crossfade);
        screens_stack.set_transition_duration(300);
        //Adding stack to window
        window.set_child(Some(&screens_stack));

        let (login_handler_sender) = init_services(sender.clone()).await;

        let model = Greeter {
            settings,
            custom_theme,
            current_screen,
            home_page,
            pin_authentication_page,
            password_authentication_page,
            power_options,
            screens_stack: screens_stack.clone(),
            login_handler_sender: Some(login_handler_sender),
        };

        let widgets = AppWidgets { screens_stack };

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        info!("Update message is {:?}", message);
        match message {
            Message::ChangeScreen(screen) => {
                match &screen {
                    Screens::PinScreen { username } => {
                        self.pin_authentication_page.detach_runtime();
                        let pin_authentication_page = create_pin_authentication_page(
                            self.settings.modules.clone(),
                            self.settings.layout.clone(),
                            sender.input_sender().clone(),
                            self.login_handler_sender.clone(),
                            username.clone(),
                        );
                        self.screens_stack.remove(
                            &self
                                .screens_stack
                                .child_by_name(
                                    Screens::PinScreen { username: None }.to_string().as_str(),
                                )
                                .unwrap(),
                        );
                        self.screens_stack.add_named(
                            pin_authentication_page.widget(),
                            Option::from(
                                Screens::PinScreen { username: None }.to_string().as_str(),
                            ),
                        );
                        self.pin_authentication_page = pin_authentication_page;

                        let _ = self
                            .pin_authentication_page
                            .sender()
                            .send(PinAuthenticationMessage::Reset);
                    }
                    Screens::PasswordScreen => {
                        self.password_authentication_page.detach_runtime();
                        let password_authentication_page = create_password_authentication_page(
                            self.settings.modules.clone(),
                            self.settings.layout.clone(),
                            sender.input_sender().clone(),
                            self.login_handler_sender.clone(),
                        );
                        self.screens_stack.remove(
                            &self
                                .screens_stack
                                .child_by_name(Screens::PasswordScreen.to_string().as_str())
                                .unwrap(),
                        );
                        self.screens_stack.add_named(
                            password_authentication_page.widget(),
                            Option::from(Screens::PasswordScreen.to_string().as_str()),
                        );
                        self.password_authentication_page = password_authentication_page;
                    }
                    Screens::PowerOptions => {
                        self.power_options.detach_runtime();
                        let power_options = create_power_options(
                            self.settings.modules.clone(),
                            self.settings.layout.clone(),
                            sender.input_sender().clone(),
                        );
                        self.screens_stack.remove(
                            &self
                                .screens_stack
                                .child_by_name(Screens::PowerOptions.to_string().as_str())
                                .unwrap(),
                        );
                        self.screens_stack.add_named(
                            power_options.widget(),
                            Option::from(Screens::PowerOptions.to_string().as_str()),
                        );
                        self.power_options = power_options;
                        let _ = self.power_options.sender().send(PowerOptionsMessage::Reset);
                    }
                    _ => {}
                }
                self.current_screen = screen;
            }
            _ => (),
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        //updating stack screen when current screen changes
        widgets
            .screens_stack
            .set_visible_child_name(self.current_screen.to_string().as_str());
    }

    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        match self.current_screen {
            Screens::PasswordScreen => {
                let _ = self
                    .password_authentication_page
                    .sender()
                    .send(PasswordAuthenticationMessage::Command(message));
            }
            Screens::PinScreen { username: None } => {
                let _ = self
                    .pin_authentication_page
                    .sender()
                    .send(PinAuthenticationMessage::Command(message));
            }
            _ => (),
        }
    }
}

fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_greeter=trace")
        .with_thread_names(true)
        .init();
    let app = RelmApp::new("lock.screen").with_args(vec![]);
    app.run_async::<Greeter>(());
}

fn create_pin_authentication_page(
    modules: Modules,
    layout: LayoutSettings,
    sender: relm4::Sender<Message>,
    login_handler_sender: Option<mpsc::Sender<LoginHandlerMessage>>,
    username: Option<String>,
) -> AsyncController<PinAuthentication> {
    let pin_authentication_page = PinAuthentication::builder()
        .launch(PinAuthenticationSettings {
            modules: modules,
            layout: layout,
            login_handler_sender,
            username: username.unwrap_or("".to_string()),
        })
        .forward(&sender, move |msg| {
            info!("pin page message to parent {:?}", msg);
            match msg {
                PinAuthenticationMessage::BackPressed => Message::ChangeScreen(Screens::HomeScreen),
                _ => Message::Dummy,
            }
        });
    pin_authentication_page
}

fn create_password_authentication_page(
    modules: Modules,
    layout: LayoutSettings,
    sender: relm4::Sender<Message>,
    login_handler_sender: Option<mpsc::Sender<LoginHandlerMessage>>,
) -> AsyncController<PasswordAuthentication> {
    let password_authentication_page = PasswordAuthentication::builder()
        .launch(PasswordAuthenticationSettings {
            modules: modules.clone(),
            layout: layout.clone(),
            login_handler_sender,
        })
        .forward(&sender, move |msg| {
            info!("password page message to parent {:?}", msg);
            match msg {
                PasswordAuthenticationMessage::BackPressed => {
                    Message::ChangeScreen(Screens::HomeScreen)
                }
                _ => Message::Dummy,
            }
        });
    password_authentication_page
}

fn create_power_options(
    modules: Modules,
    layout: LayoutSettings,
    sender: relm4::Sender<Message>,
) -> Controller<PowerOptions> {
    let power_options = PowerOptions::builder()
        .launch(PowerOptionsSettings {
            close_icon: modules.close.icon.default.to_owned(),
            shutdown_icon: modules.shutdown.icon.default.to_owned(),
            restart_icon: modules.restart.icon.default.to_owned(),
            sleep_icon: modules.sleep.icon.default.to_owned(),
        })
        .forward(
            &sender,
            clone!(@strong modules => move|msg| match msg {
                PowerOptionsMessage::ChangeScreen(screen) => Message::ChangeScreen(screen),
                _ => Message::Dummy
            }),
        );
    power_options
}

async fn init_services(sender: AsyncComponentSender<Greeter>) -> (Sender<LoginHandlerMessage>) {
    let (login_message_tx, login_message_rx) = mpsc::channel(32);

    let mut login_handler = LoginHandler::new().await.unwrap();

    let _ = relm4::spawn_local(async move {
        let _ = login_handler.run(login_message_rx, sender).await;
    });

    (login_message_tx)
}
