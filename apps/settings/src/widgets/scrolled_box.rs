use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct ScrolledBox{
    content: gtk::Box
}


#[relm4::component(pub)]
impl SimpleComponent for ScrolledBox {
    type Init = gtk::Box;
    type Input = ();
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            set_min_content_height: 360,
            set_min_content_width: 360,
            set_child: Some(&model.content)
        }
    }

    fn init(
        content: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ScrolledBox { content };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
