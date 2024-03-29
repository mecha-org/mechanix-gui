use custom_widgets::icon_button::{
    IconButton, IconButtonCss, InitSettings as IconButtonStetings,
    OutputMessage as IconButtonOutputMessage,
};
use gtk::prelude::*;
use relm4::prelude::*;

use crate::settings::FooterWidgetConfigs;

pub(crate) struct Footer {
}
pub(crate) struct FooterInit {
    pub footer_config: FooterWidgetConfigs,
}

pub struct FooterWidgets {
    back_button: Controller<IconButton>
}
#[derive(Debug, Clone)]
pub enum FooterMessage {
    BackPressed,
}

impl SimpleComponent for Footer {
    type Init = FooterInit;
    type Input = FooterMessage;
    type Output = FooterMessage;    
    type Root = gtk::Box;
    type Widgets = FooterWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["footer"])
        .vexpand(true)
        .hexpand(true)
        .valign(gtk::Align::End)
        .build()
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let back_button = IconButton::builder()
        .launch(IconButtonStetings {
            icon: init.footer_config.back_icon,
            toggle_icon: None,
            css: IconButtonCss::default(),
        })
        .forward(sender.input_sender(), |msg| {
            println!("Footer callback {:?}", msg);
            match msg {
            IconButtonOutputMessage::Clicked => FooterMessage::BackPressed,
        }});

        root.append(back_button.widget());

        let model = Footer {};

        let widgets =  FooterWidgets {
            back_button
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            FooterMessage::BackPressed => {
                println!("Footer BackPressed");
                let _ = sender.output(FooterMessage::BackPressed);
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
        
    }
    
}
