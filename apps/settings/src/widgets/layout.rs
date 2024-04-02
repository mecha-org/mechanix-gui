use gtk::prelude::*;
use relm4::prelude::*;
use std::convert::identity;

use crate::{settings::{FooterWidgetConfigs, WidgetConfigs}, widgets::footer::{Footer, FooterInit, FooterMessage}};

use super::{header::Header, scrolled_box::ScrolledBox};

#[derive(Debug, Clone)]
pub enum  LayoutMessage{
    BackPressed
}
pub struct LayoutInit {
    pub title: String,
    pub content: gtk::Box,
    pub footer_config: FooterWidgetConfigs,
}

pub(crate) struct Layout {
    footer: Controller<Footer>,
    // scrolled_box: Controller<ScrolledBox>,
}

#[relm4::component(pub)]
impl SimpleComponent for Layout {
    type Init = LayoutInit;
    type Input = LayoutMessage;
    type Output = LayoutMessage;

    view! {

        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            header.widget(){},

            scrolled_box.widget(){},

            #[local_ref]
            footer_widget -> gtk::Box{}

        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let footer_config = init.footer_config.clone();
        let header = Header::builder()
            .launch(init.title.to_owned());
        let scrolled_box = ScrolledBox::builder()
            .launch(init.content);

        let footer = Footer::builder()
            .launch(FooterInit{
                footer_config
            })
            .forward(sender.input_sender(),  |msg| {
                println!("Layout callback {:?}", msg);
                match msg {
                    FooterMessage::BackPressed => LayoutMessage::BackPressed,
                }});

        let model = Layout {
            footer
        };

        let footer_widget= model.footer.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            LayoutMessage::BackPressed => {
                println!("Footer BackPressed");
                let _ = sender.output(LayoutMessage::BackPressed);
            }
        }
    }
}
