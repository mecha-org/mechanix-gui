use gpui::*;
use theme::prelude::{AlphaExt, ThemeColors};

// Wallpaper image path - use PNG for complex images (SVGs with masks/embedded images aren't fully supported)
const WALLPAPER_PATH: &str = "icons/lockscreen/wallpaper.png";

/// Creates a wallpaper element that fills the given dimensions
pub fn wallpaper(width: Pixels, height: Pixels, colors: &ThemeColors) -> impl IntoElement {
    // Currently alpha = 0.0 
    let accent_overlay_color = colors.accent_200.with_alpha(0.0);

    div()
        .w(width)
        .h(height)
        .child(
            img(WALLPAPER_PATH)
                .w(width)
                .h(height)
                .object_fit(ObjectFit::Cover),
        )
        .child(div().absolute().inset_0().bg(accent_overlay_color))
}
