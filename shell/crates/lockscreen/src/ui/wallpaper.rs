use gpui::*;
use std::path::PathBuf;

// Wallpaper image path - use PNG for complex images (SVGs with masks/embedded images aren't fully supported)
const DEFAULT_WALLPAPER_PATH: &str = "icons/lockscreen/wallpaper.png";
/// Creates a wallpaper element that fills the given dimensions
pub fn wallpaper(width: Pixels, height: Pixels, path: Option<PathBuf>) -> impl IntoElement {
    match path {
        Some(path) => img(path),
        None => img(DEFAULT_WALLPAPER_PATH),
    }
    .w(width)
    .h(height)
    .object_fit(ObjectFit::Cover)
}
