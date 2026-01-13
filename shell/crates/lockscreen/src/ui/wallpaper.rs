use gpui::*;

// Wallpaper image path - use PNG for complex images (SVGs with masks/embedded images aren't fully supported)
const WALLPAPER_PATH: &str = "icons/lockscreen/wallpaper.png";

/// Creates a wallpaper element that fills the given dimensions
pub fn wallpaper(width: Pixels, height: Pixels) -> impl IntoElement {
    img(WALLPAPER_PATH)
        .w(width)
        .h(height)
        .object_fit(ObjectFit::Cover)
}
