use gtk::prelude::*;
use crate::settings::{LayoutSettings, Modules, WidgetConfigs};
use relm4::{
    gtk::{self},
    Component, ComponentParts, ComponentSender, SimpleComponent, Controller, ComponentController,
};
use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    InputMessage as IconButtonInputMessage, OutputMessage as IconButtonOutputMessage,
};
use tracing::info;

//Init Settings
pub struct Settings {
    pub modules: Modules,
    pub layout: LayoutSettings,
    pub widget_configs: WidgetConfigs,
}

//Model
pub struct SetTimePage {
    settings: Settings,
    hr_idx: usize,
    min_idx: usize,
    am_pm_idx: usize,
}

//Widgets
pub struct SetTimePageWidgets {
    back_button: Controller<IconButton>,
    submit_button: Controller<IconButton>,
}

//Messages
#[derive(Debug)]
pub enum Message {
    BackPressed,
    HomeIconPressed,
    HoursInputChange(usize),
    MinutesInputChange(usize),
    SelectionChanged(usize),
    SubmitPressed
}

pub struct SettingItem {
    name: String,
}

impl SimpleComponent for SetTimePage {
    type Init = Settings;
    type Input = Message;
    type Output = Message;
    type Root = gtk::Box;
    type Widgets = SetTimePageWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["page-container"])
            .build()
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let modules = init.modules.clone();
        let layout = init.layout.clone();
        let widget_configs = init.widget_configs.clone();

        let header_title = gtk::Label::builder()
            .label("Set Time")
            .css_classes(["header-title"])
            .build();

        let header = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["header"])
            .build();

        header.append(&header_title);

        let time_input_label = gtk::Label::builder()
            .label("Time set according to the standard time (UST)")
            .halign(gtk::Align::Start)
            .css_classes(["text-14-label"])
            .build();

        let input_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes(["set-time-box"])
            .build();

        root.append(&header);
        root.append(&time_input_label);

        let hr_string_array: Vec<String> = (0..13).map(|x| format!("{:02}", x)).collect();
        let hr_str_array: Vec<&str> = hr_string_array.iter().map(|s| s.as_str()).collect();
        let hr_model = gtk::StringList::new(&hr_str_array);
        let hour_dropdown = gtk::DropDown::new(Some(hr_model), gtk::Expression::NONE);
        hour_dropdown.add_css_class("time-dropdown-width");

        hour_dropdown.connect_selected_notify(|dropdown| {
            let  selected = dropdown.selected();
            info!("hr_idx : {:?} ", selected);
        });


        let label: gtk::Label = gtk::Label::builder()
            .label(":")
            .css_classes(["margin-x-10"])
            .build();

        let min_string_array: Vec<String> = (0..60).map(|x| format!("{:02}", x)).collect();
        let min_str_array: Vec<&str> = min_string_array.iter().map(|s| s.as_str()).collect();
        let min_model = gtk::StringList::new(&min_str_array);
        let minutes_dropdown = gtk::DropDown::new(Some(min_model), gtk::Expression::NONE);
        minutes_dropdown.add_css_class("time-dropdown-width");

        minutes_dropdown.connect_selected_notify(|dropdown| {
            let  selected = dropdown.selected();
            info!("min_idx : {:?} ", selected);
        });

        let am_pm_list = ["AM", "PM"];
        let am_pm_model = gtk::StringList::new(&am_pm_list);
        let am_pm_dropdown = gtk::DropDown::new(Some(am_pm_model), gtk::Expression::NONE);
        let am_pm_style = am_pm_dropdown.style_context();
        am_pm_style.add_class("time-dropdown-width");
        am_pm_style.add_class("margin-x-10");

        am_pm_dropdown.connect_selected_notify(|dropdown| {
            let  selected = dropdown.selected();
            info!("am_pm_idx : {:?} ", selected);
        });

        input_box.append(&hour_dropdown);
        input_box.append(&label);
        input_box.append(&minutes_dropdown);
        input_box.append(&am_pm_dropdown);

        root.append(&input_box);

        
        let footer = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["footer"])
        .vexpand(true)
        .hexpand(true)
        .valign(gtk::Align::End)
        .build();

        let back_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: widget_configs.footer.back_icon.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::BackPressed,
            });

        footer.append(back_button.widget());

        let submit_button = IconButton::builder()
            .launch(IconButtonStetings {
                icon: modules.submit.icon.default.to_owned(),
                toggle_icon: None,
                css: IconButtonCss::default(),
            })
            .forward(sender.input_sender(), |msg| match msg {
                IconButtonOutputMessage::Clicked => Message::SubmitPressed,
            });
        let submit_button_widget = submit_button.widget();
        submit_button_widget.set_hexpand(true);
        submit_button_widget.set_halign(gtk::Align::End);

        footer.append(submit_button_widget);
        root.append(&footer);

        let model = SetTimePage {
            settings: init,
            hr_idx: 0,
            min_idx: 0,
            am_pm_idx: 0,
        };

        let widgets = SetTimePageWidgets {
            back_button,
            submit_button,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        info!("Update message is {:?}", message);
        match message {
            Message::BackPressed => {
                let _ = sender.output(Message::BackPressed);
            }
            Message::HomeIconPressed => {
                sender.output(Message::HomeIconPressed);
            }
            Message::HoursInputChange(idx) => {
                self.hr_idx = idx;
            }
            Message::MinutesInputChange(idx) => {
                self.min_idx = idx;
            }
            Message::SelectionChanged(idx) => {
                // println!("selection changed is {:?} ", idx);
                self.am_pm_idx = idx;
            }
            Message::SubmitPressed => {}
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {}
}
