use gpui::*;
use theme::prelude::{AlphaExt, ThemeColors};

/// Creates a wallpaper element that fills the given dimensions
pub fn wallpaper(
    width: Pixels,
    height: Pixels,
    colors: &ThemeColors,
    wallpaper_path: &str,
) -> impl IntoElement {
    // Currently alpha = 0.0 
    let accent_overlay_color = colors.accent_200.with_alpha(0.0);

    div()
        .w(width)
        .h(height)
        .child(
            img(wallpaper_path)
                .w(width)
                .h(height)
                .object_fit(ObjectFit::Cover),
        )
        .child(div().absolute().inset_0().bg(accent_overlay_color))
}
