use gtk::{gdk, gio, glib};
use relm4::gtk::{self, ffi::gtk_image_new_from_paintable};
use tracing::info;

pub fn get_image_from_path(path: Option<String>, css_classes: &[&str]) -> gtk::Image {
    let image = gtk::Image::builder().css_classes(css_classes).build();

    match path {
        Some(p) => {
            let image_file = gio::File::for_path(p);
            match gdk::Texture::from_file(&image_file) {
                Ok(image_asset_paintable) => {
                    image.set_paintable(Option::from(&image_asset_paintable));
                }
                Err(_) => (),
            }
        }
        None => (),
    }
    image
}
