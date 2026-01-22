use gpui::*;
use std::path::PathBuf;

/// Creates a wallpaper element that fills the given dimensions
pub fn wallpaper(width: Pixels, height: Pixels, path: PathBuf) -> impl IntoElement {
    img(path).w(width).h(height).object_fit(ObjectFit::Cover)
}
