use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct Header{
    title: String
}


#[relm4::component(pub)]
impl SimpleComponent for Header {
    type Init = String;
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            add_css_class: "header",
            
            gtk::Label {
                #[watch]
                set_label: &model.title,
                add_css_class: "header-title",
            }
        }
    }

    fn init(
        title: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Header { title };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
