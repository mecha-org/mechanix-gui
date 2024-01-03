use std::collections::HashMap;

use custom_utils::get_image_from_path;
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonSettings,
    OutputMessage as IconButtonMessage,
};
use gtk::{
    gdk, gio, glib,
    prelude::{BoxExt, GtkWindowExt},
};
use relm4::{
    async_trait::async_trait,
    component::{AsyncComponent, AsyncComponentParts},
    factory::FactoryVecDeque,
    gtk::prelude::WidgetExt,
    AsyncComponentSender, Component, ComponentController, Controller,
};
use relm4::{gtk, RelmApp, RelmWidgetExt};

mod settings;
mod theme;
mod widgets;
use tracing::{debug, error, info};
use wayland_protocols_async::zwlr_foreign_toplevel_management_v1::handler::ToplevelKey;
use widgets::running_app::{AppDetails, Message as RunningAppMessage, RunningApp};
pub mod errors;
pub mod services;
use crate::settings::AppSwitcherSettings;
use crate::theme::AppSwitcherTheme;
use crate::{
    services::app_manager::{AppManagerMessage, AppManagerService},
    widgets::running_app::AppInstance,
};
use tokio::sync::{mpsc, oneshot};

/// # App Switcher state
///
/// This struct is the state definition of the entire application
struct AppSwitcher {
    settings: AppSwitcherSettings,
    custom_theme: AppSwitcherTheme,
    running_apps: FactoryVecDeque<RunningApp>,
    app_manager_sender: mpsc::Sender<AppManagerMessage>,
    cpu_usage: String,
    memory_usage: String,
}

/// ## Message
///
/// These are the events (or messages) that update state.
/// Each of them are handled in the ``impl Application()::update()``
#[derive(Debug, Clone)]
pub enum Message {
    AppInstanceClicked(ToplevelKey),
    AppInstanceCloseClicked(ToplevelKey),
    HomePressed,
    AppsUpdated { apps: Vec<AppDetails> },
    CpuUsageChanged(String),
    MemoryUsageChanged(String),
    CloseAllApps,
}

// #[cfg(not(feature = "layer-shell"))]
fn init_window(settings: AppSwitcherSettings) -> gtk::Window {
    let window_settings = settings.window;
    let window = gtk::Window::builder()
        .title(settings.title)
        .default_width(window_settings.size.0)
        .default_height(window_settings.size.1)
        .css_classes(["window"])
        .build();
    window
}

// #[cfg(feature = "layer-shell")]
// fn init_window(settings: AppSwitcherSettings) -> gtk::Window {
//     let window_settings = settings.window;
//     let window = gtk::Window::builder()
//         .title(settings.title)
//         .default_width(window_settings.size.0)
//         .default_height(window_settings.size.1)
//         .decorated(false)
//         .css_classes(["window"])
//         .build();

//     gtk4_layer_shell::init_for_window(&window);

//     // Display above normal windows
//     gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);

//     // The margins are the gaps around the window's edges
//     // Margins and anchors can be set like this...
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);
//     gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 24);

//     // ... or like this
//     // Anchors are if the window is pinned to each edge of the output
//     let anchors = [
//         (gtk4_layer_shell::Edge::Left, true),
//         (gtk4_layer_shell::Edge::Right, true),
//         (gtk4_layer_shell::Edge::Top, false),
//         (gtk4_layer_shell::Edge::Bottom, true),
//     ];

//     for (anchor, state) in anchors {
//         gtk4_layer_shell::set_anchor(&window, anchor, state);
//     }

//     window
// }

struct AppWidgets {
    cpu_value_label: gtk::Label,
    memory_value_label: gtk::Label,
    home_btn: Controller<IconButton>,
    close_all_btn: Controller<IconButton>,
}

#[async_trait(?Send)]
impl AsyncComponent for AppSwitcher {
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

    type CommandOutput = Message;

    fn init_root() -> Self::Root {
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => AppSwitcherSettings::default(),
        };

        info!(
            task = "initalize_settings",
            "settings initialized for App Switcher: {:?}", settings
        );

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => AppSwitcherTheme::default(),
        };

        info!(
            task = "initalize_theme",
            "theme initialized for App Switcher: {:?}", custom_theme
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
        let settings = match settings::read_settings_yml() {
            Ok(settings) => settings,
            Err(_) => AppSwitcherSettings::default(),
        };

        let css = settings.css.clone();
        relm4::set_global_css_from_file(css.default);

        let custom_theme = match theme::read_theme_yml() {
            Ok(theme) => theme,
            Err(_) => AppSwitcherTheme::default(),
        };

        let modules = settings.modules.clone();

        let modules_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .hexpand(true)
            .build();

        let cpu_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["cpu-box"])
            .hexpand(true)
            .build();
        let cpu_image = get_image_from_path(modules.cpu.icon.clone(), &["cpu-icon"]);
        cpu_box.append(&cpu_image);

        match &modules.cpu.title {
            Some(label) => {
                let cpu_label = gtk::Label::builder()
                    .label(label)
                    .css_classes(["cpu-label"])
                    .build();
                cpu_box.append(&cpu_label);
            }
            None => (),
        };
        let cpu_value_label = gtk::Label::builder()
            .label("10%")
            .css_classes(["cpu-value"])
            .build();

        cpu_box.append(&cpu_value_label);

        let memory_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["memory-box"])
            .hexpand(true)
            .build();

        let memory_image = get_image_from_path(modules.memory.icon.clone(), &["memory-icon"]);
        memory_box.append(&memory_image);

        match &modules.memory.title {
            Some(label) => {
                let memory_label = gtk::Label::builder()
                    .label(label)
                    .css_classes(["memory-label"])
                    .build();
                memory_box.append(&memory_label);
            }
            None => (),
        };

        let memory_value_label = gtk::Label::builder()
            .label("1.2/4GB")
            .css_classes(["memory-value"])
            .build();

        memory_box.append(&memory_value_label);
        modules_box.append(&cpu_box);
        modules_box.append(&memory_box);

        let running_apps: FactoryVecDeque<RunningApp> = FactoryVecDeque::builder()
            .launch(
                gtk::Box::builder()
                    .valign(gtk::Align::Start)
                    .halign(gtk::Align::Start)
                    .spacing(14)
                    .css_classes(["apps-list"])
                    .build(),
            )
            .forward(
                sender.input_sender(),
                glib::clone!(@strong modules => move|msg| {

                    info!("app widget forwarded message is {:?}", msg);
                    return match msg {
                    RunningAppMessage::AppInstanceClicked(instance) => {
                        Message::AppInstanceClicked(instance)
                    },
                    RunningAppMessage::AppInstanceCloseClicked(instance) => {
                        Message::AppInstanceCloseClicked(instance)
                    },
                }}),
            );

        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["footer"])
            .valign(gtk::Align::End)
            .vexpand(true)
            .build();

        let home_btn = IconButton::builder()
            .launch(IconButtonSettings {
                icon: modules.home.icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    IconButtonMessage::Clicked => Message::HomePressed,
                }
            });

        let close_all_btn = IconButton::builder()
            .launch(IconButtonSettings {
                icon: modules.close_all.icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| {
                info!("msg is {:?}", msg);
                match msg {
                    IconButtonMessage::Clicked => Message::CloseAllApps,
                }
            });
        let close_all_btn_widget = close_all_btn.widget();
        close_all_btn_widget.set_hexpand(true);
        close_all_btn_widget.set_halign(gtk::Align::End);

        footer.append(home_btn.widget());
        footer.append(close_all_btn_widget);

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["container"])
            .build();

        container.append(&modules_box);

        let scrollable = gtk::ScrolledWindow::builder()
            .vscrollbar_policy(gtk::PolicyType::Never) // Disable vertical scrolling
            .min_content_width(360)
            .min_content_height(360)
            .css_classes(["scrollable"])
            .child(running_apps.widget())
            .build();

        container.append(&scrollable);

        container.append(&footer);

        window.set_child(Some(&container));

        let sender: relm4::Sender<Message> = sender.input_sender().clone();
        let (app_manager_sender) = init_services(sender).await;

        let model = AppSwitcher {
            settings,
            custom_theme,
            running_apps,
            app_manager_sender,
            cpu_usage: String::from("15%"),
            memory_usage: String::from("1.2/4GB"),
        };

        let widgets = AppWidgets {
            cpu_value_label,
            memory_value_label,
            home_btn,
            close_all_btn,
        };

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        info!("Update message is {:?}", message);
        match message {
            Message::AppsUpdated { apps } => {
                info!("apps updated are {:?}", apps);
                self.running_apps.guard().clear();
                apps.into_iter().for_each(|app| {
                    self.running_apps.guard().push_back(RunningApp {
                        app_details: app,
                        close_icon: self.settings.modules.close.icon.clone(),
                    });
                });
            }
            Message::AppInstanceClicked(instance) => {
                let (tx, rx) = oneshot::channel();
                debug!("sending message to wayland to activate app instance");
                let _ = self
                    .app_manager_sender
                    .send(AppManagerMessage::ActivateAppInstance {
                        instance,
                        reply_to: tx,
                    })
                    .await;
                info!("message sent to wayland to activate app instance");
                let res = rx.await.expect("no reply from service");

                match res {
                    Ok(r) => {
                        info!("activate app instance res from wayland {:?}", r);
                    }
                    Err(e) => {
                        error!("activate app instance error from wayland {}", e);
                    }
                }
            }
            Message::AppInstanceCloseClicked(instance) => {
                let (tx, rx) = oneshot::channel();
                debug!("sending message to wayland to close app instance");
                let _ = self
                    .app_manager_sender
                    .send(AppManagerMessage::CloseAppInstance {
                        instance,
                        reply_to: tx,
                    })
                    .await;
                info!("message sent to wayland to close app instance");
                let res = rx.await.expect("no reply from service");

                match res {
                    Ok(r) => {
                        info!("close app instance res from wayland {:?}", r);
                    }
                    Err(e) => {
                        error!("close app instance error from wayland {}", e);
                    }
                }
            }
            Message::HomePressed => {}
            Message::CloseAllApps => {
                let (tx, rx) = oneshot::channel();
                debug!("sending message to wayland to close all apps instance");
                let _ = self
                    .app_manager_sender
                    .send(AppManagerMessage::CloseAllApps { reply_to: tx })
                    .await;
                info!("message sent to wayland to close all apps instance");
                let res = rx.await.expect("no reply from service");

                match res {
                    Ok(r) => {
                        info!("close all apps instance res from wayland {:?}", r);
                    }
                    Err(e) => {
                        error!("close all apps instance error from wayland {}", e);
                    }
                }
            }
            Message::CpuUsageChanged(usage) => {
                self.cpu_usage = usage;
            }
            Message::MemoryUsageChanged(usage) => {
                self.memory_usage = usage;
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: AsyncComponentSender<Self>) {
        widgets.cpu_value_label.set_label(&self.cpu_usage);
        widgets.memory_value_label.set_label(&self.memory_usage);
    }
}

fn main() {
    // Enables logger
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter("mecha_app_switcher=trace")
        .with_thread_names(true)
        .init();

    let app = RelmApp::new("app.switcher").with_args(vec![]);
    app.run_async::<AppSwitcher>(());
}

async fn init_services(
    app_switcher_sender: relm4::Sender<Message>,
) -> (mpsc::Sender<AppManagerMessage>) {
    let (app_manager_t, app_manager_tx) = init_app_manager(app_switcher_sender).await;

    (app_manager_tx)
}

async fn init_app_manager(
    app_switcher_sender: relm4::Sender<Message>,
) -> (relm4::JoinHandle<()>, mpsc::Sender<AppManagerMessage>) {
    let (tx, rx) = mpsc::channel(32);

    let t =
        tokio::spawn(async move { AppManagerService::new().run(rx, app_switcher_sender).await });

    (t, tx)
}
