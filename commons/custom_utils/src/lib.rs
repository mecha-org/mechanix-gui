use gtk::{gdk, gio, glib};
use relm4::gtk::{self, ffi::gtk_image_new_from_paintable};

pub fn get_image_from_path(path: Option<String>, css_classes: &[&str]) -> gtk::Image {
    let image = gtk::Image::builder().css_classes(css_classes).build();

    match path {
        Some(p) => {
            let assets_base_path =
                std::env::var("MECHA_STATUS_BAR_ASSETS_PATH").unwrap_or(String::from(""));
            let new_path = assets_base_path + &p;
            let image_file = gio::File::for_path(new_path);
            let image_asset_paintable = gdk::Texture::from_file(&image_file).unwrap();
            image.set_paintable(Option::from(&image_asset_paintable));
        }
        None => (),
    }
    image
}
